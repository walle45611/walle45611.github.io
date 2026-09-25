use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use regex::{Captures, Regex};
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Clone)]
struct Post {
    source: PathBuf,
    name: String,
    title: String,
    slug: String,
    date: String,
    topic: String,
    body: String,
}

fn files_under(root: &Path, extension: &str) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    if !root.exists() {
        return Ok(files);
    }
    for entry in WalkDir::new(root) {
        let entry = entry?;
        if entry.file_type().is_file()
            && entry.path().extension().and_then(|value| value.to_str()) == Some(extension)
        {
            files.push(entry.path().to_path_buf());
        }
    }
    files.sort();
    Ok(files)
}

fn fail(message: impl Into<String>) -> Box<dyn Error> {
    message.into().into()
}

fn links(content: &str) -> Vec<String> {
    let re = Regex::new(r"\[\[([^\]]+)\]\]").expect("link regex");
    re.captures_iter(content)
        .map(|capture| {
            capture[1]
                .split(['|', '#'])
                .next()
                .unwrap_or_default()
                .to_string()
        })
        .collect()
}

fn parse_post(source: &Path, note: &str) -> Result<Option<Post>> {
    let front = Regex::new(r"(?s)\A---\r?\n(.*?)\r?\n---\r?\n")?;
    let Some(capture) = front.captures(note) else {
        return Ok(None);
    };
    let properties: HashMap<String, serde_yaml::Value> = serde_yaml::from_str(&capture[1])?;
    if properties.get("blog").and_then(serde_yaml::Value::as_bool) != Some(true) {
        return Ok(None);
    }
    let url = properties
        .get("blog_url")
        .and_then(serde_yaml::Value::as_str)
        .unwrap_or_default();
    let slug_regex = Regex::new(r"^https://blog\.walle4561\.com/articles/posts/([^/\s]+)/$")?;
    let slug = slug_regex
        .captures(url)
        .ok_or_else(|| fail(format!("Invalid blog URL in {}", source.display())))?[1]
        .to_string();
    let date = properties
        .get("blog_date")
        .and_then(serde_yaml::Value::as_str)
        .unwrap_or_default();
    if !Regex::new(r"^\d{4}-\d{2}-\d{2}$")?.is_match(date) {
        return Err(fail(format!("Invalid blog date in {}", source.display())));
    }
    let title = properties
        .get("blog_title")
        .and_then(serde_yaml::Value::as_str)
        .ok_or_else(|| fail(format!("Invalid blog title in {}", source.display())))?
        .to_string();
    if title.trim().is_empty() {
        return Err(fail(format!("Empty blog title in {}", source.display())));
    }
    let topic = properties
        .get("blog_topic")
        .and_then(serde_yaml::Value::as_str)
        .unwrap_or_default();
    if !topic.is_empty() && !["data-structures", "algorithms", "problem-solving"].contains(&topic) {
        return Err(fail(format!("Invalid blog topic in {}", source.display())));
    }
    let body = note[capture.get(0).expect("front matter match").end()..].trim();
    if body.is_empty() {
        return Err(fail(format!("Empty blog body in {}", source.display())));
    }
    Ok(Some(Post {
        source: source.to_path_buf(),
        name: source
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned(),
        title,
        slug,
        date: date.to_string(),
        topic: topic.to_string(),
        body: body.to_string(),
    }))
}

fn dimensions(bytes: &[u8]) -> Result<(usize, usize)> {
    let size = imagesize::blob_size(bytes)?;
    Ok((size.width, size.height))
}

fn html_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

fn fence(line: &str) -> Option<(char, usize)> {
    let trimmed = line.trim_start_matches([' ', '\t']);
    let symbol = trimmed.chars().next()?;
    if symbol != '`' && symbol != '~' {
        return None;
    }
    let width = trimmed
        .chars()
        .take_while(|character| *character == symbol)
        .count();
    (width >= 3).then_some((symbol, width))
}

fn convert_format(body: &str) -> Result<String> {
    let heading = Regex::new(r"^#{1,5}[ \t]+")?;
    let callout = Regex::new(r"(?i)^([ \t]*>[ \t]*)\[!(\w+)\][+-]?[ \t]*(.*)$")?;
    let highlight = Regex::new(r"==(.+?)==")?;
    let mut out = Vec::<String>::new();
    let mut active_fence: Option<(char, usize)> = None;
    for line in body.lines() {
        let marker = fence(line);
        if active_fence.is_none() && marker.is_some() {
            active_fence = marker;
            out.push(line.to_string());
            continue;
        }
        if let Some((symbol, width)) = active_fence {
            out.push(line.to_string());
            if let Some((closing_symbol, closing_width)) = marker {
                if symbol == closing_symbol && closing_width >= width {
                    active_fence = None;
                }
            }
            continue;
        }
        if line.contains("<img ") {
            if out
                .last()
                .is_some_and(|previous| !previous.trim().is_empty())
            {
                out.push(String::new());
            }
            out.push(line.to_string());
            out.push(String::new());
            continue;
        }
        let mut converted = if heading.is_match(line) {
            format!("#{line}")
        } else {
            line.to_string()
        };
        if let Some(capture) = callout.captures(&converted) {
            let label = if capture[3].is_empty() {
                match &capture[2].to_ascii_lowercase()[..] {
                    "note" => "筆記",
                    "info" => "資訊",
                    "tip" => "提示",
                    "warning" => "注意",
                    "important" => "重點",
                    "example" => "範例",
                    "question" => "問題",
                    _ => &capture[2],
                }
            } else {
                &capture[3]
            };
            converted = format!("{}**{}**", &capture[1], label);
        }
        let original = converted.clone();
        converted = highlight
            .replace_all(&original, |capture: &Captures| {
                let match_range = capture.get(0).expect("highlight match");
                let before = &original[..match_range.start()];
                let through = &original[..match_range.end()];
                let next = original[match_range.end()..].chars().next();
                if next
                    .is_some_and(|character| character.is_ascii_alphanumeric() || character == '_')
                    || before.bytes().filter(|byte| *byte == b'`').count() % 2 == 1
                    || through.bytes().filter(|byte| *byte == b'`').count() % 2 == 1
                    || before.bytes().filter(|byte| *byte == b'$').count() % 2 == 1
                    || through.bytes().filter(|byte| *byte == b'$').count() % 2 == 1
                {
                    capture[0].to_string()
                } else {
                    format!("<mark>{}</mark>", &capture[1])
                }
            })
            .into_owned();
        if converted.starts_with("![")
            && out
                .last()
                .is_some_and(|previous| !previous.trim().is_empty())
        {
            out.push(String::new());
        }
        let image_line = converted.starts_with("![");
        out.push(converted);
        if image_line {
            out.push(String::new());
        }
    }
    Ok(out.join("\n").trim().to_string())
}

fn normalize_math(body: &str) -> Result<String> {
    let mut output = Vec::<String>::new();
    let mut in_math = false;
    let mut active_fence: Option<char> = None;
    for line in body.lines() {
        if let Some((symbol, _)) = fence(line) {
            active_fence = if active_fence == Some(symbol) {
                None
            } else {
                Some(symbol)
            };
            output.push(line.to_string());
            continue;
        }
        if active_fence.is_some() {
            output.push(line.to_string());
            continue;
        }
        let Some(marker) = line.find("$$") else {
            output.push(line.to_string());
            continue;
        };
        if line[..marker].bytes().filter(|byte| *byte == b'`').count() % 2 == 1 {
            output.push(line.to_string());
            continue;
        }
        if !in_math && line[marker + 2..].contains("$$") {
            output.push(line.to_string());
            continue;
        }
        let before = &line[..marker];
        let after = &line[marker + 2..];
        if !before.trim().is_empty() {
            output.push(before.trim_end().to_string());
        }
        output.push("$$".to_string());
        if !after.trim().is_empty() {
            output.push(if in_math {
                after.trim_end().to_string()
            } else {
                after.trim_start().to_string()
            });
        }
        in_math = !in_math;
    }
    let compact = Regex::new(r"(?ms)^\$\$[ \t]*\n(.*?)^\$\$[ \t]*$")?;
    Ok(compact
        .replace_all(&output.join("\n"), |capture: &Captures| {
            let lines = capture[1]
                .lines()
                .filter(|line| !line.trim().is_empty())
                .map(str::trim_end)
                .collect::<Vec<_>>();
            format!("$$\n{}\n$$", lines.join("\n"))
        })
        .into_owned())
}

pub fn export(vault: &Path, build_dir: &Path) -> Result<()> {
    let overview = fs::read_to_string(vault.join("00_Dashboard/資料結構和演算法 Overview.md"))?;
    let names = links(&overview);
    let all_names: HashSet<_> = names.iter().cloned().collect();
    if names.len() != all_names.len() {
        return Err(fail("Dashboard has duplicate note links"));
    }
    let algorithms = overview
        .split_once("# 演算法")
        .ok_or_else(|| fail("Dashboard lacks algorithm section"))?;
    let algorithm_names: HashSet<_> = links(algorithms.1).into_iter().collect();
    let mut posts = Vec::<Post>::new();
    let mut slugs = HashSet::new();
    for source in files_under(&vault.join("Note"), "md")? {
        let note = fs::read_to_string(&source)?;
        if let Some(post) = parse_post(&source, &note)? {
            if !slugs.insert(post.slug.clone()) {
                return Err(fail(format!("Duplicate blog slug: {}", post.slug)));
            }
            posts.push(post);
        }
    }
    let slugs_by_name: HashMap<_, _> = posts
        .iter()
        .map(|post| (post.name.clone(), post.slug.clone()))
        .collect();
    let posts_dir = build_dir.join("vault-posts");
    let assets_dir = build_dir.join("vault-assets");
    fs::create_dir_all(&posts_dir)?;
    fs::create_dir_all(&assets_dir)?;
    let embed = Regex::new(r"!?\[\[(.*?)\]\]")?;
    let tikz = Regex::new(r"(?ms)^```tikz[ \t]*\n(.*?)^```[ \t]*$")?;
    let mut expected_posts = HashSet::new();
    let mut expected_assets = HashSet::new();
    for post in &posts {
        let mut error = None;
        let body = embed.replace_all(&post.body, |capture: &Captures| {
            let reference = capture[1].split(['|', '#']).next().unwrap_or_default();
            if !capture[0].starts_with('!') {
                let linked = reference.strip_suffix(".md").unwrap_or(reference);
                return match slugs_by_name.get(linked) {
                    Some(slug) => format!("[{linked}](/articles/posts/{slug}/)"),
                    None => { error = Some(format!("Unmapped note link in {}: {reference}", post.name)); String::new() }
                };
            }
            let asset = vault.join(reference);
            let image = match fs::read(&asset) {
                Ok(image) => image,
                Err(_) => { error = Some(format!("Missing asset: {}", asset.display())); return String::new(); }
            };
            let digest = format!("{:x}", Sha256::digest(&image));
            let suffix = asset.extension().and_then(|value| value.to_str()).unwrap_or_default().to_ascii_lowercase();
            let name = format!("{}.{}", &digest[..20], suffix);
            let target = assets_dir.join(&name);
            if !target.exists() && fs::write(&target, &image).is_err() {
                error = Some(format!("Cannot write asset: {}", target.display())); return String::new();
            }
            expected_assets.insert(name.clone());
            let (width, height) = match dimensions(&image) {
                Ok(size) => size,
                Err(message) => { error = Some(message.to_string()); return String::new(); }
            };
            let alt = html_escape(asset.file_stem().unwrap_or_default().to_string_lossy().as_ref());
            format!("<img src=\"/vault-assets/{name}\" alt=\"{alt}\" width=\"{width}\" height=\"{height}\" decoding=\"async\">")
        }).into_owned();
        if let Some(message) = error {
            return Err(fail(message));
        }
        let body = normalize_math(&convert_format(&body)?)?;
        if tikz.is_match(&body) {
            return Err(fail(format!("TikZ remains in {}", post.source.display())));
        }
        let topic = if !post.topic.is_empty() {
            Some(post.topic.as_str())
        } else if algorithm_names.contains(&post.name) {
            Some("algorithms")
        } else if all_names.contains(&post.name) {
            Some("data-structures")
        } else {
            None
        };
        let description = format!("{}的重點整理。", post.title);
        let relative = post.source.strip_prefix(vault)?.to_string_lossy();
        let mut front = format!(
            "---\ntitle: {}\nslug: {}\n",
            serde_json::to_string(&post.title)?,
            post.slug
        );
        if let Some(topic) = topic {
            front.push_str(&format!("topic_section: {topic}\n"));
        }
        front.push_str(&format!(
            "description: {}\ndate: {}\nblog: true\nvault_source: {}\n---\n\n",
            serde_json::to_string(&description)?,
            post.date,
            serde_json::to_string(&relative.as_ref())?
        ));
        let target = posts_dir.join(format!("{}.md", post.slug));
        fs::write(&target, format!("{front}{body}\n"))?;
        expected_posts.insert(
            target
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),
        );
    }
    for entry in fs::read_dir(&posts_dir)? {
        let path = entry?.path();
        if path.extension().and_then(|value| value.to_str()) == Some("md")
            && !expected_posts.contains(
                &path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned(),
            )
        {
            fs::remove_file(path)?;
        }
    }
    for entry in fs::read_dir(&assets_dir)? {
        let path = entry?.path();
        if path.is_file()
            && !expected_assets.contains(
                &path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned(),
            )
        {
            fs::remove_file(path)?;
        }
    }
    println!(
        "Exported {} posts and {} assets",
        expected_posts.len(),
        expected_assets.len()
    );
    Ok(())
}

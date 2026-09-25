use std::collections::{BTreeMap, HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use chrono::Local;
use regex::Regex;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn invalid(message: impl Into<String>) -> Box<dyn Error> {
    message.into().into()
}

fn root() -> Result<PathBuf> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    Ok(manifest
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .and_then(Path::parent)
        .ok_or_else(|| invalid("Cannot locate repository root"))?
        .to_path_buf())
}

fn relative(path: &Path, root: &Path) -> Result<String> {
    Ok(path
        .strip_prefix(root)?
        .to_string_lossy()
        .replace('\\', "/"))
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn markdown_under(folder: &Path) -> Result<Vec<PathBuf>> {
    if !folder.exists() {
        return Ok(Vec::new());
    }
    let mut files = Vec::new();
    for item in WalkDir::new(folder) {
        let item = item?;
        if item.file_type().is_file()
            && item.path().extension().and_then(|value| value.to_str()) == Some("md")
        {
            files.push(item.into_path());
        }
    }
    files.sort();
    Ok(files)
}

pub fn run(command: &str, args: &[String]) -> Result<()> {
    let project = if let Some(index) = args.iter().position(|item| item == "--project-dir") {
        PathBuf::from(args.get(index + 1).ok_or("--project-dir requires a path")?)
    } else {
        root()?
    };
    match command {
        "sync-vault" => {
            let source = if let Some(index) = args.iter().position(|item| item == "--source") {
                PathBuf::from(args.get(index + 1).ok_or("--source requires a path")?)
            } else {
                PathBuf::from(std::env::var("HOME")?)
                    .join("Library/Mobile Documents/iCloud~md~obsidian/Documents/My vault")
            };
            sync_vault(&project, &source)
        }
        "archive" => archive(
            &project,
            args.iter().any(|item| item == "--check"),
            args.iter().any(|item| item == "--refresh-generated"),
        ),
        "stage-assets" => stage_assets(&project),
        _ => Err(invalid(format!("Unknown command: {command}"))),
    }
}

fn mirror(source: &Path, destination: &Path) -> Result<()> {
    if !source.is_dir() {
        return Err(invalid(format!("Missing source: {}", source.display())));
    }
    fs::create_dir_all(destination)?;
    let mut paths = HashSet::new();
    for item in WalkDir::new(source).min_depth(1) {
        let item = item?;
        let path = item.path();
        let relative = path.strip_prefix(source)?;
        paths.insert(relative.to_path_buf());
        let target = destination.join(relative);
        if item.file_type().is_dir() {
            fs::create_dir_all(target)?;
        } else if item.file_type().is_file() {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            if !target.is_file() || fs::read(path)? != fs::read(&target)? {
                fs::copy(path, target)?;
            }
        } else if item.file_type().is_symlink() {
            #[cfg(unix)]
            {
                if target.symlink_metadata().is_ok() {
                    fs::remove_file(&target)?;
                }
                std::os::unix::fs::symlink(fs::read_link(path)?, target)?;
            }
        }
    }
    let mut stale = WalkDir::new(destination)
        .min_depth(1)
        .into_iter()
        .collect::<std::result::Result<Vec<_>, _>>()?;
    stale.sort_by_key(|item| std::cmp::Reverse(item.depth()));
    for item in stale {
        let path = item.path();
        if !paths.contains(path.strip_prefix(destination)?) {
            if item.file_type().is_dir() {
                fs::remove_dir(path)?;
            } else {
                fs::remove_file(path)?;
            }
        }
    }
    Ok(())
}

fn sync_vault(project: &Path, source: &Path) -> Result<()> {
    if !source.join(".obsidian").is_dir() {
        return Err(invalid(format!(
            "Not an Obsidian vault: {}",
            source.display()
        )));
    }
    let destination = project.join("raw/my-vault");
    if destination.exists() && source.canonicalize()? == destination.canonicalize()? {
        return Err(invalid("Source and destination are the same"));
    }
    mirror(source, &destination)?;
    for folder in ["plugins", "snippets"] {
        let from = source.join(".obsidian").join(folder);
        if from.is_dir() {
            let to = project.join(".obsidian").join(folder);
            fs::create_dir_all(&to)?;
            for item in WalkDir::new(&from).min_depth(1) {
                let item = item?;
                let target = to.join(item.path().strip_prefix(&from)?);
                if item.file_type().is_dir() {
                    fs::create_dir_all(target)?;
                } else if item.file_type().is_file() {
                    if let Some(parent) = target.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    fs::copy(item.path(), target)?;
                }
            }
        }
    }
    let enabled = project.join(".obsidian/community-plugins.json");
    let original = source.join(".obsidian/community-plugins.json");
    let mut plugins: Vec<String> = serde_json::from_str(&fs::read_to_string(&enabled)?)?;
    for plugin in serde_json::from_str::<Vec<String>>(&fs::read_to_string(original)?)? {
        if !plugins.contains(&plugin) {
            plugins.push(plugin);
        }
    }
    fs::write(
        enabled,
        format!("{}\n", serde_json::to_string_pretty(&plugins)?),
    )?;
    println!(
        "Mirrored My vault to {}; enabled plugins: {}",
        destination.display(),
        plugins.join(", ")
    );
    Ok(())
}

#[derive(Deserialize)]
struct Manifest {
    articles: Vec<ManifestArticle>,
}
#[derive(Deserialize)]
struct ManifestArticle {
    file: String,
    sha256: String,
}

fn git(project: &Path, arguments: &[&str]) -> Result<Vec<u8>> {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(project)
        .output()?;
    if !output.status.success() {
        return Err(invalid(format!(
            "git {} failed: {}",
            arguments.join(" "),
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    Ok(output.stdout)
}

fn stage_assets(project: &Path) -> Result<()> {
    let manifest: Manifest = serde_json::from_str(&fs::read_to_string(
        project.join(".build/site/content-manifest.json"),
    )?)?;
    let embed = Regex::new(r"!\[\[(.*?)\]\]")?;
    let mut allowed = HashSet::<String>::new();
    for article in manifest.articles {
        if !article.file.starts_with("raw/my-vault/Note/") {
            continue;
        }
        let source = project.join(&article.file);
        if !source.is_file() || sha256(&fs::read(&source)?) != article.sha256 {
            return Err(invalid(format!(
                "Missing or changed source (rebuild first): {}",
                article.file
            )));
        }
        for capture in embed.captures_iter(&fs::read_to_string(&source)?) {
            let reference = capture[1].split(['|', '#']).next().unwrap_or_default();
            let asset = project.join("raw/my-vault").join(reference);
            if !asset.is_file() {
                return Err(invalid(format!("Missing asset: {}", asset.display())));
            }
            allowed.insert(relative(&asset, project)?);
        }
    }
    let tracked = git(project, &["ls-files", "-z", "--", "raw/my-vault/Assets"])?;
    let stale = tracked
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
        .map(|path| String::from_utf8_lossy(path).into_owned())
        .filter(|path| !allowed.contains(path))
        .collect::<Vec<_>>();
    if !stale.is_empty() {
        let mut args = vec!["rm", "--cached", "--"];
        args.extend(stale.iter().map(String::as_str));
        git(project, &args)?;
    }
    if !allowed.is_empty() {
        let mut sorted = allowed.iter().map(String::as_str).collect::<Vec<_>>();
        sorted.sort();
        let mut args = vec!["add", "-f", "--"];
        args.extend(sorted);
        git(project, &args)?;
    }
    println!(
        "Staged {} Blog assets; removed {} stale index entries",
        allowed.len(),
        stale.len()
    );
    Ok(())
}

fn catalog(folder: &Path) -> Result<HashMap<String, PathBuf>> {
    let expression = Regex::new(r"(?m)^- source: `([^`]+)`")?;
    let mut existing = HashMap::new();
    for path in markdown_under(folder)? {
        if let Some(capture) = expression.captures(&fs::read_to_string(&path)?) {
            existing.entry(capture[1].to_string()).or_insert(path);
        }
    }
    Ok(existing)
}

fn clean(value: &str) -> String {
    let mut text = value.trim().to_string();
    for (pattern, replacement) in [
        (r"!\[\[[^]]+\]\]", ""),
        (r"\[\[([^]|]+)\|([^]]+)\]\]", "$2"),
        (r"\[\[([^]]+)\]\]", "$1"),
        (r"!?\[([^]]+)]\([^)]+\)", "$1"),
        (r"^(?:[-*+] |\d+[.)] |\*\*\d+[.)]\*\* )", ""),
        (r"<[^>]+>", ""),
    ] {
        text = Regex::new(pattern)
            .expect("valid outline regex")
            .replace_all(&text, replacement)
            .into_owned();
    }
    text = text.replace("**", "").replace('`', "");
    text = Regex::new(r"\s+")
        .expect("valid whitespace regex")
        .replace_all(&text, " ")
        .into_owned();
    text.trim_matches([' ', '-', '|', '：', ':']).to_string()
}

fn outline(path: &Path) -> Result<(String, Vec<String>, Vec<String>)> {
    let raw = fs::read_to_string(path)?;
    let front = Regex::new(r"(?s)\A---\s*\n.*?\n---\s*\n")?;
    let body = front.replace(&raw, "");
    let first = body
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or_default()
        .trim();
    let title = if let Some(heading) = first.strip_prefix("# ") {
        clean(heading)
    } else if let Some(capture) =
        Regex::new(r#"(?m)^title:\s*["']?(.+?)["']?\s*$"#)?.captures(&raw[..raw.len().min(2000)])
    {
        capture[1].trim_matches(['"', '\'', ' ']).to_string()
    } else {
        path.file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned()
    };
    let heading_expression = Regex::new(r"^#{2,4}\s+(.+)")?;
    let excluded = Regex::new(r"(?i)^(初讀摘要|概述|參考|References?|圖片|Figure Sources)$")?;
    let secret = Regex::new(r"(?i)(?:password|secret|token|api.?key)\s*[:=]")?;
    let mut headings = Vec::new();
    let mut prose = Vec::new();
    let mut code = false;
    for line in body.lines() {
        let line = line.trim();
        if line.starts_with("```") || line.starts_with("~~~") {
            code = !code;
            continue;
        }
        if code || line.is_empty() {
            continue;
        }
        if let Some(capture) = heading_expression.captures(line) {
            let value = clean(&capture[1]);
            if !value.is_empty() && !headings.contains(&value) && !excluded.is_match(&value) {
                headings.push(value);
            }
            continue;
        }
        if ["#", "|", "![", "$$", "\\", ">"]
            .iter()
            .any(|prefix| line.starts_with(prefix))
            || line.contains("回到 [[")
            || ["---", "***"].contains(&line)
        {
            continue;
        }
        let mut value = clean(line);
        if value.chars().count() < 25
            || value.starts_with("http://")
            || value.starts_with("https://")
            || secret.is_match(&value)
        {
            continue;
        }
        if value.chars().count() > 240 {
            value = value.chars().take(237).collect::<String>() + "…";
        }
        if !prose.contains(&value) {
            prose.push(value);
        }
    }
    headings.truncate(6);
    prose.truncate(3);
    Ok((title, headings, prose))
}

fn summary_name(path: &Path, project: &Path) -> Result<String> {
    let vault = project.join("raw/my-vault");
    let part = if path.starts_with(vault.join("00_Dashboard")) {
        "dashboard".to_string()
    } else {
        path.strip_prefix(vault.join("Note"))?
            .components()
            .next()
            .ok_or("Missing section")?
            .as_os_str()
            .to_string_lossy()
            .to_lowercase()
    };
    let stem = path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_lowercase();
    let words = Regex::new(r"[a-z0-9]+")?
        .find_iter(&stem)
        .take(5)
        .map(|hit| hit.as_str())
        .collect::<Vec<_>>()
        .join("-");
    let readable = if words.is_empty() {
        "note".to_string()
    } else {
        words
            .chars()
            .take(65)
            .collect::<String>()
            .trim_matches('-')
            .to_string()
    };
    let digest = sha256(relative(path, project)?.as_bytes());
    Ok(format!("my-vault-{part}-{readable}-{}.md", &digest[..9]))
}

fn write_summary(
    path: &Path,
    project: &Path,
    today: &str,
    refresh: bool,
) -> Result<(PathBuf, String)> {
    let source = relative(path, project)?;
    let target = project
        .join("wiki/summaries")
        .join(summary_name(path, project)?);
    if target.exists() && !refresh {
        return Err(invalid(format!("Summary exists: {}", target.display())));
    }
    if target.exists() && !fs::read_to_string(&target)?.contains(&format!("- source: `{source}`")) {
        return Err(invalid("Summary slug collision"));
    }
    let (title, headings, prose) = outline(path)?;
    let dashboard = path.starts_with(project.join("raw/my-vault/00_Dashboard"));
    let section = if dashboard {
        "Dashboard".to_string()
    } else {
        path.strip_prefix(project.join("raw/my-vault/Note"))?
            .components()
            .next()
            .ok_or("Missing section")?
            .as_os_str()
            .to_string_lossy()
            .into_owned()
    };
    let mut topic = if !headings.is_empty() {
        format!("原筆記涵蓋 {}。", headings.join("、"))
    } else if let Some(first) = prose.first() {
        first.clone()
    } else {
        format!("原筆記記錄 {title} 相關內容。")
    };
    if headings.is_empty() && prose.is_empty() {
        let body = fs::read_to_string(path)?;
        if body.trim().is_empty() {
            topic = "來源筆記目前沒有正文，尚無內容可摘要。".into();
        } else if body.contains("![[") || body.contains("![") {
            topic = "來源筆記主要保存圖片嵌入；文字不足以獨立摘要，需查看原始圖片。".into();
        } else if body.contains("```") || body.contains("~~~") {
            topic = format!(
                "來源筆記以 {title} 的命令或程式碼範例為主，使用情境與結果仍需依原文確認。"
            );
        } else if body.contains('|') {
            topic = format!("來源筆記以表格整理 {title}，細節請查看原表格。");
        }
    }
    if topic.chars().count() > 210 {
        topic = topic.chars().take(207).collect::<String>() + "…";
    }
    let description = format!("整理「{title}」的原筆記內容與章節。");
    let archive = if dashboard {
        "dashboard".to_string()
    } else {
        section.to_lowercase()
    };
    let mut lines = vec![
        format!("# {title}"),
        String::new(),
        format!("- source: `{source}`"),
        format!("- source_sha256: `{}`", sha256(&fs::read(path)?)),
        format!("- ingested_at: {today}"),
        "- type: my-vault note summary".into(),
        format!("- collection: {section}"),
        String::new(),
        "## Summary".into(),
        String::new(),
        topic,
        String::new(),
        "## Source Notes".into(),
        String::new(),
    ];
    let details = if headings.is_empty() {
        prose.iter().skip(1).collect::<Vec<_>>()
    } else {
        prose.iter().collect::<Vec<_>>()
    };
    if details.is_empty() {
        lines.push("- 其餘細節請直接查原文；此頁只整理已讀到的文字、章節與內容形式。".into());
    } else {
        lines.extend(details.into_iter().map(|line| format!("- {line}")));
    }
    lines.extend([
        String::new(),
        "## Navigation".into(),
        String::new(),
        format!("- [回到 {section} 歸檔](<../archives/my-vault-{archive}.md>)"),
        String::new(),
    ]);
    fs::write(&target, lines.join("\n"))?;
    Ok((target, description))
}

fn write_archive(project: &Path, name: &str, entries: &[(PathBuf, PathBuf)]) -> Result<()> {
    let title = match name {
        "my-vault-dashboard" => "My vault：主題總覽",
        "my-vault-research" => "My vault：Research 筆記",
        "my-vault-tech" => "My vault：Tech 筆記",
        "web-clipper" => "Web Clipper 來源",
        _ => return Err(invalid("Unknown archive")),
    };
    let mut lines = vec![
        format!("# {title}"),
        String::new(),
        "此頁依來源路徑列出已歸檔的筆記與摘要。完整原文仍以 `raw/` 檔案為準。".into(),
        String::new(),
    ];
    let mut groups = BTreeMap::<String, Vec<(PathBuf, PathBuf)>>::new();
    for (source, summary) in entries {
        let parts = source
            .strip_prefix(project.join("raw"))?
            .components()
            .map(|part| part.as_os_str().to_string_lossy().into_owned())
            .collect::<Vec<_>>();
        let group = if parts[0] != "my-vault" {
            "剪藏".to_string()
        } else if parts[1] == "00_Dashboard" {
            "主題總覽".to_string()
        } else if parts.len() > 3 {
            parts[2..parts.len() - 1].join("/")
        } else {
            "一般筆記".to_string()
        };
        groups
            .entry(group)
            .or_default()
            .push((source.clone(), summary.clone()));
    }
    for (group, mut items) in groups {
        lines.extend([format!("## {group}"), String::new()]);
        items.sort_by_key(|(source, _)| {
            source
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_lowercase()
        });
        for (source, summary) in items {
            let stem = source.file_stem().unwrap_or_default().to_string_lossy();
            let filename = summary.file_name().unwrap_or_default().to_string_lossy();
            lines.push(format!(
                "- [{stem}](<../summaries/{filename}>) · `{}`",
                relative(&source, project)?
            ));
        }
        lines.push(String::new());
    }
    fs::write(
        project.join("wiki/archives").join(format!("{name}.md")),
        lines.join("\n"),
    )?;
    Ok(())
}

fn update_index(project: &Path, added: &[(PathBuf, String)], today: &str) -> Result<()> {
    let index = project.join("wiki/index.md");
    let mut data = fs::read_to_string(&index)?;
    let mut rows = Vec::new();
    for (path, description) in added {
        let stem = path.file_stem().unwrap_or_default().to_string_lossy();
        let filename = path.file_name().unwrap_or_default().to_string_lossy();
        let row = format!("- [{stem}](./summaries/{filename}) · {today}: {description}");
        let expression = Regex::new(&format!(
            r"(?m)^- \[{}\]\(\./summaries/{}\).*?$",
            regex::escape(&stem),
            regex::escape(&filename)
        ))?;
        if expression.is_match(&data) {
            data = expression.replace(&data, row.as_str()).into_owned();
        } else {
            rows.push(row);
        }
    }
    if !rows.is_empty() {
        data = data.replacen(
            "## Summaries\n",
            &format!("## Summaries\n\n{}\n", rows.join("\n")),
            1,
        );
    }
    fs::write(index, data)?;
    Ok(())
}

fn archive(project: &Path, check: bool, refresh: bool) -> Result<()> {
    let vault = project.join("raw/my-vault");
    let mut notes = markdown_under(&vault.join("Note"))?;
    notes.extend(markdown_under(&vault.join("00_Dashboard"))?);
    let mut sources = notes.clone();
    sources.extend(markdown_under(&project.join("raw/web-clipper"))?);
    let mut existing = catalog(&project.join("wiki/summaries"))?;
    let missing = sources
        .iter()
        .filter(|path| !existing.contains_key(&relative(path, project).unwrap_or_default()))
        .cloned()
        .collect::<Vec<_>>();
    let digest = Regex::new(r"(?m)^- source_sha256: `([a-f0-9]+)`")?;
    let mut changed = Vec::new();
    for (source, summary) in &existing {
        let path = project.join(source);
        if !path.is_file() {
            changed.push((source.clone(), "source missing"));
            continue;
        }
        if let Some(capture) = digest.captures(&fs::read_to_string(summary)?) {
            if capture[1] != sha256(&fs::read(&path)?) {
                changed.push((source.clone(), "source changed"));
            }
        }
    }
    if check {
        println!(
            "sources={} archived={} missing={} changed={}",
            sources.len(),
            sources.len() - missing.len(),
            missing.len(),
            changed.len()
        );
        for path in missing.iter().take(20) {
            println!("MISSING {}", relative(path, project)?);
        }
        for (source, reason) in changed.iter().take(20) {
            println!("CHANGED {source} {reason}");
        }
        if !missing.is_empty() || !changed.is_empty() {
            return Err(invalid("Archive coverage check failed"));
        }
        return Ok(());
    }
    let today = Local::now().format("%Y-%m-%d").to_string();
    let mut generated = Vec::new();
    let candidates = if refresh {
        notes
            .iter()
            .filter(|path| {
                let key = relative(path, project).unwrap_or_default();
                !existing.contains_key(&key)
                    || existing.get(&key).is_some_and(|summary| {
                        fs::read_to_string(summary)
                            .unwrap_or_default()
                            .contains("- type: my-vault note summary")
                    })
            })
            .cloned()
            .collect::<Vec<_>>()
    } else {
        missing.clone()
    };
    for path in candidates {
        if !path.starts_with(&vault) {
            return Err(invalid(format!(
                "Web clipper source lacks an existing summary: {}",
                path.display()
            )));
        }
        let (summary, description) = write_summary(&path, project, &today, refresh)?;
        existing.insert(relative(&path, project)?, summary.clone());
        generated.push((summary, description));
    }
    fs::create_dir_all(project.join("wiki/archives"))?;
    for (name, prefix) in [
        ("my-vault-dashboard", "raw/my-vault/00_Dashboard/"),
        ("my-vault-research", "raw/my-vault/Note/Research/"),
        ("my-vault-tech", "raw/my-vault/Note/Tech/"),
        ("web-clipper", "raw/web-clipper/"),
    ] {
        let entries = existing
            .iter()
            .filter(|(source, _)| source.starts_with(prefix))
            .map(|(source, summary)| (project.join(source), summary.clone()))
            .collect::<Vec<_>>();
        write_archive(project, name, &entries)?;
    }
    update_index(project, &generated, &today)?;
    println!(
        "sources={} previously_archived={} written={} changed={}",
        sources.len(),
        sources.len() - missing.len(),
        generated.len(),
        changed.len()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mirror_copies_changes_and_removes_stale_files() {
        let temp = tempfile::tempdir().expect("temporary directory");
        let source = temp.path().join("source");
        let destination = temp.path().join("destination");
        fs::create_dir_all(source.join("nested")).expect("source directory");
        fs::create_dir_all(&destination).expect("destination directory");
        fs::write(source.join("nested/note.md"), "new").expect("source file");
        fs::write(destination.join("old.md"), "old").expect("stale file");
        mirror(&source, &destination).expect("mirror");
        assert_eq!(
            fs::read_to_string(destination.join("nested/note.md")).unwrap(),
            "new"
        );
        assert!(!destination.join("old.md").exists());
    }

    #[test]
    fn archive_creates_summary_and_index_entry_for_new_note() {
        let temp = tempfile::tempdir().expect("temporary directory");
        let project = temp.path();
        let note = project.join("raw/my-vault/Note/Tech/example.md");
        fs::create_dir_all(note.parent().unwrap()).expect("note directory");
        fs::create_dir_all(project.join("wiki/summaries")).expect("summary directory");
        fs::write(&note, "# Example\n\n## Topic\n\nThis is a useful source note with enough detail to summarize.\n").expect("note");
        fs::write(
            project.join("wiki/index.md"),
            "# Wiki Index\n\n## Summaries\n",
        )
        .expect("index");
        archive(project, false, false).expect("archive");
        let summaries = markdown_under(&project.join("wiki/summaries")).expect("summaries");
        assert_eq!(summaries.len(), 1);
        assert!(fs::read_to_string(&summaries[0])
            .unwrap()
            .contains("- source: `raw/my-vault/Note/Tech/example.md`"));
        assert!(fs::read_to_string(project.join("wiki/index.md"))
            .unwrap()
            .contains("my-vault-tech-example-"));
        archive(project, true, false).expect("archive check");
    }
}

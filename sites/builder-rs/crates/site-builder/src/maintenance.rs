use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use regex::Regex;
use serde::Deserialize;
use sha2::{Digest, Sha256};

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

pub fn run(command: &str, args: &[String]) -> Result<()> {
    let project = if let Some(index) = args.iter().position(|item| item == "--project-dir") {
        PathBuf::from(args.get(index + 1).ok_or("--project-dir requires a path")?)
    } else {
        root()?
    };
    match command {
        "stage-assets" => stage_assets(&project),
        _ => Err(invalid(format!("Unknown command: {command}"))),
    }
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
        crate::build_root(project).join("content-manifest.json"),
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

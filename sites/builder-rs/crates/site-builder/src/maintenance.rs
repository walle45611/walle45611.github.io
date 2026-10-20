use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

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
}

use std::fs;
use std::path::Path;
use std::process::Command;

fn write(root: &Path, relative: &str, content: &str) {
    let path = root.join(relative);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, content).unwrap();
}

fn note(marker: &str, slug: &str) -> String {
    format!(
        "---\n{marker}\nblog_title: {slug}\nblog_date: '2026-10-18'\nblog_url: https://blog.walle4561.com/articles/posts/{slug}/\n---\n# {slug}\n\nSee [[Private]].\n"
    )
}

fn build(root: &Path) -> serde_json::Value {
    let output = Command::new(env!("CARGO_BIN_EXE_site-builder"))
        .arg("--project-dir")
        .arg(root)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_str(
        &fs::read_to_string(root.join(".build/site/content-manifest.json")).unwrap(),
    )
    .unwrap()
}

#[test]
fn publishes_only_explicitly_enabled_research_and_tech_notes() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(root, "sites/public/favicon.svg", "<svg/>");
    write(root, "raw/my-vault/Maps/DSA Overview.md", "# 演算法\n");
    let research = "raw/my-vault/Note/Research/Research.md";
    write(root, research, &note("blog: true", "research"));
    write(
        root,
        "raw/my-vault/Note/Tech/Nested/Tech.md",
        &note("blog: true", "tech"),
    );
    for (filename, marker) in [
        ("TagOnly", "tags: [Blog]"),
        ("Disabled", "blog: false\ntags: [Blog]"),
        ("Quoted", "blog: \"true\""),
        ("Legacy", "blog: post"),
        ("Missing", "tags: [note]"),
    ] {
        write(
            root,
            &format!("raw/my-vault/Note/Tech/{filename}.md"),
            &note(marker, &filename.to_lowercase()),
        );
    }
    for path in [
        "raw/my-vault/Note/Private/Private.md",
        "raw/my-vault/Note/TechOther/Outside.md",
        "raw/my-vault/Note/Root.md",
        "raw/my-vault/Maps/Dashboard.md",
        "raw/web-clipper/Enabled.md",
    ] {
        write(root, path, &note("blog: true", "excluded"));
    }
    write(
        root,
        "raw/web-clipper/Tag.md",
        "---\ntags: [Blog]\ntitle: Legacy\nslug: legacy-clip\n---\nLegacy clip\n",
    );
    // A prior export cannot keep publishing an old article after policy changes.
    write(
        root,
        ".build/site/vault-posts/stale.md",
        &note("blog: true", "stale"),
    );

    let manifest = build(root);
    let articles = manifest["articles"].as_array().unwrap();
    assert_eq!(articles.len(), 2);
    for article in articles {
        assert!(matches!(
            article["file"].as_str().unwrap(),
            "raw/my-vault/Note/Research/Research.md" | "raw/my-vault/Note/Tech/Nested/Tech.md"
        ));
    }
    let client = root.join(".build/site/client");
    let page = fs::read_to_string(client.join("articles/posts/research/index.html")).unwrap();
    assert!(page.contains("See Private."));
    assert!(!page.contains("/articles/posts/excluded/"));
    assert!(!client.join("articles/posts/stale/index.html").exists());

    write(
        root,
        research,
        &note("blog: false\ntags: [Blog]", "research"),
    );
    let manifest = build(root);
    assert_eq!(manifest["articles"].as_array().unwrap().len(), 1);
    assert!(!client.join("articles/posts/research/index.html").exists());
    assert!(client.join("articles/posts/tech/index.html").exists());
}

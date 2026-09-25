use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use regex::Regex;
use renderer_wasm::render_markdown_to_html;
use serde::Serialize;
use serde_yaml::Value;
use sha2::{Digest, Sha256};
use unicode_normalization::UnicodeNormalization;

mod maintenance;
mod vault;

const SITE_URL: &str = "https://blog.walle4561.com";
const GA_ID: &str = "G-G0PYR1QYT5";
const ADSENSE_CLIENT: &str = "ca-pub-7412528508334178";

#[derive(Debug, Clone, Serialize)]
struct Article {
    #[serde(rename = "file")]
    source_file: String,
    #[serde(rename = "sha256")]
    source_hash: String,
    route: String,
    slug: String,
    title: String,
    date: Option<String>,
    excerpt: String,
    body_html: String,
    kind: String,
    kind_label: String,
    route_kind: String,
    topic_section: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct ContentManifest {
    source: String,
    selector: String,
    articles: Vec<ContentArticleManifest>,
}

#[derive(Debug, Clone, Serialize)]
struct ContentArticleManifest {
    file: String,
    sha256: String,
    route: String,
}

#[derive(Debug, Clone)]
struct BuilderConfig {
    _project_root: PathBuf,
    raw_dir: PathBuf,
    vault_posts_dir: PathBuf,
    vault_assets_dir: PathBuf,
    out_dir: PathBuf,
    manifest_path: PathBuf,
    styles_path: PathBuf,
    favicon_path: PathBuf,
}

fn main() {
    let arguments: Vec<String> = env::args().skip(1).collect();
    if let Some(command) = arguments.first() {
        if matches!(command.as_str(), "sync-vault" | "archive" | "stage-assets") {
            if let Err(error) = maintenance::run(command, &arguments[1..]) {
                eprintln!("{command} failed: {error}");
                std::process::exit(1);
            }
            return;
        }
    }
    let config = load_config();
    if let Err(error) = vault::export(
        &config._project_root.join("raw/my-vault"),
        &config._project_root.join(".build/site"),
    ) {
        eprintln!("vault export failed: {error}");
        std::process::exit(1);
    }
    let mut articles = Vec::new();
    for (directory, prefix) in [(config.raw_dir.join("web-clipper"), "raw/web-clipper")] {
        if !directory.exists() {
            continue;
        }
        let mut found = read_articles(&directory).unwrap_or_else(|err| {
            eprintln!("failed to read raw posts: {err}");
            std::process::exit(1);
        });
        for article in &mut found {
            article.source_file = format!("{prefix}/{}", article.source_file);
        }
        articles.extend(found);
    }
    if config.vault_posts_dir.exists() {
        let vault_articles = read_articles(&config.vault_posts_dir).unwrap_or_else(|err| {
            eprintln!("failed to read vault posts: {err}");
            std::process::exit(1);
        });
        for mut article in vault_articles {
            articles.retain(|old| old.slug != article.slug);
            article.source_file = format!("raw/my-vault/{}", article.source_file);
            let source = config._project_root.join(&article.source_file);
            let source_bytes = fs::read(&source).unwrap_or_else(|err| {
                eprintln!("failed to read vault source {}: {err}", source.display());
                std::process::exit(1);
            });
            article.source_hash = sha256_hex(&source_bytes);
            articles.push(article);
        }
        sort_articles(&mut articles);
    }
    if let Err(err) = build_site(&config, &articles) {
        eprintln!("build failed: {err}");
        std::process::exit(1);
    }
    println!("Generated {} static article pages.", articles.len());
}

fn load_config() -> BuilderConfig {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut raw_dir = None;
    let mut out_dir = None;
    let mut project_dir = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--raw-dir" => {
                i += 1;
                raw_dir = args.get(i).map(PathBuf::from);
            }
            "--out-dir" => {
                i += 1;
                out_dir = args.get(i).map(PathBuf::from);
            }
            "--project-dir" => {
                i += 1;
                project_dir = args.get(i).map(PathBuf::from);
            }
            "--help" | "-h" => {
                print_usage_and_exit(0);
            }
            _ => {}
        }
        i += 1;
    }

    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let project_root = project_dir
        .filter(|p| p.exists())
        .unwrap_or_else(|| detect_project_root(&cwd));

    let raw_default = project_root.join("raw");
    let build_root = project_root.join(".build").join("site");
    let out_default = build_root.join("client");

    let styles_path = project_root.join("sites").join("styles.css");
    let favicon_path = project_root
        .join("sites")
        .join("public")
        .join("favicon.svg");
    let manifest_path = build_root.join("content-manifest.json");
    let raw_dir = raw_dir.map(|dir| resolve_existing_path(dir, &project_root, "raw"));

    BuilderConfig {
        _project_root: project_root.clone(),
        raw_dir: raw_dir
            .unwrap_or_else(|| resolve_existing_path(raw_default, &project_root, "raw")),
        vault_posts_dir: build_root.join("vault-posts"),
        vault_assets_dir: build_root.join("vault-assets"),
        out_dir: out_dir.unwrap_or(out_default),
        manifest_path,
        styles_path,
        favicon_path,
    }
}

fn resolve_existing_path(candidate: PathBuf, project_root: &Path, fallback_name: &str) -> PathBuf {
    let fallback1 = project_root.join(fallback_name);
    let fallback2 = project_root
        .parent()
        .map(|parent| parent.join(fallback_name));
    let fallback3 = PathBuf::from(fallback_name);
    let fallback2 = fallback2.unwrap_or_else(|| project_root.to_path_buf());
    let candidate_for_default = candidate.clone();
    for path in [candidate, fallback1, fallback2, fallback3].into_iter() {
        if path.exists() {
            return path;
        }
    }
    candidate_for_default
}

fn print_usage_and_exit(code: i32) -> ! {
    println!(
        "\
Usage:
  site-builder [--raw-dir <path>] [--out-dir <path>] [--project-dir <path>]
  site-builder sync-vault [--source <path>]
  site-builder archive [--check | --refresh-generated]
  site-builder stage-assets

Options:
  --raw-dir      Path to raw markdown source (default: <project>/raw)
  --out-dir      Output path for static files (default: <project>/.build/site/client)
  --project-dir  Project root for relative references
"
    );
    std::process::exit(code);
}

fn detect_project_root(cwd: &Path) -> PathBuf {
    let site_builder = cwd.file_name().and_then(|s| s.to_str()) == Some("site-builder");
    if site_builder {
        return cwd
            .parent()
            .and_then(|p| p.parent())
            .unwrap_or(cwd)
            .to_path_buf();
    }

    if cwd.ends_with("builder-rs") {
        return cwd
            .parent()
            .and_then(|p| p.parent())
            .map(PathBuf::from)
            .unwrap_or_else(|| cwd.to_path_buf());
    }

    cwd.to_path_buf()
}

fn read_articles(raw_dir: &Path) -> std::io::Result<Vec<Article>> {
    let mut articles = Vec::new();
    let mut entries = fs::read_dir(raw_dir)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|e| e.path().file_name().map(|n| n.to_owned()));

    for entry in entries {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }

        let raw = fs::read_to_string(&path)?;
        let (front_matter, body) = parse_front_matter(&raw);
        if front_matter.is_empty() || !has_blog_marker(&front_matter) {
            continue;
        }
        let slug = slug_for(
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or_default(),
            &front_matter,
        );
        let title = front_matter_value_str(&front_matter, "title")
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| first_heading(&body).unwrap_or_else(|| "Untitled".to_string()));

        let date = front_matter_value_str(&front_matter, "published")
            .or_else(|| front_matter_value_str(&front_matter, "date"))
            .or_else(|| front_matter_value_str(&front_matter, "created"))
            .and_then(|value| date_only(&value));
        let description = front_matter_value_str(&front_matter, "description");
        let summary_excerpt = excerpt_for_summary(&body);
        let excerpt_raw = description
            .filter(|v| !v.trim().is_empty())
            .unwrap_or(summary_excerpt);
        let excerpt = clip_excerpt(&excerpt_raw);
        let body_html = render_markdown_to_html(&body);
        let hash = sha256_hex(raw.as_bytes());
        let source_file =
            front_matter_value_str(&front_matter, "vault_source").unwrap_or_else(|| {
                path.file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_default()
            });
        let route = format!("/articles/posts/{slug}/");

        articles.push(Article {
            source_file,
            source_hash: hash,
            route: route.clone(),
            slug,
            title,
            date,
            excerpt,
            body_html,
            kind: "post".to_string(),
            kind_label: "Post".to_string(),
            route_kind: "posts".to_string(),
            topic_section: front_matter_value_str(&front_matter, "topic_section"),
        });
    }

    sort_articles(&mut articles);

    Ok(articles)
}

fn sort_articles(articles: &mut [Article]) {
    articles.sort_by(|a, b| {
        match (&a.date, &b.date) {
            (Some(a_date), Some(b_date)) => b_date.cmp(a_date),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.slug.cmp(&b.slug),
        }
        .then_with(|| a.title.cmp(&b.title))
    });
}

fn build_site(config: &BuilderConfig, articles: &[Article]) -> std::io::Result<()> {
    if config.out_dir.exists() {
        fs::remove_dir_all(&config.out_dir)?;
    }
    fs::create_dir_all(&config.out_dir)?;

    write_output(&config.out_dir.join("index.html"), &render_home(articles))?;
    write_output(
        &config.out_dir.join("articles/index.html"),
        &render_archive(articles),
    )?;
    write_output(
        &config
            .out_dir
            .join("topics/data-structures-algorithms/index.html"),
        &render_data_structures_topic(articles),
    )?;
    write_output(&config.out_dir.join("about/index.html"), &render_about())?;
    write_output(
        &config.out_dir.join("404.html"),
        &render_page(
            "Not found",
            "The requested page could not be found.",
            "/404.html",
            r#"<section class="intro"><div class="eyebrow">404</div><h1>Page not found.</h1><p><a href="/articles/">Browse the archive</a></p></section>"#,
            "website",
        ),
    )?;

    for article in articles {
        let article_page = render_post(article);
        write_output(
            &config.out_dir.join(format!(
                "articles/{}/{}/index.html",
                article.route_kind, article.slug
            )),
            &article_page,
        )?;
    }

    let styles = fs::read_to_string(&config.styles_path).unwrap_or_else(|_| default_styles());
    write_output(&config.out_dir.join("styles.css"), &styles)?;
    write_output(&config.out_dir.join("feed.xml"), &render_feed(articles))?;
    write_output(
        &config.out_dir.join("robots.txt"),
        &format!(
            "User-agent: *\nAllow: /\nSitemap: {}/sitemap.xml\n",
            SITE_URL.trim_end_matches('/'),
        ),
    )?;
    write_output(
        &config.out_dir.join("ads.txt"),
        "google.com, pub-7412528508334178, DIRECT, f08c47fec0942fa0\n",
    )?;
    write_output(
        &config.out_dir.join("sitemap.xml"),
        &render_sitemap(articles),
    )?;
    write_output(
        &config.out_dir.join("_headers"),
        "/styles.css\n  Cache-Control: public, max-age=0, must-revalidate\n",
    )?;
    fs::copy(&config.favicon_path, config.out_dir.join("favicon.svg"))?;
    if config.vault_assets_dir.exists() {
        copy_tree(
            &config.vault_assets_dir,
            &config.out_dir.join("vault-assets"),
        )?;
    }

    let manifest_articles = articles
        .iter()
        .map(|article| ContentArticleManifest {
            file: article.source_file.clone(),
            sha256: article.source_hash.clone(),
            route: article.route.clone(),
        })
        .collect::<Vec<_>>();

    let manifest = ContentManifest {
        source: "../raw".to_string(),
        selector: "blog metadata in raw; raw/my-vault is authoritative for matching slugs"
            .to_string(),
        articles: manifest_articles,
    };

    let manifest_json =
        serde_json::to_string_pretty(&manifest).unwrap_or_else(|_| "{}".to_string());
    write_output(&config.manifest_path, &(manifest_json + "\n"))?;

    Ok(())
}

fn copy_tree(source: &Path, destination: &Path) -> std::io::Result<()> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();
        let target = destination.join(entry.file_name());
        if path.is_dir() {
            copy_tree(&path, &target)?;
        } else if path.is_file() {
            fs::copy(path, target)?;
        }
    }
    Ok(())
}

fn render_feed(articles: &[Article]) -> String {
    let items = articles
        .iter()
        .take(20)
        .map(|article| {
            let url = page_url(&article.route);
            let pub_date = article.date.as_deref().and_then(|date| {
                chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d")
                    .ok()
                    .map(|value| value.format("%a, %d %b %Y 00:00:00 +0000").to_string())
            });
            rss::ItemBuilder::default()
                .title(Some(article.title.clone()))
                .link(Some(url.clone()))
                .guid(Some(
                    rss::GuidBuilder::default()
                        .value(url)
                        .permalink(true)
                        .build(),
                ))
                .pub_date(pub_date)
                .description(Some(article.excerpt.clone()))
                .build()
        })
        .collect::<Vec<_>>();
    rss::ChannelBuilder::default()
        .title("Walle Blog")
        .link(SITE_URL)
        .description("Notes on software, systems, and language models.")
        .items(items)
        .build()
        .to_string()
}

fn render_sitemap(articles: &[Article]) -> String {
    let mut urls = vec![
        "/".to_string(),
        "/articles/".to_string(),
        "/topics/data-structures-algorithms/".to_string(),
        "/about/".to_string(),
    ];
    urls.extend(articles.iter().map(|a| a.route.clone()));
    let entries = urls
        .into_iter()
        .map(|pathname| format!("<url><loc>{}</loc></url>", page_url(&pathname)))
        .collect::<String>();
    format!(
        "{}<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">{}</urlset>",
        r#"<?xml version="1.0" encoding="UTF-8"?>"#, entries
    )
}

fn render_home(articles: &[Article]) -> String {
    let list = render_post_list(articles, false);
    let mut body = String::new();
    body.push_str("<section>");
    body.push_str(r#"<div class="section-heading"><h2>Recent posts</h2><a href="/articles/">All posts</a></div>"#);
    body.push_str(&list);
    body.push_str("</section>");
    render_page(
        "Walle Blog",
        "Notes on software, systems, and language models.",
        "/",
        &body,
        "website",
    )
}

fn render_archive(articles: &[Article]) -> String {
    let list = render_post_list(articles, true);
    render_page(
        "Archive",
        "All published posts.",
        "/articles/",
        &list,
        "website",
    )
}

fn render_data_structures_topic(articles: &[Article]) -> String {
    let data = articles
        .iter()
        .filter(|a| a.topic_section.as_deref() == Some("data-structures"))
        .cloned()
        .collect::<Vec<_>>();
    let algorithms = articles
        .iter()
        .filter(|a| a.topic_section.as_deref() == Some("algorithms"))
        .cloned()
        .collect::<Vec<_>>();
    let problems = articles
        .iter()
        .filter(|a| a.topic_section.as_deref() == Some("problem-solving"))
        .cloned()
        .collect::<Vec<_>>();
    let body = format!(
        "<section class=\"intro\"><h1>資料結構與演算法</h1></section><section><h2>資料結構</h2>{}</section><section><h2>演算法</h2>{}</section><section><h2>刷題</h2>{}</section>",
        render_post_list(&data, true),
        render_post_list(&algorithms, true),
        render_post_list(&problems, true)
    );
    render_page(
        "資料結構與演算法",
        "依照 My vault 整理的資料結構與演算法筆記。",
        "/topics/data-structures-algorithms/",
        &body,
        "website",
    )
}

fn render_about() -> String {
    let body = r#"<article class="post">
    <header class="post-header">
      <div class="eyebrow">About</div>
      <h1>Hi, I'm Walle.</h1>
      <p class="post-deck">I build software and study how AI systems can be made useful, reliable, and understandable.</p>
    </header>
    <div class="prose">
      <p>My work sits between software engineering, language models, knowledge systems, and applied research.</p>
      <p>I use this blog to think in public: to keep notes on things I am building, questions I am working through, and ideas worth returning to.</p>
    </div>
  </article>"#;
    render_page("About", "About Walle Blog.", "/about/", body, "website")
}

fn render_post(article: &Article) -> String {
    let deck = format!(
        r#"<p class="post-deck">{}</p>"#,
        escape_html(&article.excerpt)
    );
    let body = format!(
        r#"<article class="post" lang="zh-Hant">
    <header class="post-header">
      {}
      <h1>{}</h1>
      {}
    </header>
    <div class="prose">{}</div>
    <footer class="post-footer"><a href="/articles/">Back to the archive</a></footer>
  </article>"#,
        post_meta(article),
        escape_html(&article.title),
        deck,
        article.body_html
    );
    render_page(
        &article.title,
        &article.excerpt,
        &article.route,
        &body,
        "article",
    )
}

fn render_post_list(articles: &[Article], compact: bool) -> String {
    let class_name = if compact {
        "post-list compact"
    } else {
        "post-list"
    };
    let mut out = String::new();
    out.push_str(r#"<ol class=""#);
    out.push_str(class_name);
    out.push_str(r#"">"#);
    for article in articles {
        out.push_str(r#"<li class="post-item">"#);
        out.push_str(&post_meta(article));
        out.push_str(&format!(
            r#"<h2><a href="{}">{}</a></h2>"#,
            article.route,
            escape_html(&article.title)
        ));
        if !compact {
            out.push_str(&format!(r#"<p>{}</p>"#, escape_html(&article.excerpt)));
        }
        out.push_str("</li>");
    }
    out.push_str("</ol>");
    out
}

fn post_meta(article: &Article) -> String {
    let date = match &article.date {
        Some(value) => format!(r#"<time datetime="{}">{}</time>"#, value, value),
        None => "<span>Undated</span>".to_string(),
    };
    format!(
        "<div class=\"post-meta\">{} · {}</div>",
        date, article.kind_label
    )
}

fn render_page(
    title: &str,
    description: &str,
    pathname: &str,
    body: &str,
    page_type: &str,
) -> String {
    let page_title = if title == "Walle Blog" {
        title.to_string()
    } else {
        format!("{title} | Walle Blog")
    };
    let math_markup = if page_type == "article" && (body.contains("math-tex") || body.contains('$'))
    {
        r#"<script>
      window.MathJax = {
        tex: {
          inlineMath: [['$', '$'], ['\\(', '\\)']],
          displayMath: [['$$', '$$'], ['\\[', '\\]']]
        }
      };
    </script>
    <script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3.2.2/es5/tex-chtml.js"></script>
    <script>
      window.addEventListener('load', () => {
        const prose = document.querySelector('.prose');
        if (!prose) return;

        let pending = false;
        const markOverflow = () => {
          pending = false;
          prose.querySelectorAll('.math-tex').forEach((math) => {
            math.classList.toggle(
              'has-overflow',
              !math.closest('table') && math.scrollWidth > math.clientWidth + 12
            );
          });
        };
        const scheduleCheck = () => {
          if (pending) return;
          pending = true;
          requestAnimationFrame(markOverflow);
        };

        new MutationObserver(scheduleCheck).observe(prose, { childList: true, subtree: true });
        window.addEventListener('resize', scheduleCheck);
        scheduleCheck();
      });
    </script>"#
    } else {
        ""
    };
    format!(
        r##"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>{}</title>
    <meta name="description" content="{}">
    <link rel="canonical" href="{}">
    <link rel="alternate" type="application/rss+xml" title="Walle Blog RSS" href="{}/feed.xml">
    <link rel="icon" href="/favicon.svg" type="image/svg+xml">
    <link rel="preconnect" href="https://www.googletagmanager.com">
    <link rel="preconnect" href="https://pagead2.googlesyndication.com" crossorigin="anonymous">
    <meta name="color-scheme" content="light">
    <meta name="theme-color" content="#ffffff">
    <link rel="stylesheet" href="/styles.css?v=6">
    <meta property="og:type" content="{}">
    <meta property="og:title" content="{}">
    <meta property="og:description" content="{}">
    <meta property="og:url" content="{}">
    {}
  </head>
  <body>
    <header class="site-header">
      <a class="site-title" href="/">Walle Blog</a>
      <p class="site-description">Notes on software, systems, and language models.</p>
      <nav class="site-nav" aria-label="Primary navigation">
        <a href="/">Home</a>
        <a href="/articles/">Archive</a>
        <a href="/topics/data-structures-algorithms/">資料結構與演算法</a>
        <a href="/about/">About</a>
        <a href="/feed.xml">RSS</a>
    </nav>
  </header>
    <main class="site-main">{}</main>
    <footer class="site-footer">Walle Blog · Notes for later.</footer>
    {}
  </body>
</html>
"##,
        escape_html(&page_title),
        escape_html(description),
        page_url(pathname),
        SITE_URL,
        page_type,
        escape_html(title),
        escape_html(description),
        page_url(pathname),
        math_markup,
        body,
        analytics_markup()
    )
}

fn analytics_markup() -> String {
    format!(
        r#"<script>
(function () {{
  function loadScript(src, withCrossOrigin) {{
    var tag = document.createElement("script");
    tag.async = true;
    tag.src = src;
    tag.referrerPolicy = "strict-origin-when-cross-origin";
    if (withCrossOrigin) {{
      tag.crossOrigin = "anonymous";
    }}
    document.head.appendChild(tag);
  }}

  function boot() {{
    if (window.__walleAnalyticsBooted) {{
      return;
    }}
    window.__walleAnalyticsBooted = true;

    window.dataLayer = window.dataLayer || [];
    function gtag() {{
      window.dataLayer.push(arguments);
    }}
    window.gtag = window.gtag || gtag;
    window.gtag("js", new Date());
    window.gtag("config", "{ga}");

    loadScript("https://www.googletagmanager.com/gtag/js?id={ga}");
    loadScript("https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client={ads}", true);
  }}

  if (document.readyState === "complete") {{
    if ("requestIdleCallback" in window) {{
      window.requestIdleCallback(boot, {{ timeout: 1200 }});
    }} else {{
      window.setTimeout(boot, 900);
    }}
    return;
  }}
  window.addEventListener("load", function () {{
    if ("requestIdleCallback" in window) {{
      window.requestIdleCallback(boot, {{ timeout: 1200 }});
    }} else {{
      window.setTimeout(boot, 900);
    }}
  }}, {{ once: true }});
}})();
</script>"#,
        ga = GA_ID,
        ads = ADSENSE_CLIENT
    )
}

fn page_url(pathname: &str) -> String {
    format!(
        "{}/{}",
        SITE_URL.trim_end_matches('/'),
        pathname.trim_start_matches('/')
    )
}

fn parse_front_matter(source: &str) -> (HashMap<String, Value>, String) {
    let mut lines = source.lines();
    match lines.next() {
        Some("---") | Some("---\r") => {}
        _ => return (HashMap::new(), source.to_string()),
    }
    let mut frontmatter_lines = Vec::new();
    let mut has_delimiter = false;
    for line in lines.by_ref() {
        if line.trim() == "---" {
            has_delimiter = true;
            break;
        }
        frontmatter_lines.push(line);
    }
    if !has_delimiter {
        return (HashMap::new(), source.to_string());
    }
    let body = lines.collect::<Vec<_>>().join("\n");
    let text = frontmatter_lines.join("\n");
    if text.trim().is_empty() {
        return (HashMap::new(), body);
    }
    let front_matter: HashMap<String, Value> = serde_yaml::from_str(&text).unwrap_or_default();
    (front_matter, body)
}

fn has_blog_marker(front_matter: &HashMap<String, Value>) -> bool {
    if bool_from_value(front_matter.get("blog")) {
        return true;
    }

    let tags = tags_from_value(front_matter.get("tags"));
    if tags.iter().any(|tag| tag.eq_ignore_ascii_case("blog")) {
        return true;
    }

    false
}

fn bool_from_value(value: Option<&Value>) -> bool {
    match value {
        Some(Value::Bool(true)) => true,
        Some(Value::String(value)) => matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "true" | "yes" | "blog" | "post" | "1"
        ),
        _ => false,
    }
}

fn tags_from_value(value: Option<&Value>) -> Vec<String> {
    match value {
        Some(Value::Sequence(values)) => values
            .iter()
            .filter_map(|item| item.as_str())
            .map(|item| item.to_lowercase())
            .collect(),
        Some(Value::String(value)) => value
            .split(',')
            .map(|item| item.trim().to_lowercase())
            .filter(|item| !item.is_empty())
            .collect(),
        _ => Vec::new(),
    }
}

fn front_matter_value_str(front_matter: &HashMap<String, Value>, key: &str) -> Option<String> {
    front_matter
        .get(key)
        .and_then(|value| value.as_str().map(ToString::to_string))
}

fn slug_for(file_name: &str, front_matter: &HashMap<String, Value>) -> String {
    let explicit = front_matter
        .get("slug")
        .and_then(|value| value.as_str())
        .filter(|slug| !slug.is_empty())
        .unwrap_or(file_name);
    let normalized = explicit.nfkc().collect::<String>().to_lowercase();
    let mut slug = String::new();
    let mut last_was_dash = false;
    for ch in normalized.chars() {
        if ch.is_ascii_alphanumeric() || ch.is_alphabetic() || ch.is_numeric() {
            slug.push(ch);
            last_was_dash = false;
        } else {
            if !slug.is_empty() && !last_was_dash {
                slug.push('-');
                last_was_dash = true;
            }
        }
    }
    slug = slug.trim_matches('-').to_string();
    slug
}

fn first_heading(body: &str) -> Option<String> {
    let re = Regex::new(r"(?m)^#\s+(.+)$").expect("regex");
    re.captures(body)
        .and_then(|capture| capture.get(1).map(|m| m.as_str().trim().to_string()))
}

fn strip_markdown(value: &str) -> String {
    let mut output = value.to_string();
    let image = Regex::new(r"!\[[^\]]*\]\([^)]*\)").expect("regex");
    output = image.replace_all(&output, "").to_string();
    let link = Regex::new(r"\[([^\]]+)\]\([^)]*\)").expect("regex");
    output = link.replace_all(&output, "$1").to_string();
    let fenced = Regex::new(r"(?s)```[\s\S]*?```").expect("regex");
    output = fenced.replace_all(&output, "").to_string();
    let inline_code = Regex::new(r"`([^`]+)`").expect("regex");
    output = inline_code.replace_all(&output, "$1").to_string();
    let heading = Regex::new(r"(?m)^\s*#+\s+").expect("regex");
    output = heading.replace_all(&output, "").to_string();
    let list = Regex::new(r"(?m)^\s*[-*+]\s+").expect("regex");
    output = list.replace_all(&output, "").to_string();
    let symbols = Regex::new(r"[>*_~]").expect("regex");
    output = symbols.replace_all(&output, "").to_string();
    let spaces = Regex::new(r"\s+").expect("regex");
    output = spaces.replace_all(&output, " ").to_string();
    output.trim().to_string()
}

fn excerpt_for_summary(body: &str) -> String {
    let heading = Regex::new(r"(?im)^##\s*Summary\s*$").expect("regex");
    let next_heading = Regex::new(r"(?im)^##\s+").expect("regex");

    let summary = heading
        .find(body)
        .map(|start| {
            let after_heading = &body[start.end()..];
            let next = next_heading.find(after_heading);
            let end = next.map_or(after_heading.len(), |m| m.start());
            after_heading[..end].trim().to_string()
        })
        .filter(|text| !text.is_empty())
        .unwrap_or_else(|| {
            body.split_once("<!--more-->")
                .map(|(head, _)| head.to_string())
                .unwrap_or_else(|| body.to_string())
        });
    strip_markdown(&summary)
}

fn clip_excerpt(input: &str) -> String {
    let mut chars = input.chars();
    let maybe = chars.by_ref().take(187).collect::<String>();
    if input.chars().count() > 190 {
        format!("{maybe}...")
    } else {
        input.to_string()
    }
}

fn date_only(value: &str) -> Option<String> {
    let re = Regex::new(r"(\d{4})[-/]?(\d{2})[-/]?(\d{2})").expect("regex");
    re.captures(value)
        .map(|cap| format!("{}-{}-{}", &cap[1], &cap[2], &cap[3]))
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

fn write_output(path: &Path, content: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\"', "&quot;")
        .replace('\'', "&#39;")
}

fn default_styles() -> String {
    String::from(":root { color-scheme: light; }\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn parses_front_matter_and_filters_blog_posts() {
        let tmp = tempdir().expect("tmp");
        let raw = tmp.path().join("blog.md");
        let mut file = File::create(&raw).expect("create");
        writeln!(
            file,
            "---\ntitle: test title\nslug: my-post\nblog: true\npublished: 2024-08-01\n---\n# Title\nHello\n"
        )
        .expect("write");

        let raw2 = tmp.path().join("note.md");
        let mut file2 = File::create(&raw2).expect("create");
        writeln!(
            file2,
            "---\ntitle: other\ntags: [note]\ndate: 2024-01-01\n---\nNo\n"
        )
        .expect("write");

        let articles = read_articles(tmp.path()).expect("articles");
        assert_eq!(articles.len(), 1);
        assert_eq!(articles[0].slug, "my-post");
    }

    #[test]
    fn source_url_alone_does_not_publish() {
        let mut fm = HashMap::new();
        fm.insert(
            "source".to_string(),
            Value::String("https://blog.walle4561.com/2026".into()),
        );
        assert!(!has_blog_marker(&fm));
    }

    #[test]
    fn keeps_slug_stable_for_filename_with_unicode() {
        let mut fm = HashMap::new();
        fm.insert("slug".to_string(), Value::String("My Title".into()));
        assert_eq!(slug_for("檔案名稱.md", &fm), "my-title");
        let route = format!("/articles/posts/{}/", slug_for("檔案名稱.md", &fm));
        assert!(route.ends_with('/'));
    }

    #[test]
    fn builds_markdown_to_html() {
        let html = render_markdown_to_html("# Heading\n\n`code`");
        assert!(html.contains("<h1"));
        assert!(html.contains("<code>code</code>"));
    }

    #[test]
    fn feed_is_valid_rss_with_a_description_and_publication_date() {
        let article = Article {
            source_file: "raw/my-vault/Note/test.md".into(),
            source_hash: "test".into(),
            route: "/articles/posts/test/".into(),
            slug: "test".into(),
            title: "測試 & RSS".into(),
            date: Some("2026-09-24".into()),
            excerpt: "A < B & C".into(),
            body_html: String::new(),
            kind: "post".into(),
            kind_label: "Post".into(),
            route_kind: "posts".into(),
            topic_section: None,
        };
        let feed = render_feed(&[article]);
        let channel = rss::Channel::read_from(feed.as_bytes()).expect("valid RSS XML");
        assert_eq!(channel.items().len(), 1);
        assert_eq!(channel.items()[0].description(), Some("A < B & C"));
        assert_eq!(
            channel.items()[0].pub_date(),
            Some("Thu, 24 Sep 2026 00:00:00 +0000")
        );
    }
}

# Walle Blog

A minimal static HTML/CSS blog generated from `raw/` markdown files and
versioned snapshots of the My vault data-structures notes.

The build pipeline is now fully Rust-driven and deployed directly to **Cloudflare Pages**.

## Architecture

- `sites/builder-rs/crates/site-builder` is the CLI entrypoint (`site-builder`).
- `sites/builder-rs/crates/renderer-wasm` is the markdown rendering crate
  (built for native first, and also compilable as wasm target).
- `raw/` keeps the legacy blog articles and other archived sources.
- `sites/vault-posts/` contains generated blog snapshots. The corresponding
  notes in My vault are the source of truth; snapshots with the same slug
  replace legacy `raw/` articles at build time without changing those files.
- `sites/public/vault-assets/` contains copied images referenced by those posts.
- `dist/client` is the static output directory for deployment.

## Content filtering

A raw document is published when its frontmatter has either:

```yaml
blog: true
```

or a `blog` tag:

```yaml
tags:
  - blog
```

or when `source` is under `https://blog.walle4561.com/` (legacy migration path).

All other raw documents are ignored.

## Sync data-structures and algorithms notes from My vault

Run this before publishing changes to the data-structures section:

```bash
python3 sites/sync-vault-posts.py --vault "/path/to/My vault"
cd sites/builder-rs
cargo run -p site-builder --release -- --project-dir ../.. --raw-dir ../../raw --out-dir ../../sites/dist/client
```

The exporter reads links under `# 資料結構` and `# 演算法` in
`00_Dashboard/資料結構和演算法 Overview.md`, converts Obsidian image embeds,
copies required assets, and leaves My vault and existing `raw/` files untouched.
Commit the generated posts and assets with the site changes so CI can build
without access to the local vault. The empty `Rod-Cutting Problem` note is
skipped until it has content.

## Commands

```bash
cd sites/builder-rs
cargo build --workspace --release
cargo build --target wasm32-unknown-unknown -p renderer-wasm
cargo run -p site-builder --release -- --raw-dir ../../raw --out-dir ../../sites/dist/client
```

## Analytics

- GA4 measurement ID: `G-G0PYR1QYT5`
- AdSense publisher ID: `ca-pub-7412528508334178`

`ads.txt` and analytics tags are injected into generated HTML pages automatically.

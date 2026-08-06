# Walle Blog

A minimal static HTML/CSS blog generated from `raw/` markdown files.

The build pipeline is now fully Rust-driven and deployed directly to **Cloudflare Pages**.

## Architecture

- `sites/builder-rs/crates/site-builder` is the CLI entrypoint (`site-builder`).
- `sites/builder-rs/crates/renderer-wasm` is the markdown rendering crate
  (built for native first, and also compilable as wasm target).
- `raw/` is the canonical source directory.
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

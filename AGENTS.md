# Repository Guidelines

## Project Structure & Module Organization
This repository is a Hexo blog. Author content lives in `source/`, especially `source/_posts/` for posts and `source/_data/` for shared template data such as `head.njk`. Global site settings are in `_config.yml`; theme-specific settings are in `_config.next.yml`. Scaffolds for new pages and posts live in `scaffolds/`. Generated output is written to `public/`, and deployment artifacts may appear in `.deploy_git/`; do not hand-edit either directory.

## Build, Test, and Development Commands
Use the npm scripts defined in `package.json`:

- `npm run server`: start the local Hexo dev server.
- `npm run build`: generate the static site into `public/`.
- `npm run clean`: remove cached/generated Hexo output before a rebuild.
- `npm run deploy`: publish using the configured Hexo deploy target.

CI builds on Node.js 20 in [`.github/workflows/pages.yml`](/Users/heguowei/project/blog/.github/workflows/pages.yml), and the repo includes `.nvmrc`; match that version locally when possible.

## Coding Style & Naming Conventions
Follow [`.editorconfig`](/Users/heguowei/project/blog/.editorconfig): 4-space indentation, LF line endings, UTF-8. Keep Markdown and YAML changes minimal and readable. Preserve existing Hexo front matter keys such as:

```yaml
---
title: Example Post
date: 2026-04-02
tags:
---
```

Post filenames in `source/_posts/` are descriptive and usually kebab-case or title-based; prefer ASCII names unless the article title clearly requires another language.

## Testing Guidelines
There is no dedicated test suite in this repository. Treat `npm run build` as the required verification step for content, config, and theme changes. When modifying rendered content, also spot-check the local site with `npm run server`.

## Commit & Pull Request Guidelines
Recent history follows Conventional Commit style, for example `feat: add comprehensive MicroK8s deployment guide` and `fix: correct Google Analytics tracking ID format`. Keep commit messages in the form `<type>: <summary>`.

Pull requests should include a short description, the scope of changed content or config, and screenshots for visible theme/layout changes. Call out deployment-impacting edits to `_config.yml`, `_config.next.yml`, workflow files, or Cloudflare-related settings.

## Security & Configuration Tips
Do not commit secrets or service tokens. Cloudflare Pages deploys from GitHub Actions on pushes to `main`, so review workflow and config changes carefully before merging.

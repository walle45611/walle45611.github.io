# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

This is a Hexo static site generator blog featuring technical content, primarily written in Traditional Chinese. The blog uses the NexT theme and deploys to Cloudflare Pages via GitHub Actions.

## Common Commands

### Development
- `npm run server` - Start local development server (http://localhost:4000)
- `npm run clean` - Clean generated files and cache
- `npm run build` - Generate static site files (outputs to ./public)
- `npm run deploy` - Deploy to configured git repository

### Content Management
- `hexo new post "Post Title"` - Create new blog post
- `hexo new page "Page Name"` - Create new page
- `hexo new draft "Draft Title"` - Create new draft

## Architecture

### Configuration
- `_config.yml` - Main Hexo configuration (site metadata, permalink structure, theme selection)
- `_config.next.yml` - NexT theme configuration (appearance, features, integrations)
- Uses `hexo-abbrlink` for SEO-friendly URLs with CRC16 hex encoding

### Content Structure
- `source/_posts/` - Blog posts in Markdown format
- `source/_data/head.njk` - Custom head template for additional HTML
- Posts use front-matter with `mathjax: true` for mathematical content
- Content covers data structures, algorithms, system administration, and programming tutorials

### Theme and Features
- NexT theme with Gemini scheme
- MathJax enabled for mathematical equations
- Utterances comments system
- Google Analytics integration
- Local search functionality
- Related posts feature using `hexo-related-popular-posts`

### Deployment
- GitHub Actions workflow (`.github/workflows/pages.yml`) builds and deploys to Cloudflare Pages
- Requires Pandoc for advanced Markdown rendering via `hexo-renderer-pandoc`
- Builds on Node.js 20 with npm cache optimization
- Deployment triggered on pushes to main branch

### Dependencies
- Hexo 7.3.0 as static site generator
- NexT theme for appearance
- Pandoc renderer for enhanced Markdown processing
- Various Hexo plugins for sitemap, search, and content generation

## Content Guidelines

Posts are primarily technical tutorials and educational content in Traditional Chinese, covering:
- Data structures and algorithms
- System administration (K8s, LDAP, networking)
- Programming environments and tools
- Competitive programming solutions

Mathematical content uses MathJax with LaTeX syntax in front-matter enabled posts.
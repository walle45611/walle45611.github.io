---
title: "Codex for (almost) everything"
source: "https://openai.com/index/codex-for-almost-everything/"
author:
published: 2026-04-15
created: 2026-04-22
description: "The updated Codex app for macOS and Windows adds computer use, in-app browsing, image generation, memory, and plugins to accelerate developer workflows."
tags:
  - "clippings"
---
We’re releasing a major update to Codex, making it a more powerful partner for the more than 3 million developers who use it every week to accelerate work across the full software development lifecycle.

Codex can now operate your computer alongside you, work with more of the tools and apps you use everyday, generate images, remember your preferences, learn from previous actions, and take on ongoing and repeatable work. The Codex app also now includes deeper support for developer workflows, like reviewing PRs, viewing multiple files & terminals, connecting to remote devboxes via SSH, and an in-app browser to make it faster to iterate on frontend designs, apps, and games.

## Extending Codex beyond coding

With background computer use, Codex can now use all of the apps on your computer by seeing, clicking, and typing with its own cursor. Multiple agents can work on your Mac in parallel, without interfering with your own work in other apps. For developers, this is helpful for iterating on frontend changes, testing apps, or working in apps that don’t expose an API.

<iframe src="https://player.vimeo.com/video/1183780832?h=f579830b18&amp;amp%3Bbadge=0&amp;amp%3Bautopause=0&amp;amp%3Bplayer_id=0&amp;amp%3Bapp_id=58479&amp;controls=0&amp;autopause=0" width="100%" height="100%" title=" from OpenAI on Vimeo" allow="autoplay; fullscreen; picture-in-picture; clipboard-write"></iframe>

Codex is also beginning to work natively with the web. The app now includes an in-app browser, where you can comment directly on pages to provide precise instructions to the agent. This is useful for frontend and game development today, and over time we plan to expand it so Codex can fully command the browser beyond web applications on localhost.

<iframe src="https://player.vimeo.com/video/1183780962?h=deca50f4ea&amp;amp%3Bbadge=0&amp;amp%3Bautopause=0&amp;amp%3Bplayer_id=0&amp;amp%3Bapp_id=58479&amp;controls=0&amp;autopause=0" width="100%" height="100%" title=" from OpenAI on Vimeo" allow="autoplay; fullscreen; picture-in-picture; clipboard-write"></iframe>

Codex can now use [gpt-image-1.5 ⁠](https://developers.openai.com/api/docs/models/gpt-image-1.5) to generate and iterate on images. Combined with screenshots and code, it is helpful for creating visuals for product concepts, frontend designs, mockups, and games inside the same workflow.

<iframe src="https://player.vimeo.com/video/1183780933?h=e9bf9c509d&amp;amp%3Bbadge=0&amp;amp%3Bautopause=0&amp;amp%3Bplayer_id=0&amp;amp%3Bapp_id=58479&amp;controls=0&amp;autopause=0" width="100%" height="100%" title=" from OpenAI on Vimeo" allow="autoplay; fullscreen; picture-in-picture; clipboard-write"></iframe>

We’re also releasing more than 90 additional plugins, which combine skills, app integrations, and MCP servers to give Codex more ways to gather context and take action across your tools. Some of the new plugins developers will find most useful include Atlassian Rovo to help manage JIRA, CircleCI, CodeRabbit, GitLab Issues, Microsoft Suite, Neon by Databricks, Remotion, Render, and Superpowers.

<iframe src="https://player.vimeo.com/video/1183781009?h=5a011a7afc&amp;amp%3Bbadge=0&amp;amp%3Bautopause=0&amp;amp%3Bplayer_id=0&amp;amp%3Bapp_id=58479&amp;controls=0&amp;autopause=0" width="100%" height="100%" title=" from OpenAI on Vimeo" allow="autoplay; fullscreen; picture-in-picture; clipboard-write"></iframe>

## Working across the software development lifecycle

The app now includes support for addressing GitHub review comments, running multiple terminal tabs, and connecting to remote devboxes over SSH in alpha. It also lets you open files directly in the sidebar with rich previews for PDFs, spreadsheets, slides, and docs, and use a new summary pane to track agent plans, sources, and artifacts.

<iframe src="https://player.vimeo.com/video/1183780755?h=d6742b2d60&amp;amp%3Bbadge=0&amp;amp%3Bautopause=0&amp;amp%3Bplayer_id=0&amp;amp%3Bapp_id=58479&amp;controls=0&amp;autopause=0" width="100%" height="100%" title=" from OpenAI on Vimeo" allow="autoplay; fullscreen; picture-in-picture; clipboard-write"></iframe>

Together, these improvements make it faster to move across all the stages of the software development lifecycle between writing code, checking outputs, reviewing changes, and collaborating with the agent in one workspace.

## Carry work forward over time

We have expanded automations to allow re-using existing conversation threads, preserving context previously built up. Codex can now schedule future work for itself and wake up automatically to continue on a long-term task, potentially across days or weeks.

Teams use automations for everything from landing open pull requests to following up on tasks and staying on top of fast-moving conversations across tools like Slack, Gmail, and Notion.

<iframe src="https://player.vimeo.com/video/1183782137?h=d0f18d769a&amp;amp%3Bbadge=0&amp;amp%3Bautopause=0&amp;amp%3Bplayer_id=0&amp;amp%3Bapp_id=58479&amp;controls=0&amp;autopause=0" width="100%" height="100%" title=" from OpenAI on Vimeo" allow="autoplay; fullscreen; picture-in-picture; clipboard-write"></iframe>

We’re also releasing a preview of memory, which allows Codex to remember useful context from previous experience, including personal preferences, corrections and information that took time to gather. This helps future tasks complete faster and to a level of quality previously only possible through extensive custom instructions.

Codex now also proactively proposes useful work to continue where you have left off. Using context from projects, connected plugins, and memory, Codex can now suggest how to start your work day or where to pick up on a previous project. For example Codex can identify open comments in Google Docs that require your attention, pull relevant context from Slack, Notion, and your codebase, then provide you with a prioritized list of actions.

## Availability

Starting today, these updates are rolling out to Codex desktop app users who are signed in with ChatGPT.

Personalization features including context-aware suggestions and memory will roll out to Enterprise, Edu, and EU and UK users soon. Computer use is initially available on macOS, and will roll out to EU and UK users soon.

If you’ve been using Codex in the terminal or editor, try it across the rest of your workflow. If you haven’t tried Codex yet, [download the app ⁠](https://openai.com/codex/) and get started.

## What’s next

In just the year since Codex launched, the ways developers are using Codex has expanded. Developers start with Codex to write code, then increasingly use it to understand systems, gather context, review work, debug issues, coordinate with teammates, and keep longer-running work moving.

Our mission is to ensure that AGI benefits all of humanity. That includes narrowing the gap between what people can imagine and what they can build. This release brings Codex closer to the tools, workflows, and decisions involved in building software, with much more to come soon.
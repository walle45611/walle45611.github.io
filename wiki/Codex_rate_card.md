---
title: Codex rate card
source: https://help.openai.com/en/articles/20001106-codex-rate-card
author:
published: 2026-04-02
created: 2026-04-21
description: Learn how Codex credit rates work across Plus, Pro, Business, and Enterprise/Edu plans.
tags:
  - clippings
  - web
---
## Overview

This article outlines the current credit rates for Codex, under the flexible pricing structure for Plus, Pro, Business, and Enterprise/Edu plans.

[Learn more about credits in ChatGPT Plus and Pro.](https://help.openai.com/en/articles/12642688-using-credits-for-flexible-usage-in-chatgpt-freegopluspro-sora)

[Learn more about credits in ChatGPT Business, Enterprise, and Edu.](https://help.openai.com/en/articles/11487671-flexible-pricing-for-the-enterprise-edu-and-business-plans)

**Note**: As of April 2, 2026, we’ve updated Codex pricing to align with API token usage, instead of per-message pricing. This change is applicable to **new and existing Plus, Pro, ChatGPT Business and new ChatGPT Enterprise** plans - please refer to the [new rate card](https://help.openai.com/en/articles/20001106-codex-rate-card#codex-rate-card-token-based-pricing) in the section below for details.  
  
**Customers on existing Enterprise and all other new and existing plans** should continue to use the [legacy rate card](https://help.openai.com/en/articles/20001106-codex-rate-card#legacy-rate-card). We’ll migrate you to the new rates in the upcoming weeks.  
  
All customers should **carefully review both rate cards** to understand the pricing changes, and how they apply to your plan.

## Codex rate card - token based pricing

**This rate card applies to the following customer plans:**

- **New and existing ChatGPT Plus and Pro customers**
- **New and existing ChatGPT Business customers**
- **New Enterprise customers**

Codex usage is priced based on **API token usage,** calculated as credits per million input tokens, cached input tokens and output tokens. Learn more about tokens [here](https://help.openai.com/en/articles/4936856-what-are-tokens-and-how-to-count-them).

This format replaces average per-message estimates with a direct mapping between token usage and credits. It is most useful when you want a clearer view of how input, cached input, and output affect credit consumption.

Under this model, actual credit usage depends on the mix of input, cached input, and output tokens in each task. The table below displays **credits per 1M tokens for each token type.**

| **Model** | **Input Tokens** | **Cached input tokens** | **Output Tokens** |
| --- | --- | --- | --- |
| GPT-5.4 | 62.50 credits | 6.250 credits | 375 credits |
| GPT-5.4-Mini | 18.75 credits | 1.875 credits | 113 credits |
| GPT-5.3-Codex | 43.75 credits | 4.375 credits | 350 credits |
| GPT-5.2 | 43.75 credits | 4.375 credits | 350 credits |
| GPT-5.3-Codex-Spark | *research preview* | *research preview* | *research preview* |
| GPT-Image-1.5 (image) | 200 credits | 50 credits | 800 credits |
| GPT-Image-5.1 (text) | 125 credits | 31.25 credits | 250 credits |

**Note:**

- Fast mode consumes 2x as many credits.
- Code review uses GPT-5.3-Codex.
- GPT-5.3-Codex-Spark may be available in Codex as a research preview - credit rates for this model are not final.
- Read about [Codex usage rate limits](https://developers.openai.com/codex/pricing#what-are-the-usage-limits-for-my-plan).

On average, Codex costs ~$100-$200/developer per month, though there is a large variance depending on model used, number of instances users are running, automations and usage of fast mode. [Read more](https://developers.openai.com/codex/pricing#what-can-i-do-to-make-my-usage-limits-last-longer) about best practices in maximizing your rate limits and managing token consumption.

You can monitor your workspace's token usage in Codex settings > [Usage](https://chatgpt.com/codex/settings/usage) panel.

## Legacy Rate Card

**This rate card applies to the following plans:**

- **Existing Enterprise/Edu customers**
- **New & ex
...(truncated)
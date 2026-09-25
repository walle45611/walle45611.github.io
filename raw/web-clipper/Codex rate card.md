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
- **New & existing ChatGPT Edu, Teacher and Healthcare customers**

*Customers on the plans referenced above should continue using the legacy rate card until we migrate you to the new* [*token-based pricing*](https://help.openai.com/en/articles/20001106-codex-rate-card#codex-rate-card-token-based-pricing) *for Codex. Also read about new Codex seats for ChatGPT* [*Business*](https://help.openai.com/en/articles/8792828-what-is-chatgpt-business#codex-seats-usage-based-no-fixed-cost) *and ChatGPT* [*Enterprise*](https://help.openai.com/en/articles/8265053-what-is-chatgpt-enterprise#overview)*.*

Existing Enterprise/Edu customers should **continue to use the legacy rate card** displayed below, until we migrate you to the new rates in the future.

Specifics of the migration, including timelines, will be provided to **Enterprise admins and owners by email** - contact your OpenAI sales representative if you have questions about the migration.

The legacy rate card expresses Codex usage as approximate average credits per message or pull request. These averages are useful for rough planning, but actual credit usage can vary based on task size, model choice, and reasoning requirements.

|  | **Unit** | **GPT-5.4** | **GPT-5.3-Codex** | **GPT-5.1-Codex-mini** |
| --- | --- | --- | --- | --- |
| Local Tasks | 1 message | ~7 credits | ~5 credits | ~1 credit |
| Cloud Tasks | 1 message | ~34 credits | ~25 credits | Not available |
| Code Review | 1 pull request | ~34 credits | ~25 credits | Not available |

These averages also apply to legacy GPT-5.2, GPT-5.2-Codex, GPT-5.1, GPT-5.1-Codex-Max, GPT-5, GPT-5-Codex, and GPT-5-Codex-Mini.

## FAQ

### Why are there two Codex rate cards?

We’ve modified our pricing from credits per message, to credits per token type consumed. OpenAI supports both the legacy rate card and the updated token-based rate card. The applicable version depends on workspace migration status.

### Which rate card should I use?

New and existing Plus, Pro, ChatGPT Business customers, and new ChatGPT Enterprise customers should use the [**token-based pricing rate card**](https://help.openai.com/en/articles/20001106-codex-rate-card#codex-rate-card-token-based-pricing). Customers on all other plans should use the [**legacy rate card**](https://help.openai.com/en/articles/20001106-codex-rate-card#legacy-rate-card). In the Codex credit purchase modal, the "View rate card" link may open the legacy rate card section; if your plan uses token-based pricing, use the token-based section on this page for current rates. We’ll continue to update this page over time as we migrate your plan to the new rates.

### What changed in the updated token-based rate card?

The legacy rate card shows approximate average credits per message or pull request. The updated token-based rate card shows credits by token type and converts API-priced usage into credits.

### Why is the rate card being changed?

Credits remain the core pricing unit that customers purchase and consume. The updated token-based format makes credit usage easier to map to actual model activity, aligns Codex pricing more closely with token-based metering, and gives clearer visibility into how input, cached input, and output contribute to total usage.

### How does this affect my pricing?

The impact depends on your workload mix. Some users may see higher credit consumption, while others may see lower credit consumption, depending on how much input, cached input, and output their tasks use. Output-heavy tasks and fast mode generally consume more credits than lighter tasks.
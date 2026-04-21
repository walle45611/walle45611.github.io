## [2026-04-22] ingest | AI agent design patterns

- source: `raw/AI agent design patterns.md`
- created: `wiki/summaries/ai-agent-design-patterns.md`, `wiki/concepts/agent-design-patterns.md`
- updated: `wiki/concepts/harness-engineering.md`, `wiki/index.md`, `wiki/log.md`
- notes: 新增 AI agent design patterns 摘要，整理 single / sequential / parallel 三種 workflow pattern 的取捨，並補上 agent workflow 拓撲的概念入口。

## [2026-04-22] ingest | LLM-Wiki-Worker

- source: `raw/LLM-Wiki-Worker.md`
- created: `wiki/summaries/llm-wiki-worker.md`, `wiki/concepts/agentic-knowledge-base-maintenance.md`
- updated: `wiki/concepts/harness-engineering.md`, `wiki/concepts/context-engineering.md`, `wiki/index.md`, `wiki/log.md`
- notes: 新增 LLM-Wiki-Worker 專案摘要，抽出知識庫型 agent 維護模式，補上 raw/wiki 分層、規則路由、queue 背景執行與聊天輸出分離的整合脈絡。

## [2026-04-21] ingest | Cloudflare Pull consumers

- source: `raw/Cloudflare Pull consumers.md`
- created: `wiki/summaries/cloudflare-pull-consumers.md`, `wiki/concepts/cloudflare-queue-consumer-modes.md`
- updated: `wiki/index.md`, `wiki/log.md`
- notes: 新增 Cloudflare Queues pull consumer 主題，整理 pull 與 push 的選型差異、`lease_id`/ack/retry 流程與 `visibility_timeout` 的操作邊界。

## [2026-04-20] lint | wiki health check

- checked: index.md, effective-learning.md, opencode.md, ai-coding-tools.md
- fixed: 修正 index.md 與 effective-learning.md 中的絕對路徑問題（引用了其他 vault 的路徑）；修正 opencode.md 與 ai-coding-tools.md 中的損壞 wikilinks（指向不存在的頁面如 claude-code、mcp）
- gaps: claude-code、mcp 等概念尚未建立獨立頁面；部分 summary 頁面未被概念頁引用

## [2026-04-20] lint | wiki health check

- checked: sync-obsidian-for-free.md, opencode-detailed-guide.md, obsidian-sync-strategies.md, index.md
- fixed: 移除 opencode-detailed-guide.md 中指向不存在頁面的損壞 wikilinks；為 sync-obsidian-for-free.md 補建 obsidian-sync-strategies 概念頁並更新 index.md
- gaps: claude-code、mcp、agent-skills 仍屬缺頁候選，目前僅在 OpenCode 主題中被提及，尚無足夠來源支撐獨立概念頁

## [2026-04-18] ingest | How to Learn Anything Faster Using Modern Research

- source: `raw/How to Learn Anything Faster Using Modern Research.md`
- created: `wiki/summaries/how-to-learn-anything-faster-using-modern-research.md`, `wiki/concepts/effective-learning.md`
- updated: `wiki/index.md`, `wiki/log.md`
- notes: 建立第一批學習科學內容，整理六條過時學習規則，並抽出可跨來源延伸的有效學習概念頁。

## [2026-04-19] ingest | Harness Engineering：有時候語言模型不是不夠聰明，只是沒有人類好好引導

- source: `raw/Harness Engineering：有時候語言模型不是不夠聰明，只是沒有人類好好引導.md`
- created: `wiki/summaries/harness-engineering-language-models-need-human-guidance.md`, `wiki/concepts/harness-engineering.md`, `wiki/concepts/verbalized-feedback-learning.md`
- updated: `wiki/index.md`, `wiki/log.md`
- notes: 新增 AI agent 駕馭工程主題，整理規則檔、工具邊界、回饋迴圈與 verbalized feedback 的可遷移結論，並建立摘要-概念雙向連結。

## [2026-04-19] ingest | 解剖小龍蝦 — 以 OpenClaw 為例介紹 AI Agent 的運作原理

- source: `raw/解剖小龍蝦 — 以 OpenClaw 為例介紹 AI Agent 的運作原理.md`
- created: `wiki/summaries/openclaw-ai-agent-operating-principles.md`, `wiki/concepts/context-engineering.md`
- updated: `wiki/concepts/harness-engineering.md`, `wiki/index.md`, `wiki/log.md`
- notes: 補齊 OpenClaw 視角的 agent 執行細節，整合 system prompt/工具閉環/記憶與壓縮機制，並新增 context engineering 概念頁與安全治理重點。

## [2026-04-19] ingest | MicroK8s 完整部署指南：從安裝到生產級應用

- source: `raw/MicroK8s 完整部署指南：從安裝到生產級應用.md`
- created: `wiki/summaries/microk8s-production-deployment-guide.md`, `wiki/concepts/microk8s-production-readiness.md`, `wiki/concepts/kubernetes-gitops-delivery.md`
- updated: `wiki/index.md`, `wiki/log.md`
- notes: 新增 Kubernetes/MicroK8s 主題，整理生產可用性條件與 GitOps 交付鏈（Helm、Argo CD、Sealed Secrets、Image Updater），並補齊摘要與概念雙向連結。

## [2026-04-20] ingest | Cloudflare Worker AI Pricing

- source: `raw/Cloudflare Worker AI Pricing.md`
- created: `wiki/summaries/cloudflare-workers-ai-pricing.md`, `wiki/concepts/cloudflare-workers-ai-pricing-model.md`, `wiki/concepts/neuron-based-ai-cost-metering.md`
- updated: `wiki/index.md`, `wiki/log.md`
- notes: 建立 Workers AI 計價主題，整理 daily 免費額度與超額神經元計費規則，並抽出 pricing model 與 neuron-based metering 兩條可持續累積的概念軸線。

## [2026-04-20] ingest | Codex 还是 Claude？分享一下我的VibeCoding编程观察

- source: `raw/Codex 还是 Claude？分享一下我的VibeCoding编程观察.md`
- created: `wiki/summaries/codex-vs-claude-vibecoding-observations.md`
- updated: `wiki/concepts/harness-engineering.md`, `wiki/index.md`, `wiki/log.md`
- notes: 新增 AI coding 實戰觀察摘要，將模型比較回寫到 harness 工程脈絡，補強「先架構規劃、後執行產碼」的分層工作流與風險取捨。

## [2026-04-20] ingest | OpenCode详细攻略，开源版Claude Code，免费模型与神级插件

- source: `raw/OpenCode详细攻略，开源版Claude Code，免费模型与神级插件  ai 科技 计算机 编程 coding.md`
- created: `wiki/summaries/opencode-detailed-guide.md`, `wiki/concepts/opencode.md`, `wiki/concepts/ai-coding-tools.md`
- updated: `wiki/index.md`, `wiki/log.md`
- notes: 新增 OpenCode 工具教學摘要，建立 OpenCode 與 AI Coding Tools 概念頁面，強調其作為開源版 Claude Code 的地位與 MCP/Agent Skills 支援。

## [2026-04-20] ingest | How to sync Obsidian 4 FREE · Syncthing vs Autosync

- source: `raw/How to sync Obsidian 4 FREE · Syncthing vs AutosyncPricing.md`
- created: `wiki/summaries/sync-obsidian-for-free.md`
- updated: `wiki/index.md`, `wiki/log.md`
- notes: 整理 Obsidian 免費同步方案，比較 Syncthing 與雲端同步工具的實務差異、風險（同步非備份、避免混用），並建立初步摘要結構。

## [2026-04-21] daily | 2026-04-21

- basis: [[wiki/summaries/cloudflare-pull-consumers.md]]
- updated: wiki/assets/daily/2026-04-21.md
- notes: 1 summary, 1 topic.
## [2026-04-21] ingest | 林宅血案

- source: `raw/林宅血案 - 維基百科，自由的百科全書.md`
- created: `wiki/summaries/lin-zai-xue.md`
- updated: `wiki/index.md`
- notes: 新增林宅血案摘要頁，整理案件概述、主要人物、相關概念與參考來源。

## [2026-04-21] lint | wiki health check

- checked: index.md, summaries, concepts, log.md
- fixed: none
- gaps: claude-code, mcp, agent-skills concepts missing; consider creating concept pages for these.
- notes: No contradictions or isolated pages detected. All summaries reference existing concepts. Index and log are up-to-date.## [2026-04-21] lint | wiki health check

- checked: index.md, summaries, concepts, log.md
- fixed: none
- gaps: claude-code, mcp, agent-skills concepts missing; consider creating concept pages for these.
- notes: No contradictions or isolated pages detected. All summaries reference existing concepts. No outdated content identified.

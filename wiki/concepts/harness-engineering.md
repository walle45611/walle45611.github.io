---
title: "Agent 工作流與 Harness"
tags:
  - concept/agent
aliases:
  - "agent-design-patterns"
---

# Agent 工作流與 Harness

Agent 的可用性取決於模型、上下文、工具與執行流程如何配合。Harness 管理整體環境；工作流拓樸決定各步驟如何分工與交換資料。

## 工作流選擇

- Single agent 適合少量工具與簡單任務；固定步驟可用 sequential workflow，獨立子任務可用 parallel workflow，但後者需要明確的彙整責任。
- 先定義目標、輸入輸出、共享狀態與完成條件，再決定是否拆分代理。更多 agent 不保證可靠性，協調成本、延遲與算力須一起比較。
- 規則檔提供導航與必要約束，固定程式、可執行檢查與工具權限提供較硬的邊界。低噪音、可摘要、結構化的工具輸出有助於控制上下文負擔。

## 生成、檢查與回饋

編輯工作以生成 → 檢查 → 修正形成閉環。PR 審查來源提出自動檢查、AI review、人工 review 三層，並把重複意見轉成可執行檢查。詳細 coding standards 在 review 階段按需載入是特定流程經驗，不表示必要規則應全部移除。

外部檢查器、搜尋或執行結果可提供回饋；verification 成本需與同預算的 sampling／majority vote 比較。研究與教學中的規劃、驗證與人類判斷也須分工，不能只依賴模型自評。

## 工具與隔離

模型位於本地不會消除工具風險；資料邊界、可寫目錄、憑證、網路出口與 container／VM／sandbox 分別治理。可撤回程式碼與不可撤回外部副作用是不同保證。恢復操作與形式驗證有幫助，但需求及驗證模型仍需正確。

## 限制與未解問題

規則長度、代理分工粒度與不同模型族對同一 harness 的反應尚無通用最佳值；需真實任務與可重現成本資料。形式驗證、上下文管理和任務評測分別見 [[formal-verification-for-ai-generated-software]]、[[context-engineering]]、[[llm-evaluation]]。

## 來源

- [[ai-agent-design-patterns]]
- [[ai-self-correction-decoding-workflow-reasoning]]
- [[andrej-karpathy-from-vibe-coding-to-agentic-engineering]]
- [[codex-vs-claude-vibecoding-observations]]
- [[harness-engineering-language-models-need-human-guidance]]
- [[how-i-use-ai-to-learn-things]]
- [[llm-wiki-worker]]
- [[matt-pocock-pr-review-bottleneck]]
- [[max-tegmark-vericoding-ipam-2026]]
- [[openclaw-ai-agent-operating-principles]]
- [[qwen-3-8-27b-dgx-spark-agent-harness]]
- [[tao-princeton-lecture-mathematical-thinking-and-ai]]
- [[tianyin-xu-agent-infrastructure-formal-methods]]

## 相鄰概念

- [[ai-coding-tools]]：coding 工具選擇、成本與審查經驗。
- [[microsoft-entra]]：Agent 與工作負載的身分、存取及稽核。

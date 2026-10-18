---
title: "AI 程式開發工具與審查"
tags:
  - concept/agent
aliases:
  - "opencode"
---

# AI 程式開發工具與審查

AI coding 從補全延伸至規劃、檔案修改、除錯、工具呼叫與代理工作流。評估工具時須同時看能力、上下文、權限、成本與交付驗證，不以單次 demo 判斷整體可靠性。

## 工具能力與來源範圍

已收錄來源以 OpenCode、Claude Code、Codex 等說明這類工具；OpenCode 教學涵蓋模型接入、MCP、skills、subagent 與外掛。這些是對特定時間來源的整理，不保證今天的免費模型、服務可用性、地域政策、外掛品質或帳戶風險。

GPT-5.5 的發布摘要保存當時的代理編碼描述，不能直接當成目前最佳工具排名。工具角色比較與量化交易案例亦只是來源觀察，不能泛化為所有模型固定擅長的工作。

## 工作流與審查

把需求與架構條件說清楚，再讓工具執行與檢查。PR bottleneck 來源指出產生程式變快後，審查容量及現有 codebase 品質更重要；介面行為測試、規範重構、可恢復性與 blast radius 應進入審查設計。

## 成本與限制

Credits／rate card 的單位可能取決於方案、workspace、模型與模式；token 類型、輸出長度、任務大小與 review 量都影響成本。定價來源有時效，使用前需確認實際方案。整體治理見 [[harness-engineering]]，上下文分工見 [[context-engineering]]。

## 來源

- [[andrej-karpathy-from-vibe-coding-to-agentic-engineering]]
- [[codex-rate-card]]
- [[codex-vs-claude-vibecoding-observations]]
- [[matt-pocock-pr-review-bottleneck]]
- [[opencode-detailed-guide]]

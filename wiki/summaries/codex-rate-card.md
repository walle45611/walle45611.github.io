# Codex Rate Card

- source: `raw/Codex rate card.md`
- source link: https://help.openai.com/en/articles/20001106-codex-rate-card
- original title: Codex rate card
- author: (not specified)
- published: 2026-04-02
- created: 2026-04-21
- type: product pricing documentation

## Summary

這份 OpenAI 說明文件記錄 Codex credits 的兩套計價方式與適用方案。新制以每 1M input、cached input 與 output tokens 的 credits 計算；部分 Enterprise/Edu、Edu、Teacher 與 Healthcare 方案在遷移前仍使用以 message 或 pull request 為單位的 legacy rate card。實際消耗取決於模型、token 組成、任務大小、推理需求與 fast mode。

## Key Claims

1. 新的 token-based pricing 適用於新舊 Plus、Pro、Business 與新 Enterprise 客戶；來源同時提醒現有 Enterprise 與其他方案可能仍使用 legacy rate card。
2. 新制依 input、cached input、output 三種 token 類型分開計算 credits，取代平均每則訊息的估算方式。
3. Fast mode 會消耗兩倍 credits；code review 使用 GPT-5.3-Codex；GPT-5.3-Codex-Spark 在來源記錄時仍是 research preview，費率未定案。
4. Legacy rate card 以 local task、cloud task 與 code review 的平均 credits 提供粗略規劃，實際用量仍會因任務與模型而變動。
5. 方案遷移狀態會決定應採用哪一套 rate card，使用者需確認自己的 workspace 與方案，而不能只看頁面上某一個表格。

## Important Details

- Token-based pricing 表列出 GPT-5.4、GPT-5.4-Mini、GPT-5.3-Codex、GPT-5.2 與影像模型等 credits per 1M tokens。
- Legacy pricing 以 GPT-5.4、GPT-5.3-Codex 與 GPT-5.1-Codex-mini 提供 local、cloud 與 code review 的平均 credits。
- 文件指出 Codex 平均成本可能約為每位開發者每月 100–200 美元，但個別差異很大，且取決於模型、並行 instances、自動化與 fast mode。
- 使用者可在 Codex settings 的 Usage 面板監看 workspace token usage。

## Limitations and Boundaries

這是具有時間與方案遷移條件的定價文件；表格、模型名稱、credits 與適用方案都可能更新。此摘要保留來源在 2026-04-02 的版本，不應視為目前帳務的永久保證，實際使用前應回到原始 rate card 確認。

## Related Concepts

- [ai-coding-tools](../concepts/ai-coding-tools.md)

## Alignment With Current Wiki

這份來源補充既有 AI coding 工具頁缺少的成本與 usage governance 面向；它描述的是 Codex 產品計價規則，不等同於模型能力評估或一般 API 定價。

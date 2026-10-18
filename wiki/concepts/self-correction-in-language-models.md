---
title: "模型自我修正與回饋學習"
tags:
  - concept/models
aliases:
  - "verbalized-feedback-learning"
---

# 模型自我修正與回饋學習

自我修正包含偵測錯誤與修正錯誤；文字回饋則是改變後續行為的訊號。當前 context 中的修正、更新長期記憶與更新模型參數是不同層次，不能混稱成模型已學會。

## 技術層次

- Decoding 介入，如 contrastive decoding，以額外 test-time compute 換品質；workflow 用執行器、搜尋、checklist 或驗證器提供回饋。
- Reasoning／RL 訓練可能出現 verification 行為，但 RL 是重排既有解法機率或學到新能力仍有爭論。
- 自然語言回饋可來自人或環境；具體、可操作、可驗證的訊息較適合接到生成 → 評估 → 再生成。錯誤回饋也可能降低品質。

## 比較與評估

事實記憶能力不等於修正能力。外部可驗證訊號通常比單純「再想想」更容易評估，仍須檢查訊號本身可靠性。記錄 error detection、correction、最終品質、過程與成本，並與同預算 sampling／majority vote 比較。

肯定式或質疑式 reflection 可能改變信心與修正傾向；模型族與任務不同，不能設定通用語氣。Judge 校準見 [[llm-evaluation]]；上下文保真見 [[context-engineering]]。

## 限制與未解問題

何時把回饋轉為參數更新、如何識別可學習回饋及避免 judge 高估仍未有通用答案。DeepSeekMath／GRPO 來源僅是影片簡介，缺少逐字稿與量測；它提供名詞入口，不能證明修正成效。驗證流程治理見 [[harness-engineering]]。

## 來源

- [[ai-self-correction-decoding-workflow-reasoning]]
- [[deepseekmath-grpo-introduction]]
- [[harness-engineering-language-models-need-human-guidance]]

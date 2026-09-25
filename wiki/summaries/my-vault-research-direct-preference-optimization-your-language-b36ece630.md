# Direct Preference Optimization: Your Language Model is Secretly a Reward Model

- source: `raw/my-vault/Note/Research/Direct Preference Optimization Your Language Model is Secretly a Reward Model.md`
- source_sha256: `7c362829f00e07cda5174738ab70631cc4fb6d7f458356833e4e0592cd85dd3d`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 偏好資料與 Bradley–Terry 模型、手寫例子與 reward loss、帶 KL 約束的 RLHF 目標、接下來閱讀的問題、手寫原稿。

## Source Notes

- DPO 將偏好對齊轉成直接最佳化偏好資料的分類式目標，簡化先訓練 reward model 再進行強化學習的流程。
- 透過 reward 與最優 policy 的關係，把偏好損失寫成目前模型相對於參考模型的回答機率比，提升偏好答案相對於被拒答案的機率。
- 原文在情緒控制、摘要與單輪對話展示與既有 RLHF 方法相當或更好的結果；理解重點是參考模型與偏好資料如何約束更新，而非把 DPO 視為一般正例微調。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

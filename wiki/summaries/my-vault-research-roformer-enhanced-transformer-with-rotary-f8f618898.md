# RoFormer: Enhanced Transformer with Rotary Position Embedding

- source: `raw/my-vault/Note/Research/@roformer2021.md`
- source_sha256: `a00415c918664ea66ad0bdd6ed7994d35fdda34722ebe6406d4e0ca203156030`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

RoFormer 提出 Rotary Position Embedding（RoPE），將位置資訊透過旋轉作用在 attention 的表示上。

## Source Notes

- 每個位置具有對應旋轉，讓 query 與 key 的內積包含相對位置關係，同時保留絕對位置編碼的形式。
- 原文提供理論分析與長文字分類實驗；位置公式可用於不同序列長度，不代表模型在任意更長輸入上都能保持品質。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

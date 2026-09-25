# RoFormer: Enhanced Transformer with Rotary Position Embedding

[[Assets/Note/Research/Papers/RoFormer - Enhanced Transformer with Rotary Position Embedding/RoFormer - Enhanced Transformer with Rotary Position Embedding.pdf|論文 PDF]]

## 初讀摘要

- RoFormer 提出 Rotary Position Embedding（RoPE），將位置資訊透過旋轉作用在 attention 的表示上。
- 每個位置具有對應旋轉，讓 query 與 key 的內積包含相對位置關係，同時保留絕對位置編碼的形式。
- 原文提供理論分析與長文字分類實驗；位置公式可用於不同序列長度，不代表模型在任意更長輸入上都能保持品質。

摘要依據：PDF 摘要與導論。


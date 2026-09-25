# Attention Is All You Need

- source: `raw/my-vault/Note/Research/Attention Is All You Need.md`
- source_sha256: `c99efd8230e0f574611171787c54e0ee25880df50875527c6801594820a6485d`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 Scaled Dot-Product Attention、一個 query 的完整計算、為何除以平方根、Multi-Head Attention 與 shape、Encoder、Decoder 與 causal mask、FFN 與詞彙輸出。

## Source Notes

- Transformer 以 attention 為核心建立序列轉換模型，取代當時主要依靠循環或卷積的 encoder–decoder。
- 模型結合多頭注意力、位置資訊與逐位置前饋網路；decoder 透過遮罩限制未來資訊，並對 encoder 輸出進行 attention。
- 原文在英德與英法翻譯實驗中取得較好的品質與訓練效率。閱讀時可聚焦 Q、K、V 的角色，以及 self-attention、cross-attention 與位置編碼的分工。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

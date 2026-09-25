# Sequence to Sequence Learning with Neural Networks

- source: `raw/my-vault/Note/Research/Sequence to Sequence Learning with Neural Networks.md`
- source_sha256: `924507440cb7ea99ef897ed02652606c3ec5a083f8f9cc605e60162a6473801b`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

Seq2Seq 使用一個 LSTM encoder 將輸入序列轉為固定維度表示，再由另一個 LSTM decoder 逐步產生輸出序列。

## Source Notes

- 模型端到端學習變長序列映射，原文也發現反轉來源句子的詞序可改善最佳化與翻譯表現。
- 這建立了通用 encoder–decoder 路線；固定向量承載整句資訊的限制，也提供理解後續 attention 方法的起點。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

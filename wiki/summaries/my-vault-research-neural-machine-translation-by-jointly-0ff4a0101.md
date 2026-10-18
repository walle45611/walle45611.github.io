# Neural Machine Translation by Jointly Learning to Align and Translate

- source: `raw/my-vault/Note/Research/@aligntranslate2015.md`
- source_sha256: `faf6f9232de48ba9dfc0509e72206ae3cb3872c5bf28050ad8cd507b50aa869b`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

Bahdanau Attention 針對 encoder 將整句壓成固定長度向量的瓶頸，讓 decoder 每一步選取相關來源位置。

## Source Notes

- 透過可微分的 soft alignment 對來源隱藏表示加權，產生當前目標詞所需的 context，並聯合學習對齊與翻譯。
- 原文英法翻譯結果顯示這種動態取用資訊的方式有效；理解它有助區分 encoder–decoder attention 與後來 Transformer 的 self-attention。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

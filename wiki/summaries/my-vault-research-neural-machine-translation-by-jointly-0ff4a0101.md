# Neural Machine Translation by Jointly Learning to Align and Translate

- source: `raw/my-vault/Note/Research/Neural Machine Translation by Jointly Learning to Align and Translate.md`
- source_sha256: `becb32b521fb37f80b55cb830e8368c879bffe802ef4ed09e9ae1c4ccd7d96f2`
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

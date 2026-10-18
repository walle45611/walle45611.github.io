# QLoRA: Efficient Finetuning of Quantized LLMs

- source: `raw/my-vault/Note/Research/@qlora2023.md`
- source_sha256: `e4c31a32e2cc7e7dc1fdf0d1b14bcd925e985144bf0c2e814ebd495ff93bae28`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

QLoRA 將凍結的預訓練模型以 4-bit 儲存，再把梯度傳到可訓練 LoRA adapter，以降低微調記憶體需求。

## Source Notes

- 主要設計包括 NF4、double quantization 與 paged optimizers，分別處理量化表示、量化常數及記憶體峰值。
- 原文展示大模型可在單張 GPU 上微調，並強調高品質指令資料的重要性；模型權重量化不表示所有計算及 adapter 都以 4-bit 執行。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

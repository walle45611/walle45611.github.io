---
id: "qlora-efficient-finetuning-of-quantized-llms"
type: "paper-conference"
title: "QLoRA: Efficient Finetuning of Quantized LLMs"
issued:
  date-parts:
    - - 2023
container-title: "Advances in Neural Information Processing Systems"
language: "en"
author:
  - family: "Dettmers"
    given: "Tim"
  - family: "Pagnoni"
    given: "Artidoro"
  - family: "Holtzman"
    given: "Ari"
  - family: "Zettlemoyer"
    given: "Luke"
year: "2023"
dateCreated: "2026-10-08"
reading-status: "to-read"
aliases:
  - "QLoRA: Efficient Finetuning of Quantized LLMs"
  - "QLoRA - Efficient Finetuning of Quantized LLMs"
tags:
  - "literature_note"
  - "cmu-sage-ai"
attachment:
  - "[[raw/my-vault/Assets/Note/Research/Papers/QLoRA - Efficient Finetuning of Quantized LLMs.pdf|PDF]]"
---
# QLoRA: Efficient Finetuning of Quantized LLMs

## 初讀摘要

- QLoRA 將凍結的預訓練模型以 4-bit 儲存，再把梯度傳到可訓練 LoRA adapter，以降低微調記憶體需求。
- 主要設計包括 NF4、double quantization 與 paged optimizers，分別處理量化表示、量化常數及記憶體峰值。
- 原文展示大模型可在單張 GPU 上微調，並強調高品質指令資料的重要性；模型權重量化不表示所有計算及 adapter 都以 4-bit 執行。

摘要依據：PDF 摘要與導論。


---
id: "sequence-to-sequence-learning-with-neural-networks"
type: "manuscript"
title: "Sequence to Sequence Learning with Neural Networks"
issued:
  date-parts:
    - - 2014
URL: "https://arxiv.org/abs/1409.3215"
language: "en"
author:
  - family: "Sutskever"
    given: "Ilya"
  - family: "Vinyals"
    given: "Oriol"
  - family: "Le"
    given: "Quoc V."
year: "2014"
dateCreated: "2026-10-08"
reading-status: "to-read"
aliases:
  - "Sequence to Sequence Learning with Neural Networks"
tags:
  - "literature_note"
attachment:
  - "[[raw/my-vault/Assets/Note/Research/Papers/Sequence to Sequence Learning with Neural Networks.pdf|PDF]]"
---
# Sequence to Sequence Learning with Neural Networks

## 初讀摘要

- Seq2Seq 使用一個 LSTM encoder 將輸入序列轉為固定維度表示，再由另一個 LSTM decoder 逐步產生輸出序列。
- 模型端到端學習變長序列映射，原文也發現反轉來源句子的詞序可改善最佳化與翻譯表現。
- 這建立了通用 encoder–decoder 路線；固定向量承載整句資訊的限制，也提供理解後續 attention 方法的起點。

摘要依據：PDF 摘要與導論。


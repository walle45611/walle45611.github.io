---
id: "neural-machine-translation-by-jointly-learning-to-align-and-translate"
type: "paper-conference"
title: "Neural Machine Translation by Jointly Learning to Align and Translate"
issued:
  date-parts:
    - - 2015
URL: "https://arxiv.org/abs/1409.0473"
container-title: "International Conference on Learning Representations"
language: "en"
author:
  - family: "Bahdanau"
    given: "Dzmitry"
  - family: "Cho"
    given: "KyungHyun"
  - family: "Bengio"
    given: "Yoshua"
year: "2015"
dateCreated: "2026-10-08"
reading-status: "to-read"
aliases:
  - "Neural Machine Translation by Jointly Learning to Align and Translate"
tags:
  - "literature_note"
attachment:
  - "[[raw/my-vault/Assets/Note/Research/Papers/Neural Machine Translation by Jointly Learning to Align and Translate.pdf|PDF]]"
---
# Neural Machine Translation by Jointly Learning to Align and Translate

## 初讀摘要

- Bahdanau Attention 針對 encoder 將整句壓成固定長度向量的瓶頸，讓 decoder 每一步選取相關來源位置。
- 透過可微分的 soft alignment 對來源隱藏表示加權，產生當前目標詞所需的 context，並聯合學習對齊與翻譯。
- 原文英法翻譯結果顯示這種動態取用資訊的方式有效；理解它有助區分 encoder–decoder attention 與後來 Transformer 的 self-attention。

摘要依據：PDF 摘要與導論。


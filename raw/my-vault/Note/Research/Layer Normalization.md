---
id: "layer-normalization"
type: "manuscript"
title: "Layer Normalization"
issued:
  date-parts:
    - - 2016
URL: "https://arxiv.org/abs/1607.06450"
language: "en"
author:
  - family: "Ba"
    given: "Jimmy Lei"
  - family: "Kiros"
    given: "Jamie Ryan"
  - family: "Hinton"
    given: "Geoffrey E."
year: "2016"
dateCreated: "2026-10-08"
reading-status: "to-read"
aliases:
  - "Layer Normalization"
tags:
  - "literature_note"
attachment:
  - "[[raw/my-vault/Assets/Note/Research/Papers/Layer Normalization.pdf|PDF]]"
---
# Layer Normalization

## 初讀摘要

- Layer Normalization 以單一樣本同一層的特徵計算平均與變異數，降低正規化對 mini-batch 大小的依賴。
- 正規化後加入可學習的縮放與偏移；訓練與推論使用相同計算，也能在循環網路各時間步分別套用。
- 原文展示隱藏狀態更穩定及訓練加速。閱讀時應分清楚 LayerNorm 與 BatchNorm 各自沿哪些維度計算統計量。

摘要依據：PDF 摘要與導論。


---
id: "onlayernormtransformer2020"
type: "paper-conference"
title: "On Layer Normalization in the Transformer Architecture"
issued:
  date-parts:
    - - 2020
container-title: "Proceedings of the 37th International Conference on Machine Learning"
collection-title: "Proceedings of Machine Learning Research"
volume: "119"
page: "10524-10533"
URL: "https://proceedings.mlr.press/v119/xiong20b.html"
language: "en"
author:
  - family: "Xiong"
    given: "Ruibin"
  - family: "Yang"
    given: "Yunchang"
  - family: "He"
    given: "Di"
  - family: "Zheng"
    given: "Kai"
  - family: "Zheng"
    given: "Shuxin"
  - family: "Xing"
    given: "Chen"
  - family: "Zhang"
    given: "Huishuai"
  - family: "Lan"
    given: "Yanyan"
  - family: "Wang"
    given: "Liwei"
  - family: "Liu"
    given: "Tie-Yan"
year: "2020"
dateCreated: "2026-11-03"
reading-status: "to-read"
aliases:
  - "On Layer Normalization in the Transformer Architecture"
  - "Pre-Norm Transformer"
tags:
  - "literature_note"
---
# On Layer Normalization in the Transformer Architecture

## 初讀摘要

- 研究 Transformer 中 LayerNorm 的位置如何影響訓練穩定性，並分析 learning-rate warm-up 為什麼對原始 Transformer 有幫助。
- 作者指出，Post-LN 在初始化時，靠近輸出端的參數可能有較大的期望 gradient；一開始直接使用較大的 learning rate 會讓訓練不穩。
- Pre-LN 將 LayerNorm 放進 residual block 裡。作者的理論分析顯示，初始化時 gradient 較穩定；在論文測試的任務與設定中，Pre-LN 不用 warm-up 也能得到與 baseline 相近的結果。

摘要依據：論文摘要、導論與分析；[PMLR 正式論文頁](https://proceedings.mlr.press/v119/xiong20b.html)。

## Post-LN 與 Pre-LN

以下省略 dropout，$F_\ell$ 代表某一層的 Attention 或 FFN：

$$
\text{Post-LN:}\quad x_{\ell+1}=\operatorname{LN}\left(x_\ell+F_\ell(x_\ell)\right)
$$

$$
\text{Pre-LN:}\quad x_{\ell+1}=x_\ell+F_\ell\left(\operatorname{LN}(x_\ell)\right)
$$

Pre-LN 的 residual path 保留直接的 $x_\ell$ 項，因此對輸入求導時會有恆等項 $I$：

$$
\frac{\partial x_{\ell+1}}{\partial x_\ell}
=I+J_{F_\ell}J_{\operatorname{LN}}
$$

這條直接路徑讓 gradient 有機會不經過每個子層的轉換，較能維持向前傳遞時的訊號。作者的理論分析也發現，Pre-LN 在初始化時的 gradient 較 well-behaved；Post-LN 則可能在輸出端附近出現較大的 gradient，使 warm-up 對訓練初期特別有用。

**怎麼記：** Pre-LN 的穩定性重點在 residual 提供直接的 gradient path；LayerNorm 放在 Attention／FFN 前，讓子層接收經過正規化的輸入。這是較穩定的傾向，不代表所有模型都能省略 warm-up。

## 延伸連結

[[@attention2017|Attention Is All You Need]]


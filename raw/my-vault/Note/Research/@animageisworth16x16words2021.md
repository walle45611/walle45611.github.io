---
id: "animageisworth16x16words2021"
type: "paper-conference"
title: "An Image is Worth 16x16 Words: Transformers for Image Recognition at Scale"
issued:
  date-parts:
    - - 2021
container-title: "International Conference on Learning Representations"
language: "en"
author:
  - family: "Dosovitskiy"
    given: "Alexey"
  - family: "Beyer"
    given: "Lucas"
  - family: "Kolesnikov"
    given: "Alexander"
  - family: "Weissenborn"
    given: "Dirk"
  - family: "Zhai"
    given: "Xiaohua"
  - family: "Unterthiner"
    given: "Thomas"
  - family: "Dehghani"
    given: "Mostafa"
  - family: "Minderer"
    given: "Matthias"
  - family: "Heigold"
    given: "Georg"
  - family: "Gelly"
    given: "Sylvain"
  - family: "Uszkoreit"
    given: "Jakob"
  - family: "Houlsby"
    given: "Neil"
year: "2021"
URL: "https://openreview.net/forum?id=YicbFdNTTy"
dateCreated: "2026-11-02"
reading-status: "to-read"
aliases:
  - "An Image is Worth 16x16 Words: Transformers for Image Recognition at Scale"
  - "Vision Transformer"
  - "ViT"
tags:
  - "literature_note"
attachment:
  - "[[raw/my-vault/Assets/Note/Research/Papers/An Image is Worth 16x16 Words - Transformers for Image Recognition at Scale.pdf|PDF]]"
---
# An Image is Worth 16x16 Words: Transformers for Image Recognition at Scale

## 初讀摘要

- Vision Transformer（ViT）把圖片切成固定大小的 patches，將每個 patch 攤平並線性投影成 token，再加入 `[CLS]` token 與可學習的位置 embedding。
- 這串 tokens 送進 Transformer encoder；每層以 multi-head self-attention 混合不同 patch 的資訊，再經過 MLP。分類時使用 `[CLS]` 的輸出接分類 head。
- 論文顯示，ViT 在大規模資料上預訓練後能有效用於影像辨識；資料較少時，CNN 的局部性等 inductive bias 仍可能有幫助。

摘要依據：論文摘要、導論與方法（§3）。

## 核心流程

若圖片大小為 $H\times W$、有 $C$ 個 channels，並切成 $P\times P$ patches（假設 $H$ 和 $W$ 都可被 $P$ 整除），則 patch 數量為：

$$
N=\frac{HW}{P^2}
$$

每個 patch 攤平成 $P^2C$ 維後，透過線性投影轉成 $D$ 維 token。ViT 將所有 patch tokens 前面加上 `[CLS]`，並為整串序列加上可學習的一維位置 embedding：

$$
z_0=[x_{class};\,x_p^1E;\,x_p^2E;\,\ldots;\,x_p^NE]+E_{pos}
$$

其中 $E$ 是 patch projection，$E_{pos}$ 是可學習的位置 embedding。Transformer encoder 逐層更新這些 tokens；標準 ViT 的 self-attention 可以直接連結不同位置的 patches，因此從早期層就能整合全圖資訊。最後取 `[CLS]` 的表示送進分類 head，產生 class logits，再用 softmax 換成各類別的機率。這篇的分類流程使用 `[CLS]` 表示，不是 CNN 常見的 feature map pooling 後再 flatten。

**位置編碼提醒：** ViT 沒有移除位置資訊。這篇使用可學習的一維 position embeddings，並非《Attention Is All You Need》的固定 sin/cos positional encoding。

## 和 CNN 的差異

| 面向 | CNN | ViT |
|---|---|---|
| 輸入處理 | kernel 在影像上滑動，逐步產生 feature maps | 影像切成 patches，投影成 token 序列 |
| 資訊互動 | convolution 先看局部 receptive field；堆疊多層後擴大可見範圍 | self-attention 依輸入內容計算 patches 間的關聯；標準 ViT 可跨全圖互動 |
| 內建假設 | 局部連結與 shared filters 讓模型偏向學習可重用的局部 pattern | 沒有相同程度的局部連結假設，通常需要足夠資料學會有用的影像關係 |

可把兩者的架構概念簡記為：

$$
\text{Image}\rightarrow\text{Patch Embedding}+\text{Position Embedding}\rightarrow\text{Transformer Encoder}\rightarrow\text{[CLS]}\rightarrow\text{Classifier}\rightarrow\operatorname{softmax}
$$

## 延伸連結

[[Convolutional Neural Network (CNN)]]


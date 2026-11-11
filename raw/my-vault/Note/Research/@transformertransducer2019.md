---
id: "transformertransducer2019"
type: "manuscript"
title: "Transformer-Transducer: End-to-End Speech Recognition with Self-Attention"
issued:
  date-parts:
    - - 2019
URL: "https://arxiv.org/abs/1910.12977"
language: "en"
author:
  - family: "Yeh"
    given: "Ching-Feng"
  - family: "Mahadeokar"
    given: "Jay"
  - family: "Kalgaonkar"
    given: "Kaustubh"
  - family: "Wang"
    given: "Yongqiang"
  - family: "Le"
    given: "Duc"
  - family: "Jain"
    given: "Mahaveer"
  - family: "Schubert"
    given: "Kjell"
  - family: "Fuegen"
    given: "Christian"
  - family: "Seltzer"
    given: "Michael L."
year: "2019"
dateCreated: "2026-11-02"
reading-status: "to-read"
aliases:
  - "Transformer-Transducer: End-to-End Speech Recognition with Self-Attention"
  - "Transformer-Transducer - End-to-End Speech Recognition with Self-Attention"
tags:
  - "literature_note"
attachment:
  - "[[raw/my-vault/Assets/Note/Research/Papers/Transformer-Transducer - End-to-End Speech Recognition with Self-Attention.pdf|PDF]]"
---
# Transformer-Transducer: End-to-End Speech Recognition with Self-Attention

## 初讀摘要

- 研究將 Transformer encoder 用於串流端到端語音辨識，並以截斷自注意力限制每個時間位置可讀取的上下文。
- 注意力範圍寫成左側歷史 $L$ 與右側 look-ahead $R$；限制兩側可降低計算量並支援串流，但較少上下文可能降低辨識準確度，右側 look-ahead 也會增加延遲。
- 在 LibriSpeech 實驗中，作者採用 $L=32,R=4$ 的設定，報告 test-clean / test-other 的 WER 為 6.37% / 15.30%，並將注意力計算複雜度降至 $O(T)$。

摘要依據：arXiv 摘要、§4.2 與 §5.4。

## 概念筆記

[[Truncated Self-Attention]]

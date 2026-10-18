---
title: "LLM 與 NLP 評估"
tags:
  - concept/models
aliases:
  - "nlp-evaluation-metrics"
---

# LLM 與 NLP 評估

評估先定義要判斷的性質：字面一致、語意、事實性、格式、任務成功、延遲、成本或安全。它們需要不同資料與檢查器，單一總分不能代表全部品質。

## 評估方式

| 方式 | 用途與限制 |
| --- | --- |
| 人工評分 | 貼近使用者偏好，但成本高；需清楚準則與評分者一致性。 |
| Reference metrics | BLEU／METEOR／ROUGE 比對參考答案；受措辭、前處理與 reference 完整性影響。 |
| LLM-as-a-Judge | 適合開放式判斷；需人工校準並檢查位置、長度與自偏好偏誤。 |
| 可執行／結構檢查 | schema、測試與工具或資料庫狀態可驗證明確條件；通過不代表所有需求正確。 |
| Benchmark | 描述指定資料集上的能力輪廓；不能代替產品任務與目標政策的測試。 |

## ROUGE 與生成指標

ROUGE-1／2 看 unigram／bigram 重疊，ROUGE-L 看序列關係，LSUM 對摘要整體作相應比較。應一起觀察 precision、recall、F1：只看 recall 可偏好長輸出。相同 reference、前處理與固定 cohort 才有可比較性；高 overlap 不直接證明事實正確或內容有用。

## Judge 與事實查核

使用具體評分準則、可解析輸出與 rationale；pairwise 比較交換回答順序，測試長度偏誤，並與人工校準集對照。保存 prompt、模型與設定；低溫度不保證完全確定性。事實性可拆 atomic facts，逐項查證再依重要性聚合。

## Agent 與基準

分開檢查工具選擇、參數、執行結果與結果合成，才能定位模型、context、API 描述或實作的問題。MMLU、AIME、PIQA、SWE-bench、HarmBench、Tau-bench 各有任務與政策範圍；比較須看污染、可驗證性及真實目標，避免只追分數。

## 限制與關聯

修正流程同時量測最終品質與成本；judge 自評與自我修正成效不能互相證明。延伸 [[self-correction-in-language-models]]、[[retrieval-augmented-generation]]、[[harness-engineering]]。

## 來源

- [[stanford-cme295-lecture-8-llm-evaluation]]
- [[what-is-the-rouge-metric]]

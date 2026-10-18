---
title: "RAG、證據排序與長上下文"
tags:
  - concept/models
aliases:
  - "long-context-position-effects"
---

# RAG、證據排序與長上下文

RAG 把檢索語料與生成結合，使回答利用模型參數之外的資訊。要同時設計語料、retriever、檢索時機、排序、prompt 組裝與評估；檢索到證據不代表模型已有效使用。

## 來源與架構比較

- 原始 RAG 使用 dense retriever 與 seq2seq，區分 RAG-Sequence／RAG-Token；泛稱「檢索後提示 LLM」不必採同一機制。
- Atlas 使用可更新索引支援知識密集型任務；語料與索引更新方式影響結果。
- Agentic RAG 在多步流程中調整檢索，增加規劃彈性也增加協調、記憶、成本及治理。
- MedRAG 比較語料、retriever 與模型組合，MedGraphRAG 強調證據關聯；基準成績不代替臨床安全驗證。

## 長上下文與位置

Lost in the Middle 在多文件 QA 與 UUID key-value 檢索中觀察位置效應：相關內容在中段可能較難使用。Closed-book 與只含答案文件的 Oracle 是不同基準；干擾文件會改變問題，不能把 context window 上限視為有效資訊利用能力。

增加 top-k 可改善召回，也可能增加噪音與中段證據負擔。先評估 ranking／reranking，再調長度；記錄 relevant chunk 位置、retrieval recall、答案品質及成本。Query 的位置與重複提示策略須依模型實測，不保證普遍改善。

## 限制與關聯

研究模型、資料集與任務不同，不能跨領域直接外推。當前模型的位置效應幅度與最適 prompt 組裝仍需固定 workload 測試。上游內容載入／壓縮見 [[context-engineering]]，評估見 [[llm-evaluation]]，模型機制見 [[transformer-attention-fundamentals]]。

## 來源

- [[lost-in-the-middle-how-language-models-use-long-context-explained]]
- [[my-vault-research-agentic-retrieval-augmented-generation-a-bbb5a39c5]]
- [[my-vault-research-atlas-few-shot-learning-with-5fc8a4cbb]]
- [[my-vault-research-benchmarking-retrieval-augmented-generation-for-0c5032872]]
- [[my-vault-research-lost-in-the-middle-how-0aac0a675]]
- [[my-vault-research-medical-graph-rag-evidence-based-62508d3f8]]
- [[my-vault-research-retrieval-augmented-generation-for-knowledge-00564a249]]

---
id: "retrieval-augmented-generation-for-knowledge-intensive-nlp-tasks"
type: "paper-conference"
title: "Retrieval-Augmented Generation for Knowledge-Intensive NLP Tasks"
issued:
  date-parts:
    - - 2020
container-title: "Advances in Neural Information Processing Systems"
language: "en"
author:
  - family: "Lewis"
    given: "Patrick"
  - family: "Perez"
    given: "Ethan"
  - family: "Piktus"
    given: "Aleksandra"
  - family: "Petroni"
    given: "Fabio"
  - family: "Karpukhin"
    given: "Vladimir"
  - family: "Goyal"
    given: "Naman"
  - family: "Küttler"
    given: "Heinrich"
  - family: "Lewis"
    given: "Mike"
  - family: "Yih"
    given: "Wen-tau"
  - family: "Rocktäschel"
    given: "Tim"
  - family: "Riedel"
    given: "Sebastian"
  - family: "Kiela"
    given: "Douwe"
year: "2020"
dateCreated: "2026-10-08"
reading-status: "to-read"
aliases:
  - "Retrieval-Augmented Generation for Knowledge-Intensive NLP Tasks"
tags:
  - "literature_note"
  - "cmu-sage-ai"
attachment:
  - "[[raw/my-vault/Assets/Note/Research/Papers/Retrieval-Augmented Generation for Knowledge-Intensive NLP Tasks.pdf|PDF]]"
---
# Retrieval-Augmented Generation for Knowledge-Intensive NLP Tasks

## 初讀摘要

- 原始 RAG 工作將模型參數中的知識與外部文件索引結合，改善知識密集型文字生成。
- 使用預訓練 seq2seq 模型與 dense retriever，比較整段生成共用檢索文件的 RAG-Sequence，以及各 token 可利用不同文件的 RAG-Token。
- 原文在開放域 QA 與生成任務展示改善；它是具體可訓練的模型框架，後來廣義的「檢索後提示 LLM」不一定採用完全相同的機制。

摘要依據：PDF 摘要與導論。


---
title: "PEFT、LoRA 與 QLoRA"
tags:
  - concept/models
---

# PEFT、LoRA 與 QLoRA

PEFT 以少量可訓練參數調整模型，減少相對 full fine-tuning 的容量與實驗成本。LoRA 用低秩矩陣表示更新，QLoRA 加入量化底座；三者的成本與自由度不同。

## 參數與覆蓋範圍

Rank 控制低秩容量，alpha 影響更新尺度，dropout 與過擬合控制相關；應在目標任務中一起調整。適配哪些線性層可能與 rank 同樣重要，不能只用更高 rank 解釋成效。

有限硬體或多任務實驗可先評估 adapter 方法，但層覆蓋、rank 與資料規模仍依任務決定。現有來源不足以提供所有架構的最佳值。

## Serving 的前提

Base model 可推論，不表示 runtime 已實作 adapter hook。先確認模型類別、adapter 載入與目標引擎可用，再投入調參；量化及 runtime 相容性是不同檢查。

## 相鄰概念

- [[model-quantization]]：底座與 KV 的精度取捨。
- [[local-llm-deployment]]：runtime、adapter 與部署容量。
- [[llm-evaluation]]：以固定資料與任務結果比較設定。

## 來源

- [[lora-and-qlora-fine-tuning-explained-in-depth]]
- [[vllm-gemma-4-lora-two-pitfalls]]

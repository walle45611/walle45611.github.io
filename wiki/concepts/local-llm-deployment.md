---
title: "LLM 部署、容量與相容性"
tags:
  - concept/models
aliases:
  - "llm-serving-compatibility"
---

# LLM 部署、容量與相容性

部署單位是模型、推論引擎、硬體、adapter、端點及工具權限共同形成的系統。能載入 base model，不等於可用 runtime adapter、長上下文或工具呼叫。

## 容量與效能

權重、KV cache、context、併發與系統空間一起估算；MoE active parameters 主要關乎計算量，不等於載入全部權重的容量。Unified memory 的容量優勢也不保證頻寬、延遲、溫度與功耗符合目標。

KV 儲存精度、每筆大小、保留條數與獨立層副本各自影響容量。稀疏讀取不必縮小常駐快取；分頁、prefix sharing、卸載分別處理配置浪費、重複與位置。特定模型文章的數字不是本機實測。

Continuous batching、chunked prefill 與 prefix caching 分別處理請求排程、長輸入預算與重用。量測 TTFT、decode 間隔、總吞吐、命中率和併發，不用單一 tok/s 或成功 demo 代替可用性評估。

## 相容性檢查

- Model card、權重格式、tokenizer/config、tool parser、MTP metadata、runtime 與硬體架構一起核對。
- Base model 與 base + adapter 是獨立能力；模型類別缺少 adapter hook 時，調 LoRA rank 不會解決問題。
- PyTorch、CUDA wheel、driver 與 native extension 組成版本矩陣；GPU 可見不等於 extension 可載入。官方 recipe 的版本條件不能任意省略。
- Docker 固定環境與啟停方式，但仍需確認目標架構 image、cache 位置與清理。普通生成、工具、端點、長 context／併發都須測試。

## 資料與執行邊界

地端模型縮小資料路徑範圍，無法取代憑證、網路出口、權限、sandbox 隔離及組織資料政策。OpenAI-compatible endpoint 提供介接方式，不代表認證與網路暴露已治理。固定可重複工具可降低自由生成操作的風險。

## 限制與未解問題

先以代表性資料及目標 workload 做 POC；不同量化、引擎與硬體的品質、長 context 和熱限制仍需固定條件 benchmark。適用性不是單靠參數量決定。延伸 [[model-quantization]]、[[parameter-efficient-fine-tuning]]、[[transformer-attention-fundamentals]]、[[harness-engineering]]。

## 來源

- [[deepseek-v4-1-flash-kv-cache-compression]]
- [[how-inference-engines-work-agent-loops]]
- [[qwen-3-8-27b-dgx-spark-agent-harness]]
- [[vllm-gemma-4-lora-two-pitfalls]]

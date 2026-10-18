---
title: "模型量化與 KV 精度"
tags:
  - concept/models
---

# 模型量化與 KV 精度

模型量化降低權重或 activation 的數值精度，以改善記憶體需求；訓練、推論與儲存所需的精度可以不同，不能只看壓縮比例。

## 容量與訓練

QLoRA 用量化底座配合低秩 adapter 降低微調容量需求；量化底座與運算時使用的表示須分清。來源原先「恢復高精度」的描述不表示量化已丟失的資訊可以無損恢復。

粗估 FP8 約一 byte／parameter、BF16 約兩 byte／parameter，實際還包含格式開銷、KV cache、context、併發與系統空間。MoE active parameters 關乎每 token 計算，不能據此只載入同量權重。

## 權重與 KV Cache

權重量化與歷史 K/V 儲存量化是不同對象；除位元數也要計 scale 等開銷，儲存精度不等於全部運算精度。已收錄 KV 拆解中的約 900 bytes/token 是特定配置反推，不是整個請求快取總量。

## 判斷與限制

格式與位寬依模型及 workload 驗證；比較品質、延遲、長 context 與併發，不只比較能否載入。激進量化的可用界線、訓練與推論配置仍無通用最佳值。

## 相鄰概念

- [[parameter-efficient-fine-tuning]]
- [[local-llm-deployment]]
- [[transformer-attention-fundamentals]]

## 來源

- [[lora-and-qlora-fine-tuning-explained-in-depth]]
- [[qwen-3-8-27b-dgx-spark-agent-harness]]
- [[deepseek-v4-1-flash-kv-cache-compression]]

---
title: "Workers AI 成本、配額與計量"
tags:
  - concept/cloud
aliases:
  - "neuron-based-ai-cost-metering"
---

# Workers AI 成本、配額與計量

已收錄的 Workers AI 計價來源以 neurons 作統一帳務單位，連接不同模型與輸入／輸出成本。統一單位方便比較，並不表示相同 token 數具有相同成本。

## 計量與配額

- 同時看模型轉換率、輸入／輸出量及任務類型；非文字工作負載也不能直接套文字 token 價格。
- 來源當時記錄每日免費額度 10,000 neurons、Paid 超額 $0.011／1,000 neurons、00:00 UTC 重置；這是來源的歷史定價，不視為現行報價。
- 在來源描述的模式中，能否付費超額會影響超限失敗行為，因此成本治理亦是可用性治理。

## 工作負載治理

估算每任務成本，依長輸出、多步工作與尖峰流量設計預算及限流；必要時採較小模型、縮短輸出或延後批次。估算值與實際帳務須對帳，平台轉換率變更時重新校準。

## 限制與未解問題

不同任務的真實 neuron 波動、尖峰緩衝與降級成效尚缺多來源實測。部署決策需核對當前文件；背景工作節奏另見 [[cloudflare-queue-consumer-modes]]。

## 來源

- [[cloudflare-workers-ai-pricing]]

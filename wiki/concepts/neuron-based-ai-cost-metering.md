# Neuron-Based AI Cost Metering

## Current View

在目前知識庫中，neuron-based metering 可被理解為一種跨模型計算成本的統一度量：不同模型可保有各自 token 價格與效率曲線，但最終可透過 neuron 轉換率映射到同一帳務單位。這使多模型路由下的成本比較更直接，也暴露出「同樣 token 規模、不同模型成本差異巨大」的事實。

## Stable Conclusions

1. 以單一計量單位（neurons）可降低跨模型成本比較的心智負擔。
2. token 價格與 neuron 轉換率要一起看，單看任一指標都可能誤判成本。
3. 輸入與輸出 token 的 neuron 消耗往往不對稱，輸出長度治理對成本特別敏感。
4. 圖像、音訊等非文字工作負載也能納入同一計價框架，有利平台級預算管理。

## Working Heuristics

- 建立「每任務預估 neurons」欄位，讓模型選擇與成本上限可程式化決策。
- 對高變異工作負載（長輸出、多步生成）採分級限流與預算分桶。
- 每日檢查估算值與實際帳務偏差，定期調整 neuron 估算係數。

## Open Questions

- 不同類型任務在真實流量下的 neuron 波動分布仍缺乏跨來源統計。
- 若平台後續調整模型轉換率，舊有成本預測模型的遷移策略需補充。

## Related Concepts

- [cloudflare-workers-ai-pricing-model](./cloudflare-workers-ai-pricing-model.md)

## Sources

- [cloudflare-workers-ai-pricing](../summaries/cloudflare-workers-ai-pricing.md)

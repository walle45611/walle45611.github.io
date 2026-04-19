# Cloudflare Workers AI Pricing Model

## Current View

在目前知識庫中，Cloudflare Workers AI 的定價模型可視為「每日免費額度 + 超額神經元計費」：所有方案都有每日 10,000 Neurons 免費額度，僅 Paid 方案可繼續以 `$0.011 / 1,000 Neurons` 消費。此模型的運維含義是：成本控制與可用性控制是同一件事，因為超限會直接失敗。

## Stable Conclusions

1. Neurons 是 Workers AI 的帳務核心單位；token 價格主要提供估算與比較視角。
2. 免費額度是 daily reset 機制，不是累積式月額折抵。
3. Free 與 Paid 的關鍵差異在於「能否支付超額」，不是「是否可用 Workers AI」。
4. 超限請求會失敗，代表配額管理直接影響服務可靠性。

## Working Heuristics

- 以每日配額為邊界建立 budget guardrail，並依 `00:00 UTC` 重置時點設計配額視窗。
- 對高 neuron 任務建立降級路由（輕量模型、縮短輸出、延後批次）。
- 在應用層追蹤輸入/輸出 token 與估算 neurons，與帳務面做每日對帳。

## Open Questions

- Cloudflare 目前未在此來源提供配額接近上限時的官方建議降級策略範本。
- 高峰流量下的 neuron 突增緩衝策略（例如硬上限前預警閾值）仍需更多實務資料。

## Related Concepts

- [neuron-based-ai-cost-metering](./neuron-based-ai-cost-metering.md)

## Sources

- [cloudflare-workers-ai-pricing](../summaries/cloudflare-workers-ai-pricing.md)

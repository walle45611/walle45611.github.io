# Cloudflare Worker AI Pricing

- source: https://developers.cloudflare.com/workers-ai/platform/pricing/
- original title: Cloudflare Worker AI Pricing
- author: (not specified)
- published: 2026-04-04
- type: documentation summary

## Summary

這份來源定義 Workers AI 的計價與配額基線：Free 與 Paid 方案都可使用 Workers AI，統一以 Neurons 計費；每日前 10,000 Neurons 免費，超過後在 Paid 方案按 `$0.011 / 1,000 Neurons` 收費。文件同時提供跨模型的 token-to-neuron 對照，讓使用者可由 token 用量估算實際成本，並強調每日 00:00 UTC 重置與超限後請求會直接報錯。

## Key Claims

1. Workers AI 的核心計費單位是 Neurons，而非直接以模型 token 單價作最終帳務單位。
2. Free 與 Paid 都有每日 10,000 Neurons 免費額度，但 Free 不能購買超額用量。
3. 在 Paid 方案，超過免費額度後以 `$0.011 / 1,000 Neurons` 計費。
4. 各模型的 token 價格與 neuron 消耗率不同，成本差異主要來自模型與輸入/輸出結構。
5. 配額與限制每日重置；任一限制超標時，後續操作會失敗而非延遲結算。

## Important Details

- 每日免費額度重置時間為 `00:00 UTC`。
- LLM、Embedding、Image、Audio、Other 類模型都提供 token 與 neuron 對照表。
- 高階 LLM 與多步影像生成模型在 neuron 消耗上顯著高於輕量模型。
- 使用者可在 Cloudflare dashboard 監看 Neuron 用量。

## Practical Takeaways From This Source

- 成本估算要同時看「token 單價」與「neuron 轉換率」，不能只看 token 價格。
- 控制支出時，應優先做模型分級路由與輸出長度治理，降低高 neuron 請求比例。
- 需把 UTC 日切納入配額策略，避免跨時區造成預估與實際用量落差。
- 由於超限會直接失敗，生產流程需要預先設計配額告警與降級路徑。

## Related Concepts

- [cloudflare-workers-ai-pricing-model](../concepts/cloudflare-workers-ai-pricing-model.md)
- [neuron-based-ai-cost-metering](../concepts/neuron-based-ai-cost-metering.md)

## Alignment With Current Wiki

- 這是知識庫第一份 Cloudflare Workers AI 成本資料，屬新主題擴充，暫無既有結論衝突。
- 新來源可拆成兩條長期概念軸線：平台計價機制（方案/配額/超限行為）與計量單位（token-neuron 轉換）。

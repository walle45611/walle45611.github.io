# Cloudflare Pull consumers

- source: `raw/Cloudflare Pull consumers.md`
- source link: [https://developers.cloudflare.com/queues/configuration/pull-consumers/](https://developers.cloudflare.com/queues/configuration/pull-consumers/)
- original title: Cloudflare Pull consumers
- author: (not specified)
- published: 2026-03-31
- type: documentation summary

## Summary

這份文件定義 Cloudflare Queues 的 pull-based consumer：它允許任何 Workers 之外的環境透過 HTTP 主動拉取訊息，再以 `lease_id` 明確 ack 或 retry。相較於預設較容易上手的 push-based consumer，pull 模式的優勢在於可把消費節奏交給既有基礎設施控制，特別適合受上游吞吐、長任務或外部執行環境限制的場景。

## Key Claims

1. Cloudflare Queues 預設較容易上手的是 push-based consumer，但需要接到 Workers 之外的既有系統時，應考慮 pull-based consumer。
2. Pull consumer 不會自動持續消費訊息，而是由客戶端在準備好時主動呼叫 pull API 取得批次訊息。
3. Pull consumer 的訊息處理需要顯式 ack 或 retry，因此消費流程本身就是 queue state mutation。
4. Pull 模式可同時支援多個並行 consumer，每個 consumer 取得獨立 batch，並透過 `visibility_timeout` 管理 lease 有效期。
5. 如果訊息未在 lease 期間內 ack，訊息會回到 queue 重新投遞；顯式 retry 則可立即回佇列，不必等 timeout。

## Important Details

- 啟用 HTTP pull 需使用 `wrangler queues consumer http add $QUEUE-NAME`；若 queue 已有 push consumer，必須先移除既有 worker consumer。
- Pull consumer 需要同時具備 `queues_read` 與 `queues_write` 權限的 API token，因為 ack 會改變 queue state。
- Pull API 預設 `batch_size` 為 5、上限 100；`visibility_timeout` 預設 30 秒、上限 12 小時。
- Queues 採 short polling：有訊息就立即回 batch，無訊息則回空結果，不會長時間保持連線等待。
- 回傳訊息的關鍵欄位包含 `body`、`id`、`timestamp_ms`、`attempts`、`lease_id`；其中 `lease_id` 是 ack/retry 的核心憑證。
- `json` 與 `bytes` 類型的訊息 body 可能以 base64 編碼，consumer 需自行解碼。

## Practical Takeaways From This Source

- 若消費端不在 Workers 內、或需要嚴控吞吐速率，pull consumer 比 push consumer 更合適。
- 設計 pull consumer 時，應把 ack/retry 流程視為應用協定的一部分，而不是單純讀取訊息。
- `visibility_timeout` 要根據實際處理時間調整，避免過短造成重複投遞，或過長拖慢失敗重試。
- 當上游資源有壓力時，顯式 retry 比等待 timeout 更能縮短恢復時間與減少 lease 懸掛。

## Related Concepts

- [cloudflare-queue-consumer-modes](../concepts/cloudflare-queue-consumer-modes.md)

## Alignment With Current Wiki

- 這是知識庫第一份關於 Cloudflare Queues consumer 行為的資料，與既有 Workers AI 計價主題互補，但焦點不同。
- 新來源把 Cloudflare 主題從成本治理擴展到訊息消費控制，後續可累積更多 queue、retry、backpressure 與 API-based consumer 的實務資料。

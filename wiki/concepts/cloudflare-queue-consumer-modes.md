# Cloudflare Queue Consumer Modes

## Current View

在目前知識庫中，Cloudflare Queues 的 consumer mode 可先分成兩類：push-based consumer 與 pull-based consumer。push 模式較接近「讓平台自動把訊息推進 Worker 執行」，適合快速上手與自動擴展；pull 模式則把節奏控制權交還給外部 HTTP client，由消費端在準備好時主動拉取、ack 與 retry，較適合需要與既有基礎設施整合、或必須精細控制吞吐與處理時間的場景。

## Stable Conclusions

1. Push 與 pull 的核心差異不在於功能多寡，而在於誰掌控消費節奏與重試時機。
2. Pull consumer 特別適合 Workers 之外的既有系統，或受上游資源限制的長任務流程。
3. Pull 模式下，ack/retry 是顯式協定，不是平台隱含細節；consumer 必須正面處理 `lease_id` 與 `visibility_timeout`。
4. `visibility_timeout` 的設定直接影響重複投遞風險、恢復速度與 consumer 的失敗語義。

## Working Heuristics

- 若需求只是先讓 queue 開始消費，優先從 push consumer 起步。
- 若消費端不在 Workers 內、或需要主動限流與批次控制，改用 pull consumer。
- 把 `lease_id`、ack、retry 視為應用層狀態機的一部分，避免把 pull consumer 當成單純 HTTP 讀取。
- 根據實際任務耗時調整 `batch_size` 與 `visibility_timeout`，不要只用預設值。

## Open Questions

- 不同負載型態下，push 與 pull 在成本、延遲與操作複雜度上的折衷仍需更多實務案例。
- 多個並行 pull consumer 與上游 autoscaling 之間的最佳協調策略，還需要更多來源補充。

## Related Concepts

- [context-engineering](./context-engineering.md)

## Sources

- [cloudflare-pull-consumers](../summaries/cloudflare-pull-consumers.md)

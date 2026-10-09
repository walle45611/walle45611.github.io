# Latency、Percentiles 與 SLI / SLO / SLA

Arithmetic mean 一般來說在衡量 latency 時會比較不夠有代表性，因為 latency 的 distribution 常常會有 outliers 和 tail latency。相較之下，使用 median 和 percentiles 會比較容易看出整個 latency distribution。例如 p50 = 200 ms，代表 50% 的 requests 都在小於或等於 200 ms 的時間內被回覆，而 p50 本身其實就是 median。實務上通常也會看比較高的 percentile，例如 p95、p99、p99.9，因為這些高 percentile 更容易看出 tail latency，也就是少數特別慢的 requests。Arithmetic mean 不是沒有用，而是如果只看 arithmetic mean，很容易把 latency distribution 裡面的極端慢 request 隱藏掉。

高 percentile 在實務上特別重要，因為它能觀察最慢的那一小部分 requests。在需要同時呼叫多個 backend service 的系統中，只要其中一個 backend response 特別慢，最終使用者的 request 通常也必須等待這個最慢的 backend 完成。因此 backend 呼叫數量越多，碰到慢 request 的機率也會增加，使整體 request 的 tail latency 被進一步放大，這種現象稱為 tail latency amplification。

這種 percentile 的表示方式也很常出現在服務可靠性管理中。常見的三個概念是 SLI、SLO 和 SLA。SLI（Service Level Indicator）是實際量測服務表現的指標，例如 latency、availability、error rate；SLO（Service Level Objective）是針對這些指標設定的目標，例如要求 99.9% 的 requests 必須在 1 秒內完成；SLA（Service Level Agreement）則是服務提供者與客戶之間正式約定的服務水準，若未達成可能涉及補償或 service credit。

Percentiles 很適合用在 SLO 和 SLA，因為它不是只描述平均 response time，而是可以直接規定多少比例的 requests 必須在某個 latency threshold 內完成。例如 p50 ≤ 200 ms，代表至少 50% 的 requests 在 200 ms 內完成，而 p99.9 < 1 s 則代表 99.9% 的 requests 必須在 1 秒內完成。這樣除了可以描述一般使用者的 response time，也能直接限制 tail latency，避免只有少數 requests 極度緩慢，但整體 arithmetic mean 看起來仍然正常的情況。

# Maintainability

Maintainability 主要是在說：系統不只要能跑，還要讓未來的人容易維護、理解與修改。DDIA 將它拆成三個面向：

- Operability：讓 operations team 容易維持系統穩定運作，例如容易監控、除錯、部署、復原與處理故障。
- Simplicity：降低不必要的 system complexity，讓新的 engineers 容易理解整個系統。這裡的 simplicity 指的是系統內部設計簡單，不是 UI 簡單。
- Evolvability：讓系統未來容易修改與擴充，能隨 requirements 改變而調整，也常稱為 extensibility、modifiability 或 plasticity。

> Maintainability 的核心就是：讓系統容易操作、容易理解、也容易修改。好的系統設計不只考慮現在能不能運作，也要考慮未來維護與需求變更的成本。

# Scalability 與 Shared-Nothing Architecture

現代的大型 distributed systems 很常採用 shared-nothing architecture，每個 node 擁有自己的 CPU、memory 和 storage，並透過 network 互相協作。這類架構通常適合使用 scale-out，也就是增加更多 nodes 來提升系統容量，而不是單純依賴 scale-up 去購買更強大的單機。Scale-up 的架構通常比較簡單，但高階硬體成本會快速增加，而且最終會碰到單機硬體上限；scale-out 則具有較好的 elasticity，可以逐步增加 commodity machines，但同時也會引入 partitioning、replication、consistency、network failure 與 coordination 等 distributed system complexity。

> 不過，並不存在任何一種「magic scaling sauce」。沒有某種 architecture 或 technology 能夠不考慮 workload 就自動解決 scalability。良好的 system architecture 應該圍繞明確的 assumptions 建立，例如預期的 request rate、read/write ratio、dataset size、traffic pattern、latency requirements、availability requirements 以及 consistency requirements。只有先知道系統要面對什麼 workload，才能決定應該使用 scale-up、scale-out、partitioning、replication、caching 或其他設計。

# Polyglot Persistence

Polyglot persistence 是在同一個系統中，依不同資料與 workload 的需求，混合使用不同的資料儲存技術，例如 NoSQL 和 RDBMS，而不是所有資料都只使用同一種 database。

以銀行系統作為假設案例：某些交易相關資料可以先寫入 NoSQL，再每天透過 batch 將需要的資訊寫入 RDBMS，供報表、彙整或其他查詢使用。這是 polyglot persistence 的一種搭配方式，不代表銀行交易系統一定採用這個流程。

> 選擇儲存方式時，仍須依交易資料的一致性、原子性與耐久性需求判斷，不能只用 NoSQL／RDBMS 的分類來決定。每天 batch 同步也表示 RDBMS 中的資料可能尚未反映最新交易；若用於即時餘額或交易判斷，就必須另外處理這個時間差。跨資料庫同步還需要考慮重複寫入、漏資料與失敗重試。

---
blog: true
blog_title: 'Consensus Algorithms：共識、拜占庭容錯與 Raft 閱讀筆記'
blog_date: '2026-10-20'
blog_url: https://blog.walle4561.com/articles/posts/consensus-algorithms/
blog_tag: DSA
blog_topic: 演算法
---

# Consensus Algorithms

## TL;DR

- 共識讓多個節點在部分節點故障、訊息延遲等情況下，仍能對決策或操作順序達成一致，適用於分散式資料庫與區塊鏈等系統。
- Safety 關注不做出互相衝突的已決定結果；Liveness 關注在滿足協議條件時能持續前進。
- 閱讀演算法前，先看它容忍的是節點停止運作，還是包含說謊、傳送矛盾訊息的拜占庭故障。
- Raft 主要處理非拜占庭故障下的複寫日誌；PBFT 則處理拜占庭容錯，兩者採用不同的故障假設。

---

## 共識要解決什麼問題

多個節點各自收到請求，只靠轉送資料，並不能保證最後接受相同的操作順序。共識協議協調的是哪些操作被接受，以及它們的順序；區塊鏈的交易與區塊是其中一種應用情境。

在 Replicated State Machine（複寫狀態機）中，各節點依序執行相同的已決定命令；若初始狀態與執行規則相同，就能得到相同狀態。共識協議可用來維持複寫日誌的一致性。[Raft 原論文 §2](https://raft.github.io/raft.pdf)

### Safety 與 Liveness

| 性質 | 關注的問題 |
| --- | --- |
| Safety（安全性） | 不同正常節點不能對同一個決策位置，接受互相衝突的最終結果。 |
| Liveness（活性） | 當足夠的正常節點能通訊，並滿足協議所需的條件時，系統能繼續接受新的決策。 |

節點暫時持有不同的未提交日誌，不一定代表 Safety 已被破壞；需要區分暫存狀態與協議已經決定的結果。

---

## 故障模型：Crash Fault 與 Byzantine Fault

| 模型 | 故障行為 | 演算法例子 |
| --- | --- | --- |
| Crash Fault | 節點停止運作或重啟；正常運行時遵守協議。網路也可能延遲、丟失訊息或分割。 | Raft |
| Byzantine Fault | 節點可能任意偏離協議，例如向不同節點提供矛盾訊息。 | PBFT |

Raft 不以容忍任意惡意節點為目標。固定成員的基本模型中，$2f+1$ 個節點可在至多 $f$ 個節點停止運作時，保留多數決所需的節點數；要持續取得進展，還需要多數節點能穩定通訊。[Raft 原論文](https://raft.github.io/raft.pdf)

---

## 拜占庭將軍問題與 PBFT

**拜占庭將軍問題**討論部分節點說謊、傳送矛盾訊息或不回應時，誠實節點如何達成一致。需要兼顧 Safety（不接受互相衝突的結果）、Liveness（系統能繼續前進）與容錯。

在典型 PBFT 的假設下，容忍 $f$ 個拜占庭節點需要 $n\geq3f+1$。PBFT 的主要階段是 Pre-Prepare、Prepare、Commit，通訊量約為 $O(n^2)$。這不是所有共識模型都適用的通用門檻；開放公鏈還必須處理身份可任意建立的 Sybil 問題。

例如容忍 $f=1$ 個拜占庭節點時，典型 PBFT 至少需要 4 個副本。$3f+1$ 是該模型的副本數條件，不能簡化成「一般過半數投票」；協議還要處理惡意節點向不同人傳送不同訊息的情況。[PBFT 原論文](https://www.usenix.org/conference/osdi-99/practical-byzantine-fault-tolerance)

---

## Raft：待閱讀與整理

Raft 管理 Replicated Log，主要可依 Leader Election、Log Replication 與 Safety 三部分理解。以下先保留閱讀順序，細節與例子待閱讀時補上。

- [ ] Follower、Candidate、Leader 的角色與 Term。
- [ ] Leader Election：投票、Election Timeout 與 Heartbeat。
- [ ] Log Replication：AppendEntries、Log Index 與 Term。
- [ ] Commit 與 Apply 的差別，以及多數副本如何參與確認。
- [ ] Safety：Leader 更替後，如何保留已提交的紀錄。
- [ ] 節點故障、網路分割與復原的具體例子。
- [ ] 後續延伸：Snapshot／Log Compaction 與成員變更。

閱讀入口：[Raft 原論文（Extended Version）](https://raft.github.io/raft.pdf)，先從 §2 Replicated State Machines，再讀 §5 的選舉、日誌複寫與安全性。

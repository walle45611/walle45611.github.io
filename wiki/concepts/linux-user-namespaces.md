# Linux User Namespaces

## Current View

User namespace 將程序看到的 UID/GID 與 capabilities 限定在一個權限範圍內。它能讓同一程序在 namespace 內顯示為 root、在外部仍是一般 UID；能執行的特權操作取決於目標資源所屬的 user namespace，而非只看程序在自身 namespace 裡是否為 UID 0。

## Working Model

1. 以 `uid_map`／`gid_map` 描述內外身分對應；例如演講的單一映射將內部 UID 0 對應到外部 UID 1000。
2. 新建的非 user namespace 由建立者所處的 user namespace 擁有。對 UTS、network 等資源做特權操作時，要檢查程序在**擁有該 namespace 的 user namespace** 是否具有所需 capability。
3. 因此，建立新的 user 與 UTS namespace 可讓內部程序修改自己的 hostname；若 network namespace 仍是初始 instance，內部 root 仍不能因此管理主機網路介面。
4. 無特權容器與 sandbox 可用此機制降低對主機 root 的需求，但仍要配合其他 namespace、資源限制及安全政策，才能描述完整隔離邊界。

## Boundaries

這裡根據 2023 年演講逐字稿整理原理與示範。逐字稿未完整說明 UID/GID 映射寫入限制；主機政策、核心與發行版設定也會影響實際可用性。不要從「namespace 內 UID 0」推論「主機 root」，也不要把 user namespace 當作單獨完整的 sandbox。

## Related Concepts

- [operating-system-concurrency](./operating-system-concurrency.md)：共享狀態同步與資源等待是不同於 namespace 權限隔離的作業系統問題。
- [container-networking](./container-networking.md)：network namespace 隔離網路資源；user namespace 決定這些資源的權限歸屬。

## Sources

- [michael-kerrisk-understanding-linux-user-namespaces](../summaries/michael-kerrisk-understanding-linux-user-namespaces.md)

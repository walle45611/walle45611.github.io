# 作業系統並行與同步

## Current View

多個執行單位共享狀態時，結果可能受執行順序影響。作業系統筆記把這類問題分為競爭條件、臨界區同步與資源等待；各自需要不同的檢查條件。

## Working Model

1. **競爭條件**：先找出共享資料與臨界區；互斥之外，還要檢查 progress 與 bounded waiting。[同步摘要](../summaries/blog-race-condition-and-synchronization.md)
2. **同步工具**：TAS、CAS 等原子指令可作為鎖的基礎；自旋鎖會忙等，信號量與互斥鎖各有不同的使用情境。單有原子性不能推論公平等待。[鎖與信號量](../summaries/my-vault-research-mutex-locks-semaphores-f4379acfc.md)
3. **經典情境**：有界緩衝區同時要求互斥與滿／空時的等待協調；讀者寫者問題則使存取政策與飢餓風險具體化。[同步問題](../summaries/my-vault-research-famous-synchronization-problems-e91233fa5.md)
4. **死結**：四個必要條件同時存在時可能發生死結；預防、避免、檢測與恢復提供不同成本與保證程度。[死結管理](../summaries/blog-deadlock-management.md)

## Boundaries

同步安全性、等待公平性與死結處理是不同性質，不能用「加鎖」概括。這頁聚焦作業系統中的共享狀態與資源等待；namespace 的權限隔離另見 [Linux User Namespaces](./linux-user-namespaces.md)。

## Sources

- [競爭條件與同步](../summaries/blog-race-condition-and-synchronization.md)
- [Mutex Locks & Semaphores](../summaries/my-vault-research-mutex-locks-semaphores-f4379acfc.md)
- [Famous Synchronization Problems](../summaries/my-vault-research-famous-synchronization-problems-e91233fa5.md)
- [死結管理](../summaries/blog-deadlock-management.md)

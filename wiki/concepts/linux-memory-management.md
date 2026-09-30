# Linux 虛擬記憶體與位址映射

## Current View

虛擬位址是程式的存取視圖，實體 frame 是 backing；page tables 保存映射與權限，TLB 快取翻譯。2017 年講座提供入門模型，官方文件則用來釐清講座的 TLB miss 與 DMA 簡化。

## Working Model

1. 程式提供虛擬位址，MMU 查翻譯快取；未命中可走 page-table walk，不能直接等同 page fault。若頁不在記憶體或權限不允許，才進入 fault 處理。[Linux Page Tables](https://docs.kernel.org/mm/page_tables.html)
2. Lazy allocation、copy-on-write 與 swap-in 都可能讓 fault 成為正常工作流程；page fault 不必然是程式錯誤。同上官方文件區分這些原因。
3. 講座的共享記憶體例子是不同 process 映射到同一 frame；同一虛擬位址也可指向不同 frame。共享狀態的正確協調仍需 [[operating-system-concurrency]]。
4. CPU 虛擬、CPU 實體與 device bus／DMA 位址需分清；IOMMU 可能讓 DMA 位址不同於 CPU 實體位址，應透過 DMA API 建立映射。[Linux DMA API](https://docs.kernel.org/core-api/dma-api-howto.html)

## Concrete Example

Process A 的位址 `0x1000` 與 Process B 的 `0x9000` 若都映射到同一 frame，兩者可共享 backing。若 A 與 B 都使用 `0x1000`，但各自 page table 指向不同 frame，就仍是不同資料。判斷共享應看映射，不能只比較指標數值。

## Boundaries

講座的 3 GiB／1 GiB 分界與 4 KiB page 是示例；位址布局、page size 與 TLB refill 依架構而異。[[linux-user-namespaces]] 管理 UID／GID 與權限歸屬，不能用 namespace 的存在推定兩個 task 是否共享記憶體。DMA 與 `mlock()` 的完整操作條件需再補平台資料與實驗。

## Sources

- [[introduction-to-memory-management-in-linux]]：完整本地逐字稿摘要。
- [Linux Kernel — Page Tables](https://docs.kernel.org/mm/page_tables.html)：2026-09-30 查閱。
- [Linux Kernel — Dynamic DMA mapping Guide](https://docs.kernel.org/core-api/dma-api-howto.html)：2026-09-30 查閱。

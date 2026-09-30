# Linux 記憶體管理入門

- source: `raw/web-clipper/Introduction to Memory Management in Linux.md`
- source_sha256: `bdb379adc67f9833824ed437c94a3fae4aa7174f9135c35d855433a3c80edf37`
- source link: https://www.youtube.com/watch?v=7aONIVSXiJ8
- original title: Introduction to Memory Management in Linux
- author: Matt Porter（講者，Konsulko）；Alan Ott（原始投影片作者）
- publisher: The Linux Foundation
- published: 2017-04-05
- source_created: 2026-09-29
- ingested_at: 2026-09-30
- type: YouTube transcript summary

## Summary

講座從單一位址空間的隔離限制，介紹虛擬位址如何經 MMU 映射到實體記憶體，再依序討論 kernel logical、kernel virtual 與 user virtual 位址、page／page frame、共享記憶體、lazy allocation、page tables 與 swapping。主軸是「程式看到的連續位址，不代表背後實體 frame 也連續」。

## Key Claims

1. **4:34–12:18，MMU 與映射**：虛擬記憶體讓不同 process 有各自的位址視圖與存取權限；TLB 快取部分位址翻譯，不保存全部映射。
2. **12:39–23:25，核心位址**：講者用 32-bit、3 GiB user／1 GiB kernel 作為示例，區分直接映射的 kernel logical 空間與動態映射的 kernel virtual 空間；`kmalloc()` 與 `vmalloc()` 的實體連續性需求不同。
3. **23:43–33:43，process 與共享記憶體**：相同虛擬位址可在不同 process 指向不同 frame；不同虛擬位址也可映射到同一 frame，形成共享記憶體。共享後的同步問題仍需另外處理。
4. **34:10–36:57，延遲配置**：取得虛擬範圍與配置實體 backing 可分開；首次存取可能使核心處理 fault 並建立 backing。講者提醒第一次觸碰的延遲對測量與時間敏感工作有影響。
5. **37:20–43:37，page tables 與 swap**：page tables 保存比 TLB 更多的映射；swap-in 可把資料放回不同 frame，但維持程式使用的虛擬位址。
6. **43:56–47:57，user-space API**：介紹 `mmap()`、`brk()`／`sbrk()`、配置器與 stack growth 的關係；實際配置策略依 allocator、架構與設定而定。

## 來源限制與待釐清事項

- 本頁摘要的是 2017 年講座逐字稿，轉錄含名稱與函式拼字雜訊；32-bit 分界、4 KiB page 與 highmem 示例不能推廣為所有 Linux 平台的固定布局。
- 逐字稿數次把 TLB miss 描述為 page fault，講座也指出 TLB refill 方式依架構而異；這段簡化不能直接當作所有 CPU 的一般規則。
- 講座用連續性解釋 DMA，但不應把一般 CPU 虛擬指標直接當作 device DMA 位址；DMA API、IOMMU 與 buffer 管理仍需另查。
- 不將講者對 `mlock()` 的簡介視為所有模式都會立即預先配置的保證；本次未執行配置、fault、swap 或 DMA 實驗。
- 官方文件校正與跨來源整合放在 [[linux-memory-management]]；不把後續資料假裝成影片原話。

## Related Concepts

- [[linux-memory-management]]：MMU、TLB、page tables 與 fault 的層次。
- [[operating-system-concurrency]]：共享 frame 提供共享資料，但不自動提供互斥。
- [[linux-user-namespaces]]：UID／權限隔離與位址空間隔離的區別。

## Navigation

- [Web Clipper 來源](../archives/web-clipper.md)

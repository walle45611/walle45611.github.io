# Slurm 安裝與雙節點叢集實作

- source: `raw/my-vault/Note/Tech/Slurm 叢集架構與實作筆記.md`
- source_sha256: `56879d18ff2a50e10d3cb3d9f62dd42e0a12548e202412bc7819a2b8ed68525c`
- source_created: not specified
- ingested_at: 2026-10-09
- type: source summary

## 摘要與關鍵資訊

筆記以 slurm-ctl、node01、node02 與 hpcuser 建立三台 VM Lab，區分 controller 排程、compute node 執行與共享檔案存取。

- slurmctld 分配資源與管理作業；slurmd 接受執行要求，slurmstepd 管理 job step、I/O 與收尾。最小叢集不代表已部署 slurmdbd／帳務資料庫。
- Ubuntu 24.04 發行版套件路徑是補齊的重建指南，並非逐條驗證過的歷史安裝；正式部署仍依安裝版本文件核對。
- 三台須核對 UID/GID、時間同步與同一把 MUNGE key。slurmd -C 用來取得節點 CPU topology／RealMemory，不能照抄範例值。
- 本 Lab controller 位址為 192.168.139.46；明確指定 SlurmctldHost 的通訊 IP 可處理 controller 名稱解析問題，但不建立全系統 DNS。
- sinfo、squeue、scontrol 與雙節點 srun 分別觀察 node、job 與實際執行；CG 代表收尾，不能單靠狀態推定卡住原因。
- sbatch 輸出可能在執行節點的 WorkDir；同名目錄不等於共享儲存。應查看 BatchHost、StdOut、StdErr 與 findmnt，必要時取回結果。

## 限制與待補證據

共享儲存尚無掛載驗證；slurmd 重啟緩慢缺少定案日誌；Pending 練習尚未實測。最小設定使用 task/none 與 proctrack/linuxproc，不能視為已有 cgroup 強制資源隔離。此來源擴展系統運維案例，但叢集作業排程與即時系統 RMA 的保證模型不同。

## 相關概念

- [[operating-system-concurrency]]
- [[network-protocol-layers]]

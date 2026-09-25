# Windows Server Active Directory Operation Master（FSMO）角色整理

- source: `raw/my-vault/Note/Tech/Windows Server Active Directory Operation Master（FSMO）角色整理.md`
- source_sha256: `cb0f026dabb0d0086be40614f4081626c8930b129614ac118ae8485c7341e3e6`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 一、AD 架構中 FSMO 分布、🔍 查詢指令、二、Forest 層級 FSMO（樹系）、Schema Master、Domain Naming Master、三、Domain 層級 FSMO（網域）。

## Source Notes

- 處理 AD 架構結構定義的所有變更（如物件屬性、類別等）
- 必須在 Schema Master 上安裝 schmmgmt.dll 才能啟用 GUI 操作
- 當建立樹系中新的網域名稱時，此角色會參與控制是否允許

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)

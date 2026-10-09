# Cisco VTP (VLAN Trunking Protocol)

- source: `raw/my-vault/Note/Tech/Cisco VTP (VLAN Trunking Protocol).md`
- source_sha256: `32e6638a3b1284f53aa162bd19b82befa61404f42af62a1f79bbdad0c47e223e`
- source_reviewed: 2026-10-09
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 VTP working mode、VTP version、VTP advertisement、摘要通告 (summary advertisement)、子集通告 (subset advertisement)、用戶端要求。

## Source Notes

- Server mode ==(default)==
- 可以修改VLAN、創建、發送或轉發通告訊息，同步VLAN，將VLAN保存配置在NVRAM中
- 不能創建、更改和刪除VLAN，轉發通告，同步VLAN配置，不會將VLAN配置保存在NVRAM中

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)

## 2026-10-09 來源複核

本次依目前來源重新核對章節涵蓋範圍；既有摘要保留，以下列出原先短摘要未完整呈現的導航。設定、證明與範例的適用條件仍須依原文，不把章節涵蓋視為實作已驗證。

目前來源章節：VTP working mode、VTP version、VTP advertisement、摘要通告 (summary advertisement)、子集通告 (subset advertisement)、用戶端要求、VTP sync、VTP pruning、VTP刪除、VTP的訊息、case1 VTP version 2、case2 VTP version3。

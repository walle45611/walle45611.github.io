# Cisco 路由重分配 (route redistribute)

- source: `raw/my-vault/Note/Tech/Cisco 路由重分配 (route redistribute).md`
- source_sha256: `46fbc274026b63b767c97758dd2207d08684d3488d4d6a63cd126dec4a48b675`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 位什麼需要route redistribute、重分配需要考慮的問題、如何選擇最佳路由、種子度量值 (default metric)、redistrbute type、redistrbute直連路由。

## Source Notes

- 所有直連接口，物理狀態up protocol up的情況才會重發布
- 當我不想要在R1上連接switch的介面上設定路由，有可能是因為設定動態路由協定會發送hello太佔網路頻寬或是其他考慮不設定靜態路由，就可以使用這個方法
- 這樣connected ，當L3SW redistrbute所有connected 介面時會把所有路由注入到自己的routing table中

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)

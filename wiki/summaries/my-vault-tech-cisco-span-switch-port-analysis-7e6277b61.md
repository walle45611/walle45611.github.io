# Cisco SPAN (Switch port Analysis) 簡介和使用方式

- source: `raw/my-vault/Note/Tech/Cisco SPAN (Switch port Analysis) 簡介和使用方式.md`
- source_sha256: `380026ee18103b1a08ec21d752019845e29c264cbb249eead55b303a004493ec`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 為什麼需要、SPAN分類、Local SPAN、Local SPAN設定、Remote SPAN、show。

## Source Notes

- 假設下層交換設備有問題，可能需要網路分析器來收集資料，可以拓透過端口鏡射也就是SPAN，也就是說可以直接把某個switch port反射到某個主機進行分析，分析某個switch port傳出的數據。
- local SPAN : SPAN來源與目的地均作路在本地交換機。來源或是多個port。
- remote SPAN : SPAN來源與目的地在不同的交換機上。鏡射資料流是金過特殊用途的vlan傳送到目的地交換機進行複製資料。

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)

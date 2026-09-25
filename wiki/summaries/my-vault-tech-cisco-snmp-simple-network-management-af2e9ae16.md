# Cisco SNMP (Simple Network Management Protocol) 介紹和使用方式

- source: `raw/my-vault/Note/Tech/Cisco SNMP (Simple Network Management Protocol) 介紹和使用方式.md`
- source_sha256: `278b600710b6f35541b201ea9bc7e01710a0593c4f3f69d6ec3d8d5bbd34a01c`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 SNMP概述、SNMP三大組建、SNMP的行為、SNMP version、SNMP設定、SNMPv1。

## Source Notes

- 在傳統網路中是一個很常見的網管協定，可以監控網路設備也可以看自己想看的資料，如果出現了問題，會主動推送一個事件給管理員，是使用udp 161 port
- inform : 事件訊息發送到SNMP manager，而manager要回應agent確認收到
- 會發送一個(Object ID)OID，其中Oid裡面放的資料是一串節點ID，如何訪問樹例如 : 1-102-2-101這個就是在MIB中裡面的節點ID，這串OID會發送給agent，這樣就可以得到想得到的資料

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)

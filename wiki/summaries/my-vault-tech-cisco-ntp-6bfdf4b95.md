# Cisco NTP 設定

- source: `raw/my-vault/Note/Tech/Cisco NTP 設定.md`
- source_sha256: `520930a4e8c4c79851b68b5e75015bb8d10e26c51e7c2fb6376c4b3338d78a9f`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 設定時間及時區、NTP用戶端、伺服器、用戶端\伺服器設定、NTP 防護、SNTP (simplified network time protocol)。

## Source Notes

- 日光節約時間DST一種在夏季月份犧牲正常的日出時間，而將時間調快的做法。通常使用夏令時間的地區，會在接近春季開始的時候，將時間調快一小時，並在秋季調回正常時間。實際上，夏令時間會造成在春季轉換當日的睡眠時間減少一小時，而在秋季轉換當日則會多出一小時的睡眠時間。
- 在設定NTP前應該要先將此設備較時，和把時區設定相同，這樣效果較佳，假設設定的時間為8.25 p.m.要先設定時區，甚至要告訴其DST，才啟用NTP
- 第一條指令clock timezone EST -5，EST這個參數可以隨意打，但是還是希望可以選用有意的值，最後-5是指比UTC(國際標準時間)前5個小時

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)

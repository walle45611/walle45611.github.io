# Cisco Syslog 設定

- source: `raw/my-vault/Note/Tech/Cisco Syslog 設定.md`
- source_sha256: `cdd74b799ec507a781975cb7226a42b519ceac384108e4a619f9b56eaa5fe62f`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 概念、及時發送訊息給目前使用者、儲存log以便事後檢視、Log訊息格式、Log事件等級。

## Source Notes

- 對於IT人員，log訊息極為重要，因為可以查看在一個時間發生的事件，但是訊息很多有重要的也有不太重要的訊息，所以做好log訊息的整理是一件很重要的事情，還有當裝置OS發生event的時候亦要怎麼通知管理人員也是相當重要的議題。
- 根據預設，IOS會將所有等級的log訊息顯示給console port的使用者看。這是由於預設的logging console global指令所產生的，所以可以透過no logging console關閉。
- 對於其他介面的聯接方式例如ssh或telnet使用者來說，需要使用logging monitor來告訴IOS將log message告訴所有登入的使用者。這只指令還不足也讓使用者看到log訊息還必須下terminal monitor指令，來告知IOS終端會想要接收到log訊息

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)

# Linux Nginx Web Server 設定指南

- source: `raw/my-vault/Note/Tech/Linux Nginx Web Server 設定指南.md`
- source_sha256: `c7516c3af47dd320641f5231d5502ebf8f7f530cf429144a86de4c295dc699be`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 安裝nginx、啟動nginx、nginx主要文件、網頁預設放的位置、log訊息、nginx處理請求流程。

## Source Notes

- mine.types是server傳送給用戶端對於檔案的處理方式要顯示還是下載之類的。
- example:今天有個exe檔在server上用戶端請求這個檔案server回應的時候不是查看附檔名而是去查看mine.types裡面的屬性去做相對應的選擇。
- 這是有沒有啟用sendfile的時候的topology，這樣看起來是nginx先複製一份html文件和test.mp4到nginx的內存裡面再發送給網卡緩存這樣，這樣多複製了一份檔案所以這樣比較慢。

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)

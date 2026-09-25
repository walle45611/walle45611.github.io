# 日期與視窗函數

- source: `raw/my-vault/Note/Tech/SQL/日期與視窗函數.md`
- source_sha256: `6bd9c692cb9ac944580cdee3dec0941a4a550f5640d509591fc91f7aa19279ff`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 #197 Rising Temperature、草稿中的錯誤、LAG() 參考寫法、Self join 參考寫法、易錯點。

## Source Notes

- 題目要比較今天與前一天的溫度。你想到用 LAG() 取得上一列，再檢查日期是否剛好差一天。
- 你曾把欄位清單直接放在 FROM (...) 中，少了內層完整的 SELECT ... FROM Weather；另一版把條件連接詞寫成 ADD。衍生表也需要別名。
- LAG() 取得排序後上一列，但上一列不一定是昨天；用 DATEDIFF(...)=1 才能確認日期相差一天。

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)

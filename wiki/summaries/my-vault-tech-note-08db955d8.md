# 分組與資料修改

- source: `raw/my-vault/Note/Tech/SQL/分組與資料修改.md`
- source_sha256: `b0d9b5417819216ea4061838bcb3b28b3656af63fb6411eb13f41e1a8545f35f`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 #182 Duplicate Emails、#511 Game Play Analysis I、#196 Delete Duplicate Emails、易錯點。

## Source Notes

- GROUP BY 將相同 email 分組，HAVING 再篩選筆數大於 1 的群組。分組不代表輸出會排序；要排序需另用 ORDER BY。
- 這是 #511 的 self join 寫法的另一種解法，見 Note/Tech/SQL/JOIN 與反連接。
- 對話中比較過 GROUP BY + MIN() 和 self join 的單次 LeetCode runtime。執行計畫、索引和測量波動都會影響時間；要比較時查看 EXPLAIN / EXPLAIN ANALYZE 並多次測量，不要只依一次時間下結論。

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)

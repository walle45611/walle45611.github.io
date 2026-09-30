# CASE WHEN 條件判斷

- source: `raw/my-vault/Note/Tech/SQL/CASE WHEN 條件判斷.md`
- source_sha256: `475045238393578b2323358db0d1a0f0be7fb30d0b284153d6a121d73733e05d`
- source_reviewed: 2026-09-30
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 #610 Triangle Judgement、易錯點。

## Source Notes

- 你記得三角形條件是任意兩邊和都要大於第三邊。三個條件同時成立才是三角形，因此用 AND。
- CASE WHEN 可以想成 SQL 的 if / else。題目要每列都保留並新增判斷結果，所以把 CASE 放在 SELECT；WHERE 則用來篩選要留下的列。
- 語法是 CASE WHEN 條件 THEN 結果 ELSE 結果 END，不要寫成 CASE WHERE。

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)

## 相關概念

- [[database-and-sql]]：依保留列、配對、聚合與分類需求選擇語句。

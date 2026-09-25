# JOIN 與反連接

- source: `raw/my-vault/Note/Tech/SQL/JOIN 與反連接.md`
- source_sha256: `b4689d2118c286765dc616a973226194ed2e6aedef0c46ec055f5b0f751e286e`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 保留需要的資料列、#175 Combine Two Tables、#577 Employee Bonus、Self join：同一張表扮演兩個角色、#181 Employees Earning More Than Their Managers、#511 Game Play Analysis I。

## Source Notes

- 題目要保留每個 Person；沒有地址的人仍要出現在結果中，地址欄位顯示 NULL，所以 Person 放在左邊
- LEFT JOIN 保留左表所有列；INNER JOIN 只留下配對成功的列。
- 找 bonus 小於 1000 或沒有 bonus 紀錄的員工

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)

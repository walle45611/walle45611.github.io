# Matrix-chain Multiplication

- source: `raw/my-vault/Note/Research/Matrix-chain Multiplication.md`
- source_sha256: `96185a8d4f9762410239a0d66f11d979be4b2b2f29d105433cf232fe787fa879`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 問題定義 (Problem)、為什麼這很重要？、為什麼不能用暴力法 (Brute Force)？、動態規劃 (DP) 解法、步驟 1：分析最優解的結構 (Optimal Substructure)、步驟 2：建立遞迴解 (Recursive Solution)。

## Source Notes

- 給定一串矩陣鏈 $\langle A_1, A_2, \dots, A_n \rangle$，找出一個最佳的「加括號方式」（parenthesization），使得計算總乘積 $A_1A_2\dots A_n$ 所需的「純量乘法次數」最少。
- 成本： $A_{p \times q} \times B_{q \times r}$ 的成本是 $p \times q \times r$ 次純量乘法。
- 範例： $A_1 (10 \times 100)$, $A_2 (100 \times 5)$, $A_3 (5 \times 50)$

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

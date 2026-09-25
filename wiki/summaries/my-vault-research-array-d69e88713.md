# Array 記憶體位址計算全攻略

- source: `raw/my-vault/Note/Research/Array 記憶體位址計算全攻略.md`
- source_sha256: `9125b7de8ffb054119ef9fde4f549ed151657200a840edff367b7cf657a8aac7`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 基礎觀念：一維陣列 (1-Dim Array)、進階觀念：二維陣列 (2-Dim Array)、通用公式、四大常考題型攻略 (含範例詳解)、🔥 型一：給定所有參數，求位址、🔥 型二：給 2 個元素位址，判斷 Row/Col Major。

## Source Notes

- 公式：$Loc(A[i]) = L_0 + (i - l) \times d$
- 宣告：$A[l_1 \ldots u_1, \ l_2 \ldots u_2]$
- 列數 (Rows)：$R = u_1 - l_1 + 1$

## Navigation

- [資料結構與演算法](../concepts/data-structures-and-algorithms.md)
- [回到 Research 歸檔](<../archives/my-vault-research.md>)

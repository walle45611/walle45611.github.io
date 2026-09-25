# 問題背景

- source: `raw/my-vault/Note/Research/OBST (Optimal Binary Search Tree).md`
- source_sha256: `366f9c0eead88ec2f68544c16e9eadb3d9eaf7589f22ebb1a800baaaf4a633a3`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 成本模型、期望查詢成本（CLRS 公式 14.11）、圖 14.9 的資料（範例）、範例重點、Step 1. Optimal Substructure、Step 2. Recursive Subproblem。

## Source Notes

- 你要做英→拉脫維亞字典查詢。每個英文單字出現頻率不同；常見字應該更接近根，以降低平均查詢步數。有些查詢不在字典內，也要計入「失敗查詢」的機率。
- 排序後的 n 個鍵：$K=\langle k_1<k_2<\cdots<k_n\rangle$。
- 成功查詢機率：對每個鍵 $k_i$ 有 $p_i$。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

# K-means Clustering

- source: `raw/my-vault/Note/Research/K-means Clustering.md`
- source_sha256: `11c3d35a1a603e073d9ea4a760ddb6bb127149ffed8dfbac48d993e475232707`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 核心目標：K-means Problem、核心規則：Nearest-Center Rule (最近中心法則)、求解演算法：Lloyd's Procedure、關鍵數學性質 (演算法的基礎)、應用範例、計算方式。

## Source Notes

- 給定 $n$ 個 $d$ 維的資料點集合 $S$（$S \subset \mathbb{R}^d$），以及一個正整數 $k$，我們希望找到 $k$ 個「中心點」(centers) $C$，並將 $S$ 中的點分成 $k$ 個叢集 (clusters) $S^{(1)}, \ldots, S^{(k)}$，使得每個點到其所屬叢集之中心點的「平方距離總和」最小。
- 目標函式 (Cost Function) $f(S,C)$
- 我們的目標是找到一組 centers $C = \langle \mathbf{c}^{(1)}, \ldots, \mathbf{c}^{(k)} \rangle$ 來最小化 $f(S,C)$

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

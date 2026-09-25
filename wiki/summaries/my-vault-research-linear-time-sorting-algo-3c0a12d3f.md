# Counting Sort

- source: `raw/my-vault/Note/Research/Linear-time sorting algo.md`
- source_sha256: `aca5034cf5ce59c6785b3fd9e76eca07a155060e4c8dec95cb14140ddfe731c5`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 Counting Sort 分析、Proof、核心概念、操作方式、LSD Radix Sort — 虛擬碼（Pseudocode）、Radix Sort 分析。

## Source Notes

- 符號 $A[1..n]$ 輸入，每個鍵在 $0..k$; $B[1..n]$ 輸出; $C[0..k]$ 計數。
- 初始化：對 $i=0..k$，令 $C[i]=0$。
- 計數：掃描 $A$，對 $x=A[j]$ 執行 $C[x] \leftarrow C[x]+1$。完成後 $C[i]$ 為「值等於 $i$ 的個數」。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

# 名詞解釋和定義

- source: `raw/my-vault/Note/Research/Hashing.md`
- source_sha256: `b0ed6d2bd4cb4042a4f7f136a8a97e80fd08aaeb6fc95fabf78ffd6d54485adb`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 平方值取中間位數（Middle square）、除法（Mod 運算）、折疊相加（Folding Addition）、Universal hash function、什麼是雜湊族 (Hash Family)?、萬用雜湊公式 $h_{a,b}(K)$ 中的變數。

## Source Notes

- 模型 表分成 $b$ 個 bucket，每個 bucket 有 s 個 slots；第 $i$ 個 bucket 目前放入筆數 $n_i$。
- Collision（碰撞）：也就是 $\text{hash}(x)=\text{hash}(y)$存在某個 bucket 內至少兩筆。 $\exists i: n_i\ge 2$
- Overflow（溢位）：也就是資料剛好映射到這個 bucket，但是剛好這個 bucket 裡面的 slots 都滿了就是 Overflow。 $\exists i: n_i> s$

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

# Single-Source Shortest Paths 演算法總覽

- source: `raw/my-vault/Note/Research/Single-Source Shortest Paths Problem.md`
- source_sha256: `7570dc23420956391c696cbb17a050d937aa72ba746cdcebc652356a15ce30a3`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 範例、資料結構版本 adj matrix、演算法版本、複雜度分析、Dijkstra's Algorithm 不能用在含負權邊的圖、定義。

## Source Notes

- 若經由 $u$ 可以改進 $v$ 的估計，就更新距離與前驅
- 上界：$v.d \ge \delta(s,v)$，一旦等於就不再變。
- 三角：$\delta(s,v)\le \delta(s,u)+w(u,v)$。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

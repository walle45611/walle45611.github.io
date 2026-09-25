# Gradient Accumulation Across a Batch

- source: `raw/my-vault/Note/Research/ML_DL_Obsidian_Notes/03 - Gradient Accumulation Across a Batch.md`
- source_sha256: `aad3b0747653bf4ce5334130bc45da7ea9a56217eaa452e8e0eff4fc8dc3d217`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 訓練流程、計算例子、Related。

## Source Notes

- 如果 batch 裡有 $B$ 筆資料，average loss
- 所以每一筆資料都對 parameter gradient 有貢獻，batch gradient 是這些 contribution 的平均（或某些實作採 sum，再由其他地方做 scaling）。
- 這與手寫稿「把 $N$ 筆資料分 batch，每個 batch 更新 parameters」是同一件事。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

# RMSProp

- source: `raw/my-vault/Note/Research/ML_DL_Obsidian_Notes/02 - RMSProp.md`
- source_sha256: `afa1f4bb8d9c7c10326a22ee6fbfe92653456ecfeb83e7ebaba9b117d18c92a5`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 Root Mean Square 的直覺、和手寫稿的關係、計算例子、Related。

## Source Notes

- RMSProp 的核心是：每個 parameter 都用自己的 recent gradient scale 來調整有效 learning rate。
- g_{t,i}=\frac{\partial\mathcal L}{\partial\theta_i}
- 先維護 squared gradient 的 exponential moving average

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

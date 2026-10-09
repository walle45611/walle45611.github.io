# RMSProp

- source: `raw/my-vault/Note/Research/02 - RMSProp.md`
- source_sha256: `8a534be5cb58a9c7e54c198df44c0d9e5c368ff73ae959346aa57bd228be0448`
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

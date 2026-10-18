# Feature Normalization

- source: `raw/my-vault/Note/Research/05 - Feature Normalization.md`
- source_sha256: `0bc1abca7284e6ddf92d4c10817d6d814301deb0997efd590d7d8dc2cb30e220`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 按特徵計算、計算例子、與 BatchNorm 的差別、Related、手寫原稿。

## Source Notes

- Feature normalization 是調整輸入特徵尺度。手寫稿使用的是 z-score 標準化，不是將每筆向量的長度變成 1，也不是加入懲罰項的 regularization。
- 令 $x_i^{(n)}$ 表示第 $n$ 筆樣本的第 $i$ 個特徵，上標是樣本編號，不是次方。對訓練集的 $N$ 筆資料
- 同一特徵共用一組 $\mu_i,\sigma_i$，不同特徵各算各的。若寫成向量 $(\mathbf x-\boldsymbol\mu)/\boldsymbol\sigma$，除法是逐元素運算。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

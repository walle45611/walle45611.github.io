# Layer Normalization

- source: `raw/my-vault/Note/Research/Layer Normalization.md`
- source_sha256: `be4abf201f68651b27f9134df93ab8dacd37b92e05cc2765efa785fc165e2872`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

Layer Normalization 以單一樣本同一層的特徵計算平均與變異數，降低正規化對 mini-batch 大小的依賴。

## Source Notes

- 正規化後加入可學習的縮放與偏移；訓練與推論使用相同計算，也能在循環網路各時間步分別套用。
- 原文展示隱藏狀態更穩定及訓練加速。閱讀時應分清楚 LayerNorm 與 BatchNorm 各自沿哪些維度計算統計量。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

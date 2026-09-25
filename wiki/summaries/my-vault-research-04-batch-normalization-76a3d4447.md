# Batch Normalization

- source: `raw/my-vault/Note/Research/ML_DL_Obsidian_Notes/04 - Batch Normalization.md`
- source_sha256: `68f698518c9d6354638eeaf50b1148a6ca37a8cb6aa1b294b2fd04ab5277743d`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 訓練時的運算、三筆資料的完整計算、推論時與 moving average、Related、手寫原稿。

## Source Notes

- BatchNorm（BN）在模型中對特徵標準化，再以可學習的 $\gamma,\beta$ 調整尺度與位置。同一批樣本使用同一組模型權重；對同一個神經元輸出的數值跨樣本計算統計量，不是每筆資料各自換一組權重。
- 對同一特徵的 $B$ 個值 $z^{(1)},\ldots,z^{(B)}$
- u^{(n)}=\gamma\tilde z^{(n)}+\beta

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

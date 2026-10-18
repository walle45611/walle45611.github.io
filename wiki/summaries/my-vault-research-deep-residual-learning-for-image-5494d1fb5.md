# Deep Residual Learning for Image Recognition

- source: `raw/my-vault/Note/Research/@residuallearning2015.md`
- source_sha256: `3da04b73d47d28891c04dfd659ec9874c5d9a38dd6c6e0794869bdbc6fec2ecd`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

ResNet 針對深層網路的退化問題：增加層數後，連訓練誤差都可能變高，並非單純過擬合。

## Source Notes

- 以 shortcut connection 讓區塊學習殘差 F(x)，輸出為 F(x)+x，使深層網路較容易接近恆等映射並進行最佳化。
- 原文以影像辨識等實驗展示深層殘差網路的效果。閱讀重點是殘差學習如何改善最佳化，以及 shortcut 在維度改變時如何處理。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

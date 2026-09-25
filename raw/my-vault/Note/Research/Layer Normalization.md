# Layer Normalization

[[Assets/Note/Research/Papers/Layer Normalization/Layer Normalization.pdf|論文 PDF]]

## 初讀摘要

- Layer Normalization 以單一樣本同一層的特徵計算平均與變異數，降低正規化對 mini-batch 大小的依賴。
- 正規化後加入可學習的縮放與偏移；訓練與推論使用相同計算，也能在循環網路各時間步分別套用。
- 原文展示隱藏狀態更穩定及訓練加速。閱讀時應分清楚 LayerNorm 與 BatchNorm 各自沿哪些維度計算統計量。

摘要依據：PDF 摘要與導論。


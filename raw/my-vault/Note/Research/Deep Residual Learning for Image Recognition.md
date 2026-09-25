# Deep Residual Learning for Image Recognition

[[Assets/Note/Research/Papers/Deep Residual Learning for Image Recognition/Deep Residual Learning for Image Recognition.pdf|論文 PDF]]

## 初讀摘要

- ResNet 針對深層網路的退化問題：增加層數後，連訓練誤差都可能變高，並非單純過擬合。
- 以 shortcut connection 讓區塊學習殘差 F(x)，輸出為 F(x)+x，使深層網路較容易接近恆等映射並進行最佳化。
- 原文以影像辨識等實驗展示深層殘差網路的效果。閱讀重點是殘差學習如何改善最佳化，以及 shortcut 在維度改變時如何處理。

摘要依據：PDF 摘要與導論。


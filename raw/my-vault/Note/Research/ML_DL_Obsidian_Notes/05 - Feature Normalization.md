# Feature Normalization

Feature normalization 是調整輸入特徵尺度。手寫稿使用的是 **z-score 標準化**，不是將每筆向量的長度變成 1，也不是加入懲罰項的 regularization。

## 按特徵計算

令 $x_i^{(n)}$ 表示第 $n$ 筆樣本的第 $i$ 個特徵，上標是樣本編號，不是次方。對訓練集的 $N$ 筆資料：

$$
\mu_i=\frac1N\sum_{n=1}^N x_i^{(n)},\qquad
\sigma_i^2=\frac1N\sum_{n=1}^N(x_i^{(n)}-\mu_i)^2
$$

$$
\tilde x_i^{(n)}=\frac{x_i^{(n)}-\mu_i}{\sigma_i}
$$

同一特徵共用一組 $\mu_i,\sigma_i$，不同特徵各算各的。若寫成向量 $(\mathbf x-\boldsymbol\mu)/\boldsymbol\sigma$，除法是逐元素運算。

只用訓練集估計統計量，再套用到 validation、test 和新資料，避免資料洩漏。交叉驗證時，每一折都只用該折訓練部分估計。常數特徵的 $\sigma_i=0$，可移除或將除數設為 1，使中心化後結果為 0。

## 計算例子

三筆資料的某項特徵為 $(2,4,6)$：

$$
\mu=4,\qquad\sigma^2=\frac{(-2)^2+0^2+2^2}{3}=\frac83,
\qquad\sigma\approx1.6330
$$

$$
\tilde{\mathbf x}=\frac{(2,4,6)-4}{1.6330}
\approx(-1.2247,0,1.2247)
$$

標準化後平均為 0，使用同一個除以 $N$ 的定義計算，變異數為 1。測試資料若為 $8$，仍使用訓練集的 $4$ 與 $1.6330$：$\tilde x\approx2.4495$，不能把測試資料重新湊成一批算自己的平均。

## 與 BatchNorm 的差別

輸入標準化通常先固定訓練集統計量；BatchNorm 是模型內的運算，在訓練時依 mini-batch 計算中間特徵的統計量，並學習縮放與平移。兩者可以同時使用。

## Related

[[Note/Research/ML_DL_Obsidian_Notes/04 - Batch Normalization|04 - Batch Normalization]]

## 手寫原稿

> [!note]- 照片 1：手寫原稿
> ![[Assets/Note/Research/ML_DL_Obsidian_Notes/Photo 1.jpg|700]]

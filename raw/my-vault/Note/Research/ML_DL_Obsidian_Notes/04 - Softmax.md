# Softmax

神經網路最後一層常先輸出 logits：

$$
\mathbf z=W^T\mathbf h+\mathbf b
$$

這裡 $\mathbf h$ 可以理解成 hidden layer output / learned features。

Softmax 把 logits 轉成 probability distribution：

$$
\boxed{
\hat y_i=\frac{e^{z_i}}{\sum_j e^{z_j}}
}
$$

因此：

$$
0<\hat y_i<1,\qquad \sum_i\hat y_i=1
$$

## 手寫稿例子重算

若：

$$
\mathbf z=
\begin{bmatrix}
3.2\\
1.1\\
-0.2
\end{bmatrix}
$$

則：

$$
\operatorname{softmax}(\mathbf z)
\approx
\begin{bmatrix}
0.865\\
0.106\\
0.029
\end{bmatrix}
$$

最大的 logit 得到最大的 predicted probability。

## Numerical Stability

實作時通常會減去最大 logit：

$$
\operatorname{softmax}(z_i)
=
\frac{e^{z_i-z_{\max}}}{\sum_j e^{z_j-z_{\max}}}
$$

數學結果相同，但可降低 overflow 風險。

## 計算例子

沿用上面的 logits，先減去最大值 $3.2$：

$$
\mathbf z-3.2=(0,-2.1,-3.4),\qquad
(e^0,e^{-2.1},e^{-3.4})\approx(1,0.12246,0.03337)
$$

分母為 $1+0.12246+0.03337=1.15583$，因此：

$$
\hat{\mathbf y}\approx(0.8652,0.1059,0.0289)
$$

若正確類別是第一類，cross-entropy 為 $-\ln0.8652\approx0.1448$。對 logits 的梯度是 $\hat{\mathbf y}-\mathbf y\approx(-0.1348,0.1059,0.0289)$，其中 $\mathbf y=(1,0,0)$。

## Related

- [[Note/Research/ML_DL_Obsidian_Notes/02 - Likelihood NLL and Cross-Entropy|02 - Likelihood NLL and Cross-Entropy]]
- [[Note/Research/ML_DL_Obsidian_Notes/01 - Neuron Features and Parameters|Neuron and Parameters]]

# MAE and MSE

模型的總 loss 常寫成每筆資料 error 的平均：

$$
\boxed{
\mathcal L(\theta)=\frac1N\sum_{n=1}^{N}e_n
}
$$

## MAE

Mean Absolute Error：

$$
\boxed{
\operatorname{MAE}=\frac1N\sum_{n=1}^{N}|y_n-\hat y_n|
}
$$

特性：

- 對 outlier 比 MSE 不敏感
- $|x|$ 在 0 點不可微，但可使用 subgradient

## MSE

Mean Squared Error：

$$
\boxed{
\operatorname{MSE}=\frac1N\sum_{n=1}^{N}(y_n-\hat y_n)^2
}
$$

特性：

- 大 error 被平方，因此懲罰更重
- 對 outlier 較敏感
- derivative 平滑，常用於 regression

## Loss 與參數

真正做 optimization 時，loss 是 parameters 的 function：

$$
\mathcal L=\mathcal L(\theta)
$$

我們不是直接「調 loss」，而是調 $\theta$ 使 loss 下降：

$$
\theta^*=\arg\min_\theta \mathcal L(\theta)
$$

## 計算例子

取真值 $\mathbf y=(1,2,3)$、預測 $\hat{\mathbf y}=(2,2,5)$，誤差 $\hat{\mathbf y}-\mathbf y=(1,0,2)$：

$$
\operatorname{MAE}=\frac{1+0+2}{3}=1,
\qquad
\operatorname{MSE}=\frac{1^2+0^2+2^2}{3}=\frac53\approx1.6667
$$

MSE 對每個預測值的偏導數為：

$$
\frac{\partial\operatorname{MSE}}{\partial\hat y_i}
=\frac23(\hat y_i-y_i),
\qquad
\nabla_{\hat{\mathbf y}}\operatorname{MSE}=(2/3,0,4/3)
$$

這是對預測值的梯度；要得到權重梯度，還要乘上預測對權重的導數。

## Related

- [[Note/Research/ML_DL_Obsidian_Notes/01 - Gradient Descent and Learning Rate|Gradient Descent]]
- [[Note/Research/ML_DL_Obsidian_Notes/02 - Likelihood NLL and Cross-Entropy|02 - Likelihood NLL and Cross-Entropy]]

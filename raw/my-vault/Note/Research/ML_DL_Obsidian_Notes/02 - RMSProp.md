# RMSProp

RMSProp 的核心是：每個 parameter 都用自己的 recent gradient scale 來調整有效 learning rate。

對 parameter $\theta_i$：

$$
g_{t,i}=\frac{\partial\mathcal L}{\partial\theta_i}
$$

先維護 squared gradient 的 exponential moving average：

$$
\boxed{
s_{t,i}
=\rho s_{t-1,i}+(1-\rho)g_{t,i}^2
}
$$

更新：

$$
\boxed{
\theta_{t+1,i}
=
\theta_{t,i}
-
\eta\frac{g_{t,i}}{\sqrt{s_{t,i}}+\epsilon}
}
$$

## Root Mean Square 的直覺

如果最近某個 parameter 的 gradient 一直很大：

$$
\sqrt{s_{t,i}}\uparrow
$$

所以有效 step：

$$
\frac{\eta}{\sqrt{s_{t,i}}+\epsilon}
$$

會變小。

反之，如果 gradient scale 很小，step 相對會放大。

## 和手寫稿的關係

手寫稿將 $\sigma_i$ 寫成近期 gradient magnitude 的 RMS。標準 RMSProp 通常不是對「所有歷史」做簡單平均，而是使用 exponential moving average，讓較新的 gradient 權重較高。

## 計算例子

設 $\rho=0.9$、$\eta=0.1$、$s_0=0$、$\theta_0=1$，給定前兩步梯度皆為 $2$。本例分母不為零，數值近似忽略極小的 $\epsilon$：

$$
s_1=0.9(0)+0.1(2^2)=0.4,
\qquad\theta_1=1-0.1\frac{2}{\sqrt{0.4}}\approx0.6838
$$

$$
s_2=0.9(0.4)+0.1(2^2)=0.76,
\qquad\theta_2=0.6838-0.1\frac{2}{\sqrt{0.76}}\approx0.4544
$$

梯度相同，但平方梯度累積後，第二步移動量約 $0.2294$，小於第一步的 $0.3162$。

## Related

- [[Note/Research/ML_DL_Obsidian_Notes/01 - Momentum|01 - Momentum]]
- [[Note/Research/ML_DL_Obsidian_Notes/03 - Adam|03 - Adam]]
- [[Note/Research/ML_DL_Obsidian_Notes/05 - AdaGrad and Gradient Scaling|05 - AdaGrad and Gradient Scaling]]

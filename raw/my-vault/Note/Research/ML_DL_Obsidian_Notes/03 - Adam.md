# Adam

可以先用一句話記：

> Adam ≈ Momentum 的 first moment + RMSProp 類似的 second moment adaptive scaling。

但標準 Adam 還包含 bias correction。

## First moment

$$
\boxed{
m_t=\beta_1m_{t-1}+(1-\beta_1)g_t
}
$$

## Second moment

$$
\boxed{
v_t=\beta_2v_{t-1}+(1-\beta_2)g_t^2
}
$$

平方為 element-wise square。

## Bias correction

初始 $m_0=v_0=0$，因此早期 moving average 會偏向 0：

$$
\hat m_t=\frac{m_t}{1-\beta_1^t}
$$

$$
\hat v_t=\frac{v_t}{1-\beta_2^t}
$$

## Update

$$
\boxed{
\theta_{t+1}
=
\theta_t
-
\eta\frac{\hat m_t}{\sqrt{\hat v_t}+\epsilon}
}
$$

常見 default（概念上記即可）：

$$
\beta_1=0.9,\qquad \beta_2=0.999
$$

## 計算例子

設 $g_1=2$、$\beta_1=0.9$、$\beta_2=0.999$、$\eta=0.1$，初始 $m_0=v_0=0$、$\theta_0=1$：

$$
m_1=0.1(2)=0.2,\qquad v_1=0.001(2^2)=0.004
$$

Bias correction 給出：

$$
\hat m_1=\frac{0.2}{1-0.9}=2,
\qquad\hat v_1=\frac{0.004}{1-0.999}=4
$$

忽略極小的 $\epsilon$，第一步更新為：

$$
\theta_1=1-0.1\frac{2}{\sqrt4}=0.9
$$

若漏掉 bias correction，同一組數字會得到 $1-0.1(0.2/\sqrt{0.004})\approx0.6838$，可見修正會影響初期步長。

## Related

- [[Note/Research/ML_DL_Obsidian_Notes/01 - Momentum|01 - Momentum]]
- [[Note/Research/ML_DL_Obsidian_Notes/02 - RMSProp|02 - RMSProp]]
- [[Note/Research/ML_DL_Obsidian_Notes/04 - Optimizer Comparison|04 - Optimizer Comparison]]

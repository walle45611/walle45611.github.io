# Momentum

Vanilla gradient descent：

$$
\theta_{t+1}=\theta_t-\eta g_t
$$

Momentum 會保留前幾步的移動方向。

一種常見寫法：

$$
\boxed{
v_t=\beta v_{t-1}+g_t
}
$$

$$
\boxed{
\theta_{t+1}=\theta_t-\eta v_t
}
$$

另一種與手寫稿接近的 sign convention：

$$
\Delta\theta_t
=
\beta\Delta\theta_{t-1}-\eta g_t
$$

$$
\theta_{t+1}=\theta_t+\Delta\theta_t
$$

兩種寫法核心相同：**累積一致方向的速度，抑制來回震盪。**

![](https://commons.wikimedia.org/wiki/Special:FilePath/Gradient_descent_with_momentum.svg)

Source: https://commons.wikimedia.org/wiki/File:Gradient_descent_with_momentum.svg

## 直覺

在狹長 valley 中：

- steep direction 的 gradient 常正負交替 → momentum 可減少 oscillation
- 長期一致方向 → velocity 累積 → 前進更快

## 計算例子

採用本文 $v_t=\beta v_{t-1}+g_t$ 的寫法，設 $\beta=0.9$、$\eta=0.1$、初始 $v_0=0$、$\theta_0=1$。給定前兩步梯度皆為 $2$，每步使用新速度更新參數：

$$
v_1=0.9(0)+2=2,\qquad\theta_1=1-0.1(2)=0.8
$$

$$
v_2=0.9(2)+2=3.8,\qquad\theta_2=0.8-0.1(3.8)=0.42
$$

第二步移動 $0.38$，比第一步的 $0.2$ 大，展示同方向梯度的累積。這裡給定梯度序列以示範更新運算；實際梯度須在更新後的參數重新計算。

## Related

- [[Note/Research/ML_DL_Obsidian_Notes/02 - RMSProp|02 - RMSProp]]
- [[Note/Research/ML_DL_Obsidian_Notes/03 - Adam|03 - Adam]]
- [[Note/Research/ML_DL_Obsidian_Notes/01 - Gradient Descent and Learning Rate|Gradient Descent]]

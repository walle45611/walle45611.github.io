# Local Minima, Saddle Points and Critical Points

## Global minimum

$$
\theta^*=\arg\min_\theta \mathcal L(\theta)
$$

表示整個 parameter space 裡 loss 最小的位置。

## Local minimum

只要求在某個 neighborhood 裡：

$$
\mathcal L(\theta^*)\le \mathcal L(\theta)
$$

## Critical point

可微 function 的 critical point 常滿足：

$$
\boxed{
\nabla\mathcal L(\theta^*)=0
}
$$

但 gradient = 0 不代表一定是 minimum。

可能是：

- local minimum
- local maximum
- saddle point

![](https://commons.wikimedia.org/wiki/Special:FilePath/Saddle_point.svg)

Source: https://commons.wikimedia.org/wiki/File:Saddle_point.svg

## Saddle point

典型例子：

$$
f(x,y)=x^2-y^2
$$

在 $(0,0)$：

$$
\nabla f=0
$$

但沿著 $x$ 方向 function 往上，沿著 $y$ 方向 function 往下，因此不是 minimum 也不是 maximum。

深度網路的高維 loss landscape 裡，saddle point 是重要的 optimization 現象。

## Loss 停滯不等於 local minimum

只看 loss 曲線變平，不能判定停在 local minimum；也可能是梯度很小、saddle 附近、步長太小或其他最佳化問題。沿用 $f(x,y)=x^2-y^2$：

$$
f(0,0)=0,\qquad f(0.1,0)=0.01,\qquad f(0,0.1)=-0.01
$$

同一點附近有上升與下降方向，這才呈現 saddle 的特性。

## Related

- [[Note/Research/ML_DL_Obsidian_Notes/03 - Taylor Expansion Hessian and Eigenvalues|03 - Taylor Expansion Hessian and Eigenvalues]]
- [[Note/Research/ML_DL_Obsidian_Notes/01 - Gradient Descent and Learning Rate|01 - Gradient Descent and Learning Rate]]

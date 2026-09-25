# Gradient Descent and Learning Rate

Training 的核心 optimization problem：

$$
\boxed{
\theta^*=\arg\min_\theta \mathcal L(\theta)
}
$$

初始化：

$$
\theta^{(0)}\sim \text{initialization distribution}
$$

在第 $t$ 步先計算 gradient：

$$
\mathbf g_t=\nabla_\theta\mathcal L(\theta_t)
$$

再更新：

$$
\boxed{
\theta_{t+1}=\theta_t-\eta\mathbf g_t
}
$$

其中 $\eta$ 是 learning rate，是 hyperparameter。

![](https://commons.wikimedia.org/wiki/Special:FilePath/Gradient_descent.svg)

Source: https://commons.wikimedia.org/wiki/File:Gradient_descent.svg

## 為什麼減 gradient？

Gradient 指向 function 上升最快的方向，因此：

$$
-\nabla \mathcal L
$$

就是局部下降最快的方向。

## Learning Rate 太大 / 太小

- 太小：收斂慢
- 太大：可能 overshoot、震盪，甚至 divergence
- 適當：快速下降且穩定

## Gradient = local linear approximation

對很小的 $d\mathbf x$：

$$
f(\mathbf x+d\mathbf x)
\approx
f(\mathbf x)+\nabla f(\mathbf x)^T d\mathbf x
$$

手寫稿例子：

$$
f(\mathbf x)=\mathbf x^T\mathbf x
$$

$$
\mathbf x_0=[3,4]^T,\qquad d\mathbf x=[0.001,0.002]^T
$$

因為：

$$
\nabla f(\mathbf x)=2\mathbf x
$$

所以：

$$
df\approx [6,8]\begin{bmatrix}0.001\\0.002\end{bmatrix}=0.022
$$

而實際 change：

$$
f(\mathbf x_0+d\mathbf x)-f(\mathbf x_0)=0.022005
$$

非常接近。

## 計算例子

令 $\mathcal L(\theta)=(\theta-3)^2$，則 $g=2(\theta-3)$。從 $\theta_0=0$、$\eta=0.1$ 開始：

$$
g_0=-6,\qquad\theta_1=0-0.1(-6)=0.6
$$

$$
g_1=2(0.6-3)=-4.8,\qquad\theta_2=0.6-0.1(-4.8)=1.08
$$

Loss 依序為 $9\rightarrow5.76\rightarrow3.6864$。若改用 $\eta=1.1$，第一步會到 $6.6$，loss 變成 $12.96$；這個例子中過大的步長反而使 loss 增加。

## Related

- [[Note/Research/ML_DL_Obsidian_Notes/02 - Local Minima Saddle Points and Critical Points|02 - Local Minima Saddle Points and Critical Points]]
- [[Note/Research/ML_DL_Obsidian_Notes/01 - Momentum|Momentum]]
- [[Note/Research/ML_DL_Obsidian_Notes/02 - Computational Graph and Backpropagation|Backpropagation]]
- [[Note/Research/ML_DL_Obsidian_Notes/05 - Learning Rate Scheduling and Warm-up|05 - Learning Rate Scheduling and Warm-up]]

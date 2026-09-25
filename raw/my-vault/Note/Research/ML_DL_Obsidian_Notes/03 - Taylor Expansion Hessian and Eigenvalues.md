# Taylor Expansion, Hessian and Eigenvalues

在 $\theta'$ 附近，二階 Taylor approximation：

$$
\mathcal L(\theta)
\approx
\mathcal L(\theta')
+
\nabla\mathcal L(\theta')^T(\theta-\theta')
+
\frac12(\theta-\theta')^T H(\theta')(\theta-\theta')
$$

其中 Hessian：

$$
\boxed{
H_{ij}=\frac{\partial^2\mathcal L}{\partial\theta_i\partial\theta_j}
}
$$

如果 $\theta'$ 是 critical point：

$$
\nabla\mathcal L(\theta')=0
$$

所以主要看二次項：

$$
\mathcal L(\theta)-\mathcal L(\theta')
\approx
\frac12\mathbf v^TH\mathbf v
$$

其中 $\mathbf v=\theta-\theta'$。

## Hessian 與 eigenvalue

若：

$$
H\mathbf u=\lambda\mathbf u
$$

則：

$$
\mathbf u^TH\mathbf u
=
\mathbf u^T(\lambda\mathbf u)
=
\boxed{\lambda\|\mathbf u\|^2}
$$

因此 eigenvalue 的 sign 就是在對應 eigenvector 方向上的 curvature sign。

## Critical point 分類

- 所有 eigenvalues $>0$：$H$ positive definite → strict local minimum
- 所有 eigenvalues $<0$：$H$ negative definite → strict local maximum
- 同時有 positive 和 negative eigenvalues：$H$ indefinite → saddle point

> [!note]
> 若有 eigenvalue = 0，二階測試可能無法直接判定，需要看更高階項或其他分析。

## 手寫稿例子

$$
H=
\begin{bmatrix}
0&-2\\
-2&0
\end{bmatrix}
$$

其 eigenvalues：

$$
\lambda_1=2,\qquad \lambda_2=-2
$$

一正一負，所以這個 curvature 是 indefinite，對應 saddle-type behavior。

## 一維 Taylor 例子

$$
e^x\approx 1+x+\frac12x^2
$$

是在 $x=0$ 做二階 Taylor expansion。

## Related

- [[Note/Research/ML_DL_Obsidian_Notes/02 - Local Minima Saddle Points and Critical Points|02 - Local Minima Saddle Points and Critical Points]]
- [[Note/Research/ML_DL_Obsidian_Notes/01 - Momentum|Momentum]]

# Differentials, Gradients and Jacobians

微分把函數在某一點附近的輸入變化，映射成一階輸出變化。手寫稿的核心是：**導數在固定點是一個線性映射，即使原函數是非線性的。**

## 微分與實際改變量

純量函數可寫成 $df=f'(x)\,dx$。更一般地：

$$
df=Df(x)[dx],\qquad
f(x+\Delta x)=f(x)+Df(x)[\Delta x]+o(\|\Delta x\|)
$$

$df$ 是線性近似給出的變化，不一定等於有限步長的實際差值。若 $f(x)=x^2,x=3,dx=0.1$，則 $df=6(0.1)=0.6$；實際 $f(3.1)-f(3)=0.61$，差異 $0.01$ 是二階項。

## 輸入與輸出形狀

以下向量採 column vector，$J_{ij}=\partial f_i/\partial x_j$。

| 輸入 → 輸出 | 導數的表示 | 微分 |
|---|---|---|
| 純量 → 純量 | 純量 $f'(x)$ | $df=f'(x)dx$ |
| 純量 → 向量 | 向量 $\mathbf f'(x)$ | $d\mathbf f=\mathbf f'(x)dx$ |
| 純量 → 矩陣 | 各元素的導數組成矩陣 $F'(x)$ | $dF=F'(x)dx$ |
| $n$ 維向量 → 純量 | 梯度 $\nabla f\in\mathbb R^n$ | $df=(\nabla f)^T d\mathbf x$ |
| $n$ 維向量 → $m$ 維向量 | Jacobian $J\in\mathbb R^{m\times n}$ | $d\mathbf f=Jd\mathbf x$ |
| 矩陣 → 純量 | 同形狀矩陣梯度 $\nabla_X f$ | $df=\operatorname{tr}((\nabla_X f)^T dX)$ |
| 向量 → 矩陣、矩陣 → 向量或矩陣 | 可用多索引張量，或攤平成 Jacobian | 導數作用於輸入擾動 |

矩陣輸入不代表一定要在程式中建立高階張量。例如矩陣到矩陣，若保留輸出與輸入的四個索引，導數是四階張量；攤平後則可表示成矩陣。

## Gradient 與 Jacobian 計算

令 $f(x_1,x_2)=x_1^2+x_1x_2+x_2^2$，在 $(1,2)$：

$$
\nabla f=(2x_1+x_2,x_1+2x_2)^T=(4,5)^T
$$

取 $d\mathbf x=(0.1,-0.2)^T$，$df=4(0.1)+5(-0.2)=-0.6$。實際 $f(1.1,1.8)-f(1,2)=6.43-7=-0.57$。

再取向量函數：

$$
\mathbf h(x_1,x_2)=\begin{bmatrix}x_1^2+x_2\\x_1x_2\end{bmatrix},\qquad
J=\begin{bmatrix}2x_1&1\\x_2&x_1\end{bmatrix}
$$

在 $(1,2)$，$J=\begin{bmatrix}2&1\\2&1\end{bmatrix}$。若 $d\mathbf x=(0.1,0.2)^T$，則 $d\mathbf h=Jd\mathbf x=(0.4,0.4)^T$。

## 與反向傳播的關係

若上游 loss 梯度為 $\nabla_{\mathbf h}\mathcal L=(3,4)^T$：

$$
\nabla_{\mathbf x}\mathcal L=J^T\nabla_{\mathbf h}\mathcal L
=\begin{bmatrix}2&2\\1&1\end{bmatrix}\begin{bmatrix}3\\4\end{bmatrix}
=\begin{bmatrix}14\\7\end{bmatrix}
$$

前向擾動用 $Jd\mathbf x$；反向傳播用 $J^T\nabla_{\mathbf h}\mathcal L$。自動微分可以直接計算這類乘積，不必顯式建立整個 Jacobian。

## Related

[[Note/Research/ML_DL_Obsidian_Notes/01 - Chain Rule and Total Derivative|01 - Chain Rule and Total Derivative]]
[[Note/Research/ML_DL_Obsidian_Notes/02 - Computational Graph and Backpropagation|02 - Computational Graph and Backpropagation]]

## 手寫原稿

> [!note]- 照片 3：手寫原稿
> ![[Assets/Note/Research/ML_DL_Obsidian_Notes/Photo 3.jpg|700]]

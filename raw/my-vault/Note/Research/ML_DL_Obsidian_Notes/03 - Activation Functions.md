# Activation Functions

## Sigmoid

$$
\boxed{
\sigma(x)=\frac{1}{1+e^{-x}}
}
$$

範圍：

$$
0<\sigma(x)<1
$$

Derivative：

$$
\sigma'(x)=\sigma(x)(1-\sigma(x))
$$

![](https://commons.wikimedia.org/wiki/Special:FilePath/Sigmoid-function-2.svg)

Source: https://commons.wikimedia.org/wiki/File:Sigmoid-function-2.svg

Sigmoid 的主要問題是 $|x|$ 很大時 derivative 很小，深層網路容易出現 vanishing gradient。

## ReLU

Rectified Linear Unit：

$$
\boxed{
\operatorname{ReLU}(x)=\max(0,x)
}
$$

Derivative（忽略 $x=0$ 的 convention）：

$$
\operatorname{ReLU}'(x)=
\begin{cases}
0,&x<0\\
1,&x>0
\end{cases}
$$

![](https://commons.wikimedia.org/wiki/Special:FilePath/ReLU%20and%20GELU.svg)

Source: https://commons.wikimedia.org/wiki/File:ReLU_and_GELU.svg

## 從手寫稿的公式看

原本 sigmoid network：

$$
y=b_o+\sum_i c_i\,\sigma\left(b_i+\sum_jw_{ij}x_j\right)
$$

換成 ReLU：

$$
y=b_o+\sum_i c_i\,\max\left(0,b_i+\sum_jw_{ij}x_j\right)
$$

## Related

- [[Note/Research/ML_DL_Obsidian_Notes/02 - Hidden Layers and Multiple Features|02 - Hidden Layers and Multiple Features]]
- [[Note/Research/ML_DL_Obsidian_Notes/02 - Computational Graph and Backpropagation|Backpropagation]]

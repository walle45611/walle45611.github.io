# Hidden Layers and Multiple Features

假設 input 有很多 features：

$$
\mathbf x=
\begin{bmatrix}
x_1\\x_2\\\vdots\\x_d
\end{bmatrix}
$$

第 $i$ 個 hidden neuron 的 linear part：

$$
\boxed{
v_i=b_i+\sum_j w_{ij}x_j
}
$$

activation：

$$
a_i=\sigma(v_i)
$$

如果 output 是 scalar regression：

$$
\boxed{
y=b_o+\sum_i c_i a_i
}
$$

整體可寫成：

$$
\mathbf v=W\mathbf x+\mathbf b
$$

$$
\mathbf a=\sigma(\mathbf v)
$$

$$
y=\mathbf c^T\mathbf a+b_o
$$

## 為什麼 hidden layer 有用？

只有 linear transformation：

$$
y=W\mathbf x+b
$$

不管堆幾層，只要中間沒有 nonlinear activation，最後仍然等價於一個 linear transformation。

加入 activation：

$$
W_2\sigma(W_1x+b_1)+b_2
$$

才能表示 nonlinear function。

## Related

- [[Note/Research/ML_DL_Obsidian_Notes/01 - Neuron Features and Parameters|01 - Neuron Features and Parameters]]
- [[Note/Research/ML_DL_Obsidian_Notes/03 - Activation Functions|03 - Activation Functions]]

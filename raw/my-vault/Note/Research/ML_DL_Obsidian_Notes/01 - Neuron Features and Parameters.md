# Neuron, Features and Parameters

最基本的 neuron 可以拆成兩步：

$$
z=\mathbf w^T\mathbf x+b
$$

$$
a=\sigma(z)
$$

其中：

- $\mathbf x$：features / input
- $\mathbf w$：weights
- $b$：bias
- $z$：pre-activation / linear output
- $a$：activation

Parameters 是模型需要從資料學到的未知量：

$$
\theta=
\begin{bmatrix}
\mathbf w\\
b\\
\vdots
\end{bmatrix}
$$

![](https://commons.wikimedia.org/wiki/Special:FilePath/Neural%20Network.svg)

Source: https://commons.wikimedia.org/wiki/File:Neural_Network.svg

## 一個 hidden representation 再接 output

若 hidden layer 先得到 $\mathbf h$，output layer 可以寫成：

$$
z_{out}=\mathbf w'^T\mathbf h+b'
$$

classification 時，再把 output logits 丟進 [[Note/Research/ML_DL_Obsidian_Notes/04 - Softmax|Softmax]]。

## Related

- [[Note/Research/ML_DL_Obsidian_Notes/02 - Hidden Layers and Multiple Features|02 - Hidden Layers and Multiple Features]]
- [[Note/Research/ML_DL_Obsidian_Notes/03 - Activation Functions|03 - Activation Functions]]

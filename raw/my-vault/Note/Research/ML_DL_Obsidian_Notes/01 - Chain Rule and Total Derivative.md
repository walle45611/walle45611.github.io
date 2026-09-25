# Chain Rule and Total Derivative

Backpropagation 的數學核心就是 chain rule。

## Case 1：單一路徑

如果：

$$
x=g(s),\qquad y=f(x)
$$

則：

$$
\boxed{
\frac{dy}{ds}=\frac{dy}{dx}\frac{dx}{ds}
}
$$

也就是 computational graph 上 derivative 沿路相乘。

## Case 2：分支後再合流

如果：

$$
x=g(s),\qquad y=h(s),\qquad z=k(x,y)
$$

$s$ 同時透過兩條 path 影響 $z$，因此：

$$
\boxed{
\frac{dz}{ds}
=
\frac{\partial z}{\partial x}\frac{dx}{ds}
+
\frac{\partial z}{\partial y}\frac{dy}{ds}
}
$$

這個「不同 path 的 gradient 要相加」是理解 backprop 很重要的一點。

## Example

$$
f(x,y)=x^2+xy+y^2
$$

則：

$$
\frac{\partial f}{\partial x}=2x+y
$$

$$
\frac{\partial f}{\partial y}=x+2y
$$

## Related

- [[Note/Research/ML_DL_Obsidian_Notes/02 - Computational Graph and Backpropagation|02 - Computational Graph and Backpropagation]]
- [[Note/Research/ML_DL_Obsidian_Notes/04 - Differentials Gradients and Jacobians|04 - Differentials Gradients and Jacobians]]

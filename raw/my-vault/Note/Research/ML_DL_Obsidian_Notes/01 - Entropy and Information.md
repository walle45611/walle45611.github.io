# Entropy and Information

單一事件發生時的 information content：

$$
I(x)=-\log_2 p(x)
$$

所以：

- probability 越高 → information 越低
- probability 越低 → information 越高

例如：

$$
p(x)=\frac18
$$

則

$$
I(x)=-\log_2\frac18=3\text{ bits}
$$

Entropy 是「平均 information content」：

$$
\boxed{H(P)=-\sum_i p_i\log p_i}
$$

如果底數為 2，單位是 bits；如果使用 natural log，單位常寫 nats。

## 直覺

資料分布越平均，通常越難預測，因此 entropy 越高。

例如二元分布：

- $P=(0.5,0.5)$：entropy 最大
- $P=(0.99,0.01)$：entropy 很低
- $P=(1,0)$：entropy 為 0

> [!summary]
> Entropy 不是「某一個事件有多少資訊」，而是整個 probability distribution 的平均不確定性。

## 計算例子

取 $P=(0.75,0.25)$，以 2 為底：

$$
H(P)=-0.75\log_2 0.75-0.25\log_2 0.25
\approx 0.3113+0.5=0.8113\text{ bits}
$$

公平硬幣的熵是 $-2(0.5\log_2 0.5)=1$ bit，因此這個較不平均的分布不確定性較低。

## Related

- [[Note/Research/ML_DL_Obsidian_Notes/02 - Likelihood NLL and Cross-Entropy|02 - Likelihood NLL and Cross-Entropy]]
- [[Note/Research/ML_DL_Obsidian_Notes/04 - Softmax|04 - Softmax]]

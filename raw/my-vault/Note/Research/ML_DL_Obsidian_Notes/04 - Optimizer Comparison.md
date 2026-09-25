# Optimizer Comparison

| Optimizer | 核心概念 | 每參數 adaptive step | Momentum-like memory |
|---|---|---:|---:|
| SGD | 直接沿 negative gradient | No | No |
| Momentum | 累積過去方向 | No | Yes |
| AdaGrad | 累積全部歷史 squared gradients | Yes | No |
| RMSProp | 依近期 squared gradient scale 縮放 | Yes | No |
| Adam | first moment + second moment | Yes | Yes |

## 一條主線記憶

$$
\text{SGD}
\rightarrow
\text{Momentum}
$$

是在解「方向震盪、前進太慢」。

$$
\text{SGD}
\rightarrow
\text{RMSProp}
$$

是在解「不同 parameter 的 gradient scale 差很多」。

$$
\text{Adam}
\approx
\text{Momentum idea}
+
\text{RMSProp-like scaling}
$$

> [!note]
> Adam 很常用，但「最終 generalization 一定優於 SGD」並不成立。Optimizer 是 optimization dynamics 的選擇，不等同於模型本身的泛化能力。

## Related

- [[Note/Research/ML_DL_Obsidian_Notes/01 - Momentum|01 - Momentum]]
- [[Note/Research/ML_DL_Obsidian_Notes/02 - RMSProp|02 - RMSProp]]
- [[Note/Research/ML_DL_Obsidian_Notes/03 - Adam|03 - Adam]]
- [[Note/Research/ML_DL_Obsidian_Notes/04 - Batch Mini-batch and Batch Size|Batch Size]]
- [[Note/Research/ML_DL_Obsidian_Notes/05 - AdaGrad and Gradient Scaling|05 - AdaGrad and Gradient Scaling]]

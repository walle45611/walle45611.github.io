# Underfitting, Overfitting and Model Bias

## Underfitting / high model bias

模型太簡單，連 training data 都 fit 不好：

$$
\mathcal L_{train}\text{ 高}
$$

常見原因：

- model capacity 不足
- features 不夠
- training 不充分
- optimization 失敗

## Overfitting

training loss 很低，但 validation/test loss 明顯較高：

$$
\mathcal L_{train}\ll \mathcal L_{val}
$$

表示模型把 training-specific patterns 學得太細，但 generalization 不好。

## Bias–complexity trade-off

![](https://commons.wikimedia.org/wiki/Special:FilePath/Bias-variance-tradeoff.png)

Source: https://commons.wikimedia.org/wiki/File:Bias-variance-tradeoff.png

隨著 model complexity 增加：

- training error 通常下降
- validation/test error 常先下降，再因 overfitting 上升

因此 model complexity 不是越大越好，而要看 held-out performance。

> [!note]
> 手寫稿中的「model bias」可理解成模型假設太強／表達能力不足所造成的 systematic error，也就是 underfitting 的方向。

## Related

- [[Note/Research/ML_DL_Obsidian_Notes/04 - Model Complexity Data Augmentation and Optimization Failure|04 - Model Complexity Data Augmentation and Optimization Failure]]
- [[Note/Research/ML_DL_Obsidian_Notes/01 - Train Validation and Test Sets|01 - Train Validation and Test Sets]]

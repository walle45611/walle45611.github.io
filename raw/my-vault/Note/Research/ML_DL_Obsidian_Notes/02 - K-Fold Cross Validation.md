# K-Fold Cross Validation

K-fold 的做法：把可用於 model selection 的 data 分成 $K$ 份。

每一輪：

- $K-1$ folds：training
- 1 fold：validation

輪流讓每一 fold 當 validation。

![](https://commons.wikimedia.org/wiki/Special:FilePath/K-fold_cross_validation_EN.svg)

Source: https://commons.wikimedia.org/wiki/File:K-fold_cross_validation_EN.svg

如果第 $k$ 輪 validation score 為 $s_k$：

$$
\boxed{
\bar s=\frac1K\sum_{k=1}^{K}s_k
}
$$

## 正確的資料邊界

通常：

$$
\boxed{
\text{Train/Validation data}
\xrightarrow{K\text{-fold}}
\text{model selection}
}
$$

但真正 final test set 仍應保留，不參與 K-fold model tuning。

## 何時有用？

- dataset 不大
- 想降低單次 train/validation split 帶來的偶然性
- 傳統 ML model selection

大型 deep learning 因 training cost 高，常直接固定 validation set，而不是完整 K-fold。

## Related

- [[Note/Research/ML_DL_Obsidian_Notes/01 - Train Validation and Test Sets|01 - Train Validation and Test Sets]]

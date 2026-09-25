# Likelihood, NLL and Cross-Entropy

## Likelihood

給定 training data $\mathcal D=\{(x_n,y_n)\}_{n=1}^N$，模型參數為 $\theta$：

$$
L(\theta)=p(\mathcal D\mid \theta)
$$

Maximum Likelihood Estimation (MLE) 的目標是：

$$
\theta^*=\arg\max_\theta L(\theta)
$$

乘很多機率不方便，因此通常取 log：

$$
\theta^*=\arg\max_\theta \log L(\theta)
$$

最大化 log-likelihood 等價於最小化 Negative Log-Likelihood：

$$
\boxed{\theta^*=\arg\min_\theta -\log L(\theta)}
$$

## Classification 的 Cross-Entropy

若真實 label distribution 是 $y$，模型預測是 $\hat y$：

$$
\boxed{
\mathcal L_{CE}
=-\sum_{k=1}^{K}y_k\log \hat y_k
}
$$

如果 $y$ 是 one-hot，例如正確類別為 $c$：

$$
y_c=1
$$

其他位置為 0，因此：

$$
\boxed{\mathcal L_{CE}=-\log \hat y_c}
$$

也就是：模型給「正確答案」的 probability 越小，loss 越大。

## 為什麼 CE 和 MLE 常常連在一起？

對 multiclass classification + softmax 而言：

$$
\text{maximize likelihood}
\Longleftrightarrow
\text{minimize NLL}
\Longleftrightarrow
\text{minimize cross-entropy}
$$

> [!important]
> Entropy $H(P)$ 和 Cross-Entropy $H(P,Q)$ 不完全相同。Entropy 衡量真實分布本身的不確定性；Cross-Entropy 衡量用 $Q$ 來描述由 $P$ 產生的資料需要多少資訊。

## 計算例子

假設兩筆獨立樣本的正確類別預測機率分別為 $0.8$、$0.6$，使用自然對數：

$$
\text{Likelihood}=0.8\times0.6=0.48
$$

$$
\text{NLL}=-\ln 0.48=-\ln0.8-\ln0.6
\approx0.2231+0.5108=0.7340
$$

若標籤為 one-hot，平均 cross-entropy 就是 $0.7340/2\approx0.3670$ nats。注意總 NLL 與平均 loss 差了一個樣本數。

## Related

- [[Note/Research/ML_DL_Obsidian_Notes/01 - Entropy and Information|01 - Entropy and Information]]
- [[Note/Research/ML_DL_Obsidian_Notes/04 - Softmax|04 - Softmax]]
- [[Note/Research/ML_DL_Obsidian_Notes/03 - MAE and MSE|03 - MAE and MSE]]

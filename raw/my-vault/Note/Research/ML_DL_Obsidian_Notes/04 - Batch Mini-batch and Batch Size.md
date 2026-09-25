# Batch, Mini-batch and Batch Size

完整 dataset：

$$
\mathcal D=\{(x_n,y_n)\}_{n=1}^{N}
$$

Full-batch gradient：

$$
\nabla\mathcal L(\theta)
=
\frac1N\sum_{n=1}^{N}\nabla\mathcal L_n(\theta)
$$

如果把資料切成 mini-batches $B_1,B_2,\dots$，每次只用其中一批估計 gradient：

$$
\mathbf g_B
=
\frac1{|B|}\sum_{n\in B}\nabla\mathcal L_n(\theta)
$$

更新：

$$
\theta\leftarrow\theta-\eta\mathbf g_B
$$

## Small batch

優點：

- 一個 epoch 裡有更多次 parameter update
- gradient noise 較大，有時能幫助探索 loss landscape
- 單次記憶體需求較低

缺點：

- gradient variance 較大
- GPU utilization 可能比較差

## Large batch

優點：

- gradient estimate 比較穩定
- GPU / accelerator 平行運算效率通常較高

缺點：

- 每個 epoch 的 update 次數較少
- 記憶體需求較高
- learning rate 等 hyperparameters 通常需要重新調整

> [!important]
> 「large batch 一定比較好」或「small batch 一定比較容易跳出 local minimum」都太絕對。實際效果依模型、optimizer、learning-rate schedule 與硬體而定。

## Related

- [[Note/Research/ML_DL_Obsidian_Notes/04 - Optimizer Comparison|Optimizer Comparison]]
- [[Note/Research/ML_DL_Obsidian_Notes/03 - Gradient Accumulation Across a Batch|Gradient Accumulation]]

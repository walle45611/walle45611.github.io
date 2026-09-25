# Learning Rate Scheduling and Warm-up

Scheduling 讓 learning rate 隨訓練步數改變。Decay 是下降階段；warm-up 是開始時逐漸升高。它們可以與 SGD、Momentum、Adam 等優化器搭配。

$$
\theta_{t+1}=\theta_t-\eta_t g_t
$$

對 adaptive optimizer，還有每個參數的梯度尺度調整。全域 $\eta_t$ 與逐參數分母是兩個不同因素，不能把所有步長變化都叫 scheduling。

## Decay 與計算例子

一種 exponential decay：

$$
\eta_t=\eta_0\alpha^t,\qquad0<\alpha<1
$$

若 $\eta_0=0.1,\alpha=0.9$，前三個值為 $0.1,0.09,0.081$。當梯度固定為 $2$ 作為運算示例，對應移動量為 $0.2,0.18,0.162$。實際訓練會在每次更新後重新算梯度。

## Linear warm-up 接 linear decay

以下用 $t=0,\ldots,T$ 表示排程位置，$W$ 為 warm-up 結束位置，$T>W$：

$$
\eta_t=
\begin{cases}
\eta_{\max}\dfrac{t}{W},&0\le t\le W,\\
\eta_{\max}\dfrac{T-t}{T-W},&W<t\le T.
\end{cases}
$$

例：$W=2,T=6,\eta_{\max}=0.1$。

| $t$ | 0 | 1 | 2 | 3 | 4 | 5 | 6 |
|---|---|---|---|---|---|---|---|
| $\eta_t$ | 0 | 0.05 | 0.1 | 0.075 | 0.05 | 0.025 | 0 |

這張表定義了數學端點；實作時需確認 scheduler 是更新前或更新後呼叫，以及從第 0 步或第 1 步開始。Warm-up 不表示整段訓練都使用小 learning rate。

## 手寫稿提到的 RAdam

手寫稿把 warm-up 與 BERT、Residual Network 並列，可作為相關訓練情境的提示，不代表這些架構一定要使用同一種排程。

RAdam 論文從訓練初期 adaptive learning rate 的高變異性解釋 warm-up，並提出修正項。這是該論文的理論與實驗觀點，不宜簡化成「所有 warm-up 都只因為標準差估不準」，也不表示 RAdam 在所有任務都更好。[On the Variance of the Adaptive Learning Rate and Beyond](https://arxiv.org/abs/1908.03265)

## Related

[[Note/Research/ML_DL_Obsidian_Notes/01 - Gradient Descent and Learning Rate|01 - Gradient Descent and Learning Rate]]
[[Note/Research/ML_DL_Obsidian_Notes/03 - Adam|03 - Adam]]

## 手寫原稿

> [!note]- 照片 2：手寫原稿
> ![[Assets/Note/Research/ML_DL_Obsidian_Notes/Photo 2.jpg|700]]

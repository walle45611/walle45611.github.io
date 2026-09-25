# Direct Preference Optimization: Your Language Model is Secretly a Reward Model

[[Assets/Note/Research/Papers/Direct Preference Optimization - Your Language Model is Secretly a Reward Model/Direct Preference Optimization - Your Language Model is Secretly a Reward Model.pdf|論文 PDF]]

## 初讀摘要

- DPO 將偏好對齊轉成直接最佳化偏好資料的分類式目標，簡化先訓練 reward model 再進行強化學習的流程。
- 透過 reward 與最優 policy 的關係，把偏好損失寫成目前模型相對於參考模型的回答機率比，提升偏好答案相對於被拒答案的機率。
- 原文在情緒控制、摘要與單輪對話展示與既有 RLHF 方法相當或更好的結果；理解重點是參考模型與偏好資料如何約束更新，而非把 DPO 視為一般正例微調。

摘要依據：PDF 摘要與導論。


閱讀狀態：**閱讀中，尚未讀完。** 目前整理到偏好模型、reward model 的訓練，以及帶 KL 約束的 RLHF 目標；尚未整理 DPO 核心推導與最終 loss。

論文：[Rafailov et al., arXiv:2305.18290](https://arxiv.org/abs/2305.18290)。內容依手寫照片整理，計算例子補上中間步驟。

## 偏好資料與 Bradley–Terry 模型

$x$ 是 prompt，$y_w$ 是一組比較中較受偏好的回答，$y_l$ 是較不受偏好的回答。資料可表示為 $(x,y_w,y_l)$。這表示相對偏好，不等於 $y_w$ 必然正確或 $y_l$ 必然錯誤。

用 $r(x,y)$ 表示回答的純量分數，偏好機率建模為：

$$
P(y_w\succ y_l\mid x)
=\frac{e^{r(x,y_w)}}{e^{r(x,y_w)}+e^{r(x,y_l)}}
$$

令 $r_w=r(x,y_w),r_l=r(x,y_l)$，分子分母同除以 $e^{r_w}$：

$$
P(y_w\succ y_l\mid x)
=\frac1{1+e^{r_l-r_w}}
=\sigma(r_w-r_l)
$$

因此兩個分數的 softmax，可以寫成分數差的 sigmoid。Reward 本身不必在 $[0,1]$；轉換後才是偏好機率，且它不是語言模型生成整段回答的機率。

真實偏好與 reward 通常未知，可用 $r^*$ 表示假設存在的真實分數，以參數模型 $r_\phi$ 近似。Bradley–Terry 是建模假設，不表示所有人類偏好都必然符合它。

## 手寫例子與 reward loss

設 $r_w=2,r_l=1$：

$$
P(y_w\succ y_l\mid x)
=\frac{e^2}{e^2+e^1}
\approx\frac{7.3891}{7.3891+2.7183}
\approx0.7311
$$

也可直接算 $\sigma(2-1)=\sigma(1)\approx0.7311$，約為 $73.11\%$。若兩分數相同，機率為 $0.5$；分數對調則約為 $0.2689$。

對偏好資料集 $\mathcal D$，reward model 的負對數似然為：

$$
\mathcal L_R(\phi)
=-\mathbb E_{(x,y_w,y_l)\sim\mathcal D}
\left[\log\sigma\big(r_\phi(x,y_w)-r_\phi(x,y_l)\big)\right]
$$

單筆例子的 loss 為 $-\ln0.7311\approx0.3133$；分數相同時為 $-\ln0.5\approx0.6931$。

令分數差 $\Delta=r_w-r_l$，則 $\partial\mathcal L/\partial\Delta=\sigma(\Delta)-1$。在 $\Delta=1$ 時約為 $-0.2689$，梯度下降傾向提高偏好回答相對於另一回答的分數。

這裡是 **reward modeling 的監督式偏好學習目標，還不是最終 DPO loss**。此外，兩個 reward 同加常數不改變機率：$\sigma((r_w+c)-(r_l+c))=\sigma(r_w-r_l)$，因此偏好比較只識別分數差。

## 帶 KL 約束的 RLHF 目標

手寫稿接著記錄的目標可寫成：

$$
\max_\theta\;
\mathbb E_{x\sim\mathcal D}
\left[
\mathbb E_{y\sim\pi_\theta(\cdot\mid x)}[r_\phi(x,y)]
-\beta D_{\mathrm{KL}}\big(\pi_\theta(\cdot\mid x)\|\pi_{\mathrm{ref}}(\cdot\mid x)\big)
\right]
$$

$\pi_\theta$ 是要調整的語言模型，$\pi_{\mathrm{ref}}$ 是固定參考模型，常取 SFT 後模型；$r_\phi$ 是前面訓練的 reward model。$\beta>0$ 控制這個目標中的 KL 懲罰強度。

$$
D_{\mathrm{KL}}(\pi_\theta\|\pi_{\mathrm{ref}})
=\mathbb E_{y\sim\pi_\theta}
\left[\log\frac{\pi_\theta(y\mid x)}{\pi_{\mathrm{ref}}(y\mid x)}\right]
$$

KL 比較的是兩個模型的**回答分布**，不是參數的歐氏距離；它也不對稱。這個形式在鼓勵高 reward 的同時，懲罰偏離參考分布。

補充計算：假設某個 prompt 下只有兩種回答，$\pi_\theta=(0.75,0.25)$、$\pi_{\mathrm{ref}}=(0.5,0.5)$，reward 為 $(2,1)$：

$$
\mathbb E[r]=0.75(2)+0.25(1)=1.75
$$

$$
D_{\mathrm{KL}}=0.75\ln1.5+0.25\ln0.5\approx0.1308
$$

若 $\beta=0.1$，此 prompt 的目標值約為 $1.75-0.1(0.1308)=1.7369$。這是示意分布，不是實際語言模型輸出。

## 接下來閱讀的問題

- 帶 KL 約束的目標如何推導出最優 policy？
- Reward 如何改寫為 policy 與 reference policy 的 log probability ratio？
- 如何代入偏好機率，得到直接訓練 policy 的 DPO loss？

以上留待後續閱讀補上，不視為已完成的推導。

[[Logistic Regression 與 Log-Odds 計算|Sigmoid 與 log-odds]]
[[Note/Research/ML_DL_Obsidian_Notes/02 - Likelihood NLL and Cross-Entropy|Likelihood 與負對數似然]]

## 手寫原稿

> [!note]- 照片 1：偏好機率、reward modeling 與 RLHF 目標
> ![[Assets/Note/Research/Papers/Direct Preference Optimization - Your Language Model is Secretly a Reward Model/Photo 1.jpg|800]]

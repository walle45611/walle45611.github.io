# ZhiFangDanTai: Fine-Tuning Graph-Based Retrieval-Augmented Generation Model for Traditional Chinese Medicine Formula

[[Assets/Note/Research/Papers/ZhiFangDanTai - Fine-Tuning Graph-Based Retrieval-Augmented Generation Model for Traditional Chinese Medicine Formula/ZhiFangDanTai - Fine-Tuning Graph-Based Retrieval-Augmented Generation Model for Traditional Chinese Medicine Formula.pdf|論文 PDF]]

## 初讀摘要

- ZhiFangDanTai 結合 GraphRAG 與 LLM 微調，改善中醫方劑生成的內容完整性與說明能力。
- 透過圖譜檢索整理結構化中醫知識，並建立包含方劑組成、配伍角色、功效及相關診斷資訊的增強指令資料，訓練模型使用檢索內容。
- 原文提供理論分析與資料集實驗支持結合方法的效果；理論中的假設與實驗指標需分開核對，不能把誤差或幻覺界限解讀為任意臨床場景的保證。

摘要依據：PDF 摘要與導論。


閱讀狀態：**閱讀中，尚未讀完。** 目前記錄 Proposition 1 的互資訊直覺，以及 Taylor 展開與梯度下降界限；其他命題與實驗尚未整理。

來源：[arXiv:2509.05867v1](https://arxiv.org/html/2509.05867v1)、[期刊 DOI](https://doi.org/10.1109/JBHI.2025.3607819)。以下以三張手寫稿為主，對照預印本 §III-F1；計算例子與推導疑問是整理時補充。

## 目前閱讀的問題

這篇研究結合 GraphRAG 與語言模型微調來處理中醫方劑任務。目前關注的是：檢索到的上下文提供什麼額外資訊，以及這與誤差下降有何關係。

| 符號 | 本筆記的理解 |
|---|---|
| $X$ | 輸入，例如症狀描述 |
| $C$ | GraphRAG 提供的上下文或摘要 |
| $Y$ | 目標回答 |
| $I(Y;C\mid X)$ | 已知輸入後，上下文對目標的額外資訊 |
| $\gamma$ | 假設的互資訊下界 |
| $\beta$ | 此處 loss 的 smoothness 常數，並非 DPO 筆記中的 KL 係數 |
| $\eta$ | 梯度下降步長 |

## 條件互資訊的直覺

$$
I(Y;C\mid X)=H(Y\mid X)-H(Y\mid X,C)
$$

已知症狀 $X$ 後，再知道檢索內容 $C$，能減少多少對目標 $Y$ 的不確定性？這是分布上的平均量，不是某一份檢索文字的直接品質分數。

補充例子：若 $H(Y\mid X)=1.2$ nats、$H(Y\mid X,C)=0.8$ nats，則互資訊是 $0.4$ nats。這不是「正確率增加 40%」。

當機率來自同一個真實聯合分布時，也有：

$$
I(Y;C\mid X)
=\mathbb E\left[\log\frac{p(Y\mid X,C)}{p(Y\mid X)}\right]
$$

任意訓練模型的 $q_\theta$ 不必等於真實 $p$，因此模型 NLL 的差不能自動當成互資訊。另一個待釐清點是：若固定知識庫後 $C$ 完全是 $X$ 的確定函數，標準定義下 $I(Y;C\mid X)=0$；要解釋正的下界，需要說明隨機變數、外部知識與分布如何設定。這不否定檢索對有限能力模型有實際幫助。

## 手寫 Proposition 1：先保留主張與條件

照片記錄的主張是：若 $I(Y;C\mid X)\ge\gamma$，GraphRAG 與 SFT 的組合可得到形如以下的界：

$$
\mathcal E(\theta_{\mathrm{GraphRAG+SFT}})
\le\mathcal E(\theta_{\mathrm{SFT}})-\frac{\gamma}{\beta}
$$

這是原文命題的主張，**尚不能只靠「上下文有用」就認定成立**。原文後續還連結互資訊、梯度大小與多步更新；下面先整理照片中能直接跟算的 smoothness 推導。[對照 §III-F1](https://arxiv.org/html/2509.05867v1#S3.SS6.SSS1)

## Taylor 展開到下降界

先把 $\mathcal E(\theta)$ 當成正在分析的可微目標。假設其梯度滿足 $\beta$-Lipschitz 條件：

$$
\|\nabla\mathcal E(\theta')-\nabla\mathcal E(\theta)\|
\le\beta\|\theta'-\theta\|
$$

在二次可微等適當條件下，可用 Hessian 的算子範數上界 $\|\nabla^2\mathcal E\|_{op}\le\beta$ 理解。它約束曲率大小，並不要求函數凸。

令 $\Delta\theta=\theta_{t+1}-\theta_t$，Taylor 餘項中間點 $\xi$ 位於兩個參數向量的線段上，不是訓練資料點，也不是任意挑的點：

$$
\mathcal E(\theta_{t+1})
=\mathcal E(\theta_t)+\nabla\mathcal E(\theta_t)^T\Delta\theta
+\frac12\Delta\theta^T\nabla^2\mathcal E(\xi)\Delta\theta
$$

因此：

$$
\mathcal E(\theta_{t+1})
\le\mathcal E(\theta_t)+\nabla\mathcal E(\theta_t)^T\Delta\theta
+\frac\beta2\|\Delta\theta\|^2
$$

代入梯度下降 $\Delta\theta=-\eta\nabla\mathcal E(\theta_t)$。一階項為 $-\eta\|\nabla\mathcal E(\theta_t)\|^2$，二階上界為 $\beta\eta^2\|\nabla\mathcal E(\theta_t)\|^2/2$：

$$
\boxed{\mathcal E(\theta_{t+1})\le\mathcal E(\theta_t)
-\left(\eta-\frac{\beta\eta^2}{2}\right)\|\nabla\mathcal E(\theta_t)\|^2}
$$

這裡使用同一目標的精確梯度。Mini-batch 梯度、訓練 loss 與測試風險的差異，不能直接略去。

## 為什麼取步長 $1/\beta$

令 $g(\eta)=\eta-\beta\eta^2/2$，則：

$$
g'(\eta)=1-\beta\eta,\qquad g''(\eta)=-\beta<0
$$

所以 $\eta=1/\beta$ 使這個保證下降量的係數最大；不是說它必然是任何實際訓練最好的 learning rate。

$$
g(1/\beta)=\frac1\beta-\frac\beta2\frac1{\beta^2}=\frac1{2\beta}
$$

$$
\boxed{\mathcal E(\theta_{t+1})\le\mathcal E(\theta_t)
-\frac1{2\beta}\|\nabla\mathcal E(\theta_t)\|^2}
$$

當 $0<\eta<2/\beta$，係數為正；若梯度為零，這個界不保證嚴格下降。

### 照片中的 $\beta=10$ 例子

此時 $g(\eta)=\eta-5\eta^2$，最大值在 $\eta=0.1$，$g(0.1)=0.05$。若當前目標為 $2$、梯度範數平方為 $4$：

$$
\mathcal E(\theta_{t+1})\le2-0.05(4)=1.8
$$

若取 $\eta=0.3$，$g(0.3)=-0.15$，這個界就不再提供下降保證。

## 尚待釐清的推導銜接

原文使用 $\|\nabla\mathcal E\|^2\ge I(Y;C\mid X)^2$，再從多步更新得到 $\gamma/\beta$ 的總改善。這兩步不是 smoothness 本身的結論，需另查依據與步數條件。單步目標下降也不等於兩個訓練方案的泛化差距已獲證明。

接著閱讀時，先確認 $\mathcal E$ 究竟是訓練目標、期望 NLL 或泛化誤差，以及上述三者在證明中如何對應。其餘命題與實驗結果留待讀完再整理。

## 相關筆記

[[Note/Research/ML_DL_Obsidian_Notes/03 - Taylor Expansion Hessian and Eigenvalues|Taylor Expansion、Hessian 與特徵值]]
[[Note/Research/ML_DL_Obsidian_Notes/01 - Gradient Descent and Learning Rate|Gradient Descent 與 Learning Rate]]
[[Note/Research/ML_DL_Obsidian_Notes/01 - Entropy and Information|Entropy 與資訊量]]
[[Direct Preference Optimization Your Language Model is Secretly a Reward Model|DPO（閱讀中）]]

## 手寫原稿

> [!note]- 照片 1：Proposition 1 與條件互資訊
> ![[Assets/Note/Research/Papers/ZhiFangDanTai - Fine-Tuning Graph-Based Retrieval-Augmented Generation Model for Traditional Chinese Medicine Formula/Photo 1.jpg|800]]

> [!note]- 照片 2：Taylor 展開與步長最佳化
> ![[Assets/Note/Research/Papers/ZhiFangDanTai - Fine-Tuning Graph-Based Retrieval-Augmented Generation Model for Traditional Chinese Medicine Formula/Photo 2.jpg|800]]

> [!note]- 照片 3：梯度下降上界與係數化簡
> ![[Assets/Note/Research/Papers/ZhiFangDanTai - Fine-Tuning Graph-Based Retrieval-Augmented Generation Model for Traditional Chinese Medicine Formula/Photo 3.jpg|800]]

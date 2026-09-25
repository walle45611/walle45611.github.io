# AdaGrad and Gradient Scaling

AdaGrad 依每個參數過去累積的梯度平方調整步長。以下使用逐座標版本、固定全域 learning rate，初始累積量 $G_{0,i}=0$：

$$
g_{t,i}=\frac{\partial\mathcal L}{\partial\theta_i},\qquad
G_{t,i}=G_{t-1,i}+g_{t,i}^2
$$

$$
\theta_{t+1,i}=\theta_{t,i}-\eta\frac{g_{t,i}}{\sqrt{G_{t,i}}+\epsilon}
$$

累積量不會下降，因此固定 $\eta$ 下的有效係數 $\eta/(\sqrt{G_{t,i}}+\epsilon)$ 不會增加；但實際更新量仍取決於當步梯度。參考：[AdaGrad 原論文](https://jmlr.org/papers/v12/duchi11a.html)、[更新公式](https://docs.pytorch.org/docs/2.14/generated/torch.optim.Adagrad.html)。

## 手寫稿的 RMS 與標準 AdaGrad

手寫稿從梯度歷史的 root mean square 出發：

$$
\operatorname{RMS}_{t,i}=\sqrt{\frac1t\sum_{k=1}^t g_{k,i}^2}
$$

這個 RMS 定義本身正確，但標準 AdaGrad 的分母採平方**總和**的平方根：

$$
\sqrt{G_{t,i}}=\sqrt{\sum_{k=1}^t g_{k,i}^2}
=\sqrt t\operatorname{RMS}_{t,i}
$$

忽略 $\epsilon$、使用相同全域 $\eta$ 時，改除以 RMS 會讓更新量是標準 AdaGrad 的 $\sqrt t$ 倍。若同時把全域 learning rate 改成 $\eta/\sqrt t$，才可抵消差異。

## 兩步計算

給定兩步梯度皆為 $2$，$\eta=0.1$，參數初值為 $1$。忽略極小的 $\epsilon$：

$$
G_1=4,\qquad\theta_{\text{after 1}}=1-0.1\frac2{\sqrt4}=0.9
$$

$$
G_2=8,\qquad\theta_{\text{after 2}}=0.9-0.1\frac2{\sqrt8}\approx0.8293
$$

若改用歷史 RMS，第二步分母是 $\sqrt{8/2}=2$，參數會到 $0.8$。這正是總和與平均造成的差異。

## 與 RMSProp、Momentum、Adam 的關係

AdaGrad 保留全部歷史平方梯度；RMSProp 改用 $s_t=\rho s_{t-1}+(1-\rho)g_t^2$，讓較舊的梯度影響逐漸衰減。

平方會消除正負號，因此尺度統計本身不保留方向；更新分子 $g_t$ 仍保留方向。Momentum 累積帶符號梯度，Adam 則結合一階動量、平方梯度尺度與 bias correction。手寫稿的「Adam = RMSProp + Momentum」適合概念記憶，不能直接把兩份程式相加當成完整公式。

## Related

[[Note/Research/ML_DL_Obsidian_Notes/02 - RMSProp|02 - RMSProp]]
[[Note/Research/ML_DL_Obsidian_Notes/03 - Adam|03 - Adam]]
[[Note/Research/ML_DL_Obsidian_Notes/04 - Optimizer Comparison|04 - Optimizer Comparison]]

## 手寫原稿

> [!note]- 照片 4：手寫原稿
> ![[Assets/Note/Research/ML_DL_Obsidian_Notes/Photo 4.jpg|700]]

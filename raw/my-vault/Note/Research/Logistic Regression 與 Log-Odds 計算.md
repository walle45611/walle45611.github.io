# Logistic Regression 與 Log-Odds 計算

來源：Attention 手寫照片第六張左側。這是二元分類基礎補充，並非 Attention Is All You Need 特有的公式。

令 $p=P(y=1\mid x)$，模型先形成線性分數 $s=w^Tx+b$；若將 $x_0=1$ 放入特徵，bias 也可併入 $w$。

$$
\operatorname{odds}=\frac{p}{1-p},\qquad
\log\frac{p}{1-p}=s
$$

Odds 是勝算，不是機率。將 log-odds 解回 $p$：

$$
\frac{p}{1-p}=e^s
\Rightarrow p=e^s(1-p)
\Rightarrow p(1+e^s)=e^s
$$

$$
p=\frac{e^s}{1+e^s}=\frac1{1+e^{-s}}=\sigma(s)
$$

## 計算例子

若 $w=(1,-1),x=(2,1),b=0$，則 $s=2-1=1$，$p=1/(1+e^{-1})\approx0.7311$。勝算為 $e\approx2.7183$；取自然對數後回到 $s=1$。

若 $p=0.8$，勝算是 $0.8/0.2=4$，log-odds 為 $\ln4\approx1.3863$，再套 sigmoid 就回到 $0.8$。

[[Note/Research/ML_DL_Obsidian_Notes/03 - Activation Functions|Sigmoid 與其他活化函數]]
[[Note/Research/ML_DL_Obsidian_Notes/02 - Likelihood NLL and Cross-Entropy|Likelihood、NLL 與 Cross-Entropy]]
[[Attention Is All You Need]]

## 手寫原稿

> [!note]- 照片 6：手寫原稿
> ![[Assets/Note/Research/Papers/Attention Is All You Need/Photo 6.jpg|800]]

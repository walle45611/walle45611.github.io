# Attention Is All You Need

[[Assets/Note/Research/Papers/Attention Is All You Need/Attention Is All You Need.pdf|論文 PDF]]

## 初讀摘要

- Transformer 以 attention 為核心建立序列轉換模型，取代當時主要依靠循環或卷積的 encoder–decoder。
- 模型結合多頭注意力、位置資訊與逐位置前饋網路；decoder 透過遮罩限制未來資訊，並對 encoder 輸出進行 attention。
- 原文在英德與英法翻譯實驗中取得較好的品質與訓練效率。閱讀時可聚焦 Q、K、V 的角色，以及 self-attention、cross-attention 與位置編碼的分工。

摘要依據：PDF 摘要與導論。


來源：本次七張手寫照片；公式對照 [Attention Is All You Need，§3](https://arxiv.org/html/1706.03762v7#S3)。以下小維度數值為教學例子，不是訓練所得的 attention 權重。

Transformer 用 attention 建立序列位置之間的關聯，再透過 FFN 轉換各位置的表示；位置編碼提供順序資訊。原論文採 encoder–decoder 架構。這篇整合手寫重點、數值計算與位置編碼直覺，並將後來的 Pre-Norm 設計標為延伸內容。

## Scaled Dot-Product Attention

將同一序列的表示 $X$ 投影成 $Q=XW_Q,K=XW_K,V=XW_V$。Query 與各個 key 比較，再用得到的權重加總 value：

$$
A=\operatorname{softmax}\left(\frac{QK^T}{\sqrt{d_k}}+M\right),\qquad O=AV
$$

Softmax 沿 key 位置計算，每個 query 對應一列。$A$ 是 attention 權重，不是下一個 token 的詞彙機率；$O$ 是 value 的加權組合。Self-attention 的三者來自同一序列，但投影矩陣不同，因此 $Q,K,V$ 不必相等。

手寫的「a cute teddy bear is reading」用詞語示意位置；實際 tokenizer 可能將 teddy bear 分成兩個以上 token，不能直接把詞語數當作 token 數。

### 一個 query 的完整計算

取 $q=(1,0)$、$k_1=(1,0),k_2=(0,1)$，$v_1=(2,0),v_2=(0,4)$，因此 $d_k=2$：

$$
qK^T=(1,0),\qquad s=(1/\sqrt2,0)\approx(0.7071,0)
$$

$$
(a_1,a_2)=\frac{(e^{0.7071},1)}{e^{0.7071}+1}
\approx(0.6698,0.3302)
$$

$$
o=0.6698(2,0)+0.3302(0,4)\approx(1.3395,1.3210)
$$

### 為何除以平方根

在各分量彼此獨立、平均為 0、變異數為 1 的示意假設下，$q_i k_i$ 的變異數為 1，各乘積之間共變異數為 0：

$$
\operatorname{Var}(q\cdot k)=\sum_{i=1}^{d_k}\operatorname{Var}(q_i k_i)=d_k
$$

若 $d_k=64$，標準差為 $8$，所以除以 $\sqrt{64}=8$ 後變異數為 1。這解釋縮放動機，不代表真實訓練中的 $Q,K$ 永遠符合獨立標準分布，也不是每次重新估計點積的標準差。

## Multi-Head Attention 與 shape

$B$ 是 batch size、$h$ 是 **head 數量**、$L$ 是序列長度、$d_k$ 是每個 head 的 query/key 維度。以下沿用手寫設定 $B=2,h=8,L=10,d_k=d_v=64,d_{model}=512$。

| 運算 | Shape |
|---|---|
| 輸入 $X$ | $[2,10,512]$ |
| 合併投影 $XW_Q$ | $[2,10,512]$ |
| reshape | $[2,10,8,64]$ |
| 交換序列軸與 head 軸 | $[2,8,10,64]$ |
| $QK^T$，只轉置最後兩軸 | $[2,8,10,10]$ |
| 每個 head 的 $AV$ | $[2,8,10,64]$ |
| 交換軸後串接 heads | $[2,10,512]$ |
| 乘 $W_O\in\mathbb R^{512\times512}$ | $[2,10,512]$ |

不能只把 $[B,L,h d_k]$ 直接 reshape 成 $[B,h,L,d_k]$ 而忽略軸順序。

`Q[:, 0, :, :]` 的 shape 是 `[2,10,64]`，表示**兩筆樣本**的第 1 個 head；`Q[1, 0, :, :]` 才是第 2 筆樣本、第 1 個 head，shape 為 `[10,64]`。

$$
\operatorname{MultiHead}=\operatorname{Concat}(O_1,\ldots,O_h)W_O
$$

每個 head 使用學到的不同投影，不是把 token 序列切成八段。

## Encoder、Decoder 與 causal mask

Encoder self-attention 可看來源序列所有有效位置。Decoder self-attention 使用 causal mask：第 $i$ 個位置可看 $j\le i$ 的 decoder 輸入，不能看後面的位置。

例如三個位置的 additive mask：

$$
M=\begin{bmatrix}0&-\infty&-\infty\\0&0&-\infty\\0&0&0\end{bmatrix}
$$

Mask 必須在 softmax **之前**加到 logits；手寫簡式省略的 softmax 需補回。若某列 logits 原為 $(1,2,3)$，只允許前兩個位置，則：

$$
\operatorname{softmax}(1,2,-\infty)\approx(0.2689,0.7311,0)
$$

訓練時 decoder 輸入右移一格，例如輸入 `BOS, I, love`，對應目標 `I, love, apples`。可平行算所有位置的 loss，但 mask 防止看到答案；生成時則逐步加入已生成 token。

原論文 decoder 還有 **cross-attention**：query 來自 decoder，key/value 來自 encoder 輸出。這一層可讀完整來源序列，不使用遮住未來來源詞的 causal mask，但仍須處理 padding。

## FFN 與詞彙輸出

Position-wise FFN 對每個位置使用相同權重，分別轉換該位置的向量；跨 token 的混合由 attention 完成。原論文使用 ReLU，GELU、SiLU 是其他設計中常見的替代，不應混寫為原論文設定。

採 column vector 記法：

$$
\operatorname{FFN}(x)=W_2\operatorname{ReLU}(W_1x+b_1)+b_2
$$

$$
x\in\mathbb R^{512},\quad W_1\in\mathbb R^{2048\times512},\quad
W_2\in\mathbb R^{512\times2048}
$$

若採論文的 row vector 寫法，則矩陣轉置方向相反；乘法順序與 shape 必須一致。

小例子：$x=(1,-2)^T$，$W_1=\begin{bmatrix}1&0\\0&1\\1&1\end{bmatrix}$，$W_2=\begin{bmatrix}1&2&0\\0&1&1\end{bmatrix}$，bias 皆為零。則 $W_1x=(1,-2,-1)^T$，ReLU 後為 $(1,0,0)^T$，輸出為 $(1,0)^T$。

最後的 hidden state 再投影到詞彙表大小 $|\mathcal V|$：

$$
\text{logits}=hW_{vocab},\qquad p=\operatorname{softmax}(\text{logits})
$$

例如手寫的內積 $0.2(0.1)+0.7(0.8)+(-0.1)(0)=0.58$ 是一個 logit，不是機率。若完整詞彙 logits 為 $(0.58,0,-0.58)$，softmax 約為 $(0.5338,0.2989,0.1673)$。

## LayerNorm、Post-Norm 與 Pre-Norm

Transformer 此處用 LayerNorm，對同一 token 的特徵維度計算統計量，不是跨 mini-batch 的 BatchNorm：

$$
\mu=\frac1d\sum_i x_i,\quad v=\frac1d\sum_i(x_i-\mu)^2,\quad
\operatorname{LN}(x)_i=\gamma_i\frac{x_i-\mu}{\sqrt{v+\epsilon}}+\beta_i
$$

以下省略 dropout。原論文採 **Post-Norm**：

$$
x_{\ell+1}=\operatorname{LN}(x_\ell+F_\ell(x_\ell))
$$

照片左側畫的是後來常見的 **Pre-Norm**。以沒有 cross-attention 的兩個子層為例：

$$
u=x+\operatorname{Attention}(\operatorname{LN}_1(x)),\qquad
x_{next}=u+\operatorname{FFN}(\operatorname{LN}_2(u))
$$

兩個 LN 可有各自的參數；attention 依架構可帶 causal mask。完整原論文 decoder 還須包含 cross-attention，不能用這兩行代表它的全部。

Pre-Norm 每個子層可寫成 $x_{\ell+1}=x_\ell+F_\ell(\operatorname{LN}(x_\ell))$，因此沿 residual 主路徑有恆等項：

$$
\frac{\partial x_{\ell+1}}{\partial x_\ell}=I+J_{F_\ell}J_{\operatorname{LN}}
$$

Post-Norm 則為 $J_{\operatorname{LN}}(I+J_{F_\ell})$。這有助理解兩者的梯度路徑差異，但不保證 Pre-Norm 永遠更好或完全不需 warm-up。延伸：[On Layer Normalization in the Transformer Architecture](https://arxiv.org/abs/2002.04745)。

## Sinusoidal Positional Encoding

位置編碼與 token embedding 同維度，才能逐元素相加。偶數維度用 sin、奇數維度用 cos，角度以弧度計算：

$$
PE(pos,2i)=\sin\left(\frac{pos}{10000^{2i/d_{model}}}\right),\quad
PE(pos,2i+1)=\cos\left(\frac{pos}{10000^{2i/d_{model}}}\right)
$$

若 $d_{model}=8$，$i=0,1,2,3$，共四組 sin/cos，並非 $i=0,\ldots,7$。分母依序為 $1,10,100,1000$。

在 $pos=3$：

$$
PE(3)\approx(0.141120,-0.989992,0.295520,0.955336,
0.029996,0.999550,0.003000,0.999996)
$$

### 8 維逐項表格與 embedding 相加

第七張原稿沿用 $d_{model}=8,pos=3$。令 $\omega_i=1/10000^{2i/8}$，每組角度就是 $pos\cdot\omega_i$：

| $i$ | 維度 $2i,2i+1$ | 分母 | $\omega_i$ | 角度（弧度） | sin | cos |
|---|---|---|---|---|---|---|
| 0 | 0, 1 | 1 | 1 | 3 | 0.141120 | -0.989992 |
| 1 | 2, 3 | 10 | 0.1 | 0.3 | 0.295520 | 0.955336 |
| 2 | 4, 5 | 100 | 0.01 | 0.03 | 0.029996 | 0.999550 |
| 3 | 6, 7 | 1000 | 0.001 | 0.003 | 0.003000 | 0.999996 |

高頻組隨位置變動較快，低頻組變動較慢。手寫的 $\cos(0.03)\approx0.9996$ 是四位小數近似，$\cos(0.003)\approx1.0000$ 也不表示精確值等於 1。

沿用照片中的 embedding，此處視為已完成所需縮放的向量：

$$
E(\mathrm{cat})=(0.2,-0.1,0.5,0.3,-0.4,0.8,0.1,-0.2)
$$

逐元素相加，例如第一維 $0.2+0.141120=0.341120$，第二維 $-0.1-0.989992=-1.089992$：

$$
h_3^{(0)}=E(\mathrm{cat})+PE(3)
\approx
(0.341120, -1.089992, 0.795520, 1.255336, -0.370004, 1.799550, 0.103000, 0.799996)
$$

上標 $(0)$ 表示進入第一個 Transformer block 前的輸入表示，不是零次方；下標 $3$ 表示位置。若 $E$ 是原論文中未縮放的 lookup，則應改算 $\sqrt8 E+PE(3)$。

### 相加的數值例子

取 $d_{model}=4,pos=2$，分母為 $1,100$：

$$
PE(2)\approx(0.909297,-0.416147,0.019999,0.999800)
$$

若 $E=(0.2,0.8,-0.4,0.5)$ 已是要送入相加步驟的 embedding，則：

$$
E+PE(2)\approx(1.109297,0.383853,-0.380001,1.499800)
$$

原論文還先將 embedding lookup 乘以 $\sqrt{d_{model}}$。若此處 $E$ 指未縮放 lookup，在 $d_{model}=4$ 時應算 $2E+PE(2)$，得到 $(1.309297,1.183853,-0.780001,1.999800)$。兩種例子只差在 $E$ 的定義。

### 固定位置差對應固定旋轉

令 $\omega_i=10000^{-2i/d_{model}}$，以 $(\sin,\cos)$ 順序寫成 column vector：

$$
\begin{bmatrix}\sin(\omega_i(pos+k))\\\cos(\omega_i(pos+k))\end{bmatrix}
=
\begin{bmatrix}\cos(\omega_i k)&\sin(\omega_i k)\\-\sin(\omega_i k)&\cos(\omega_i k)\end{bmatrix}
\begin{bmatrix}\sin(\omega_i pos)\\\cos(\omega_i pos)\end{bmatrix}
$$

位置與距離可以分兩層理解：位置 $pos$ 決定每個頻率的角度 $\theta_i=\omega_i pos$；兩位置 $a,b$ 的角度差為 $\omega_i(b-a)$。模型收到的是多組 sin/cos 座標，不是單獨的角度值。以 $(\cos,\sin)$ 或 $(\sin,\cos)$ 表示都可以，但旋轉矩陣的排列與正負號必須配合；本篇採原論文的 $(\sin,\cos)$ 維度順序。

固定 $k$ 的變換不依賴 $pos$。單一頻率會週期重複；多個頻率共同提供位置訊息，不應理解成單一角度就能唯一代表任意距離。此處是加到 embedding 的 sinusoidal PE，不能與將旋轉施加到 Q/K 的 RoPE 混為一談。

## 第六張：矩陣運算補充

右側照片是一個三角恆等式搭配矩陣乘法的練習，未見它與 attention 的直接推導關係。先把可辨識的兩矩陣記為 $M,T$，不從手寫的逆矩陣標記推測未提供的原矩陣：

$$
M=\begin{bmatrix}s&c&-1\\c&s&-2sc\\s&c&2\end{bmatrix},\quad
T=\begin{bmatrix}1&0&s\\0&1&c\\0&0&1\end{bmatrix},\quad
s=\sin\theta,\ c=\cos\theta
$$

$$
MT=\begin{bmatrix}s&c&s^2+c^2-1\\c&s&cs+sc-2sc\\s&c&s^2+c^2+2\end{bmatrix}
=\begin{bmatrix}s&c&0\\c&s&0\\s&c&3\end{bmatrix}
$$

使用了 $s^2+c^2=1$ 與 $\sin2\theta=2sc$。左側 logistic regression 另整理於 [[Logistic Regression 與 Log-Odds 計算]]。

## 手寫原稿

> [!note]- 照片 1：手寫原稿
> ![[Assets/Note/Research/Papers/Attention Is All You Need/Photo 1.jpg|800]]

> [!note]- 照片 2：手寫原稿
> ![[Assets/Note/Research/Papers/Attention Is All You Need/Photo 2.jpg|800]]

> [!note]- 照片 3：手寫原稿
> ![[Assets/Note/Research/Papers/Attention Is All You Need/Photo 3.jpg|800]]

> [!note]- 照片 4：手寫原稿
> ![[Assets/Note/Research/Papers/Attention Is All You Need/Photo 4.jpg|800]]

> [!note]- 照片 5：手寫原稿
> ![[Assets/Note/Research/Papers/Attention Is All You Need/Photo 5.jpg|800]]

> [!note]- 照片 6：手寫原稿
> ![[Assets/Note/Research/Papers/Attention Is All You Need/Photo 6.jpg|800]]

> [!note]- 照片 7：手寫原稿
> ![[Assets/Note/Research/Papers/Attention Is All You Need/Photo 7.jpg|800]]

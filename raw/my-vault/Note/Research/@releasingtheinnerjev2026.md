---
id: releasingtheinnerjev2026
type: manuscript
title: "Releasing the Inner Jev: A Simple Recipe for Open Decision Models"
issued:
  date-parts:
    - - 2026
URL: https://arxiv.org/abs/2610.17205v1
DOI: 10.48550/arXiv.2610.17205
container-title: arXiv
language: en
author:
  - family: Geuter
    given: Jonathan
  - family: Cheng
    given: Andrew
  - family: Alvarez-Melis
    given: David
year: "2026"
dateCreated: 2026-10-30
reading-status: reading
aliases:
  - "Releasing the Inner Jev: A Simple Recipe for Open Decision Models"
  - Jeff 1.0
  - OpenDecisions
tags:
  - literature_note
attachment:
  - "[[raw/my-vault/Assets/Note/Research/Papers/Releasing the Inner Jev - A Simple Recipe for Open Decision Models.pdf|PDF]]"
---
# Releasing the Inner Jev: A Simple Recipe for Open Decision Models

## TL;DR

- **直接讀取模型對 A/B/C 等選項字母的 logits，就能一次輸出各選項機率；校準後，Gemma 4 31B 的 Capability 為 79.6，接近 Jev 的 80.1。**
- **LoRA 搭配可訓練的 decision head 修正項後，Gemma 4 31B 三次訓練平均升至 84.4；消融顯示主要增益來自 LoRA，沒有明確證據顯示訓練 head 會額外提升。**
- **主結果比較 OpenDecisions 與 JevBench-Open；前者是作者用來選訓練配方的驗證集，後者是 JevBench 公開的 231 題。論文也在其他公開資料集上檢查泛化。**

## Research Question and Conclusion

### Overall in this paper

作者要證明的是：能不能用開放權重的語言模型，做出一個接近 Jev-style 的決策模型。

**共同的輸入與輸出：** 提供狀態／情境、問題和有限選項，模型一次讀取後，對 A、B、C 等選項輸出機率。選項先由使用者提供，模型負責比較選項，不負責搜尋候選答案。

論文比較兩條路：

1. **不微調：** 直接從原語言模型 head 讀出 A／B／C 的 logits，再做 softmax 和溫度校準。Gemma 4 31B 在 OpenDecisions 得 79.6，接近 Jev 的 80.1。
2. **微調：** 用 LoRA 調整模型表示，同時訓練一個從原字母輸出列開始的 decision head 修正項。這是 Jeff-1.0-Large 的主要配方；Gemma 4 31B 三次訓練平均在 OpenDecisions 得 84.4。

**所以 Jeff-1.0-Large 有 decision head 修正項。** 消融裡「凍結 head」的版本只是沒有更新這個額外修正項；它仍用原本的答案字母輸出 head。該版本分數和主配方接近，代表 LoRA 是主要增益來源，沒有證據顯示額外訓練 head 一定能再加分。

機率也要可信：若模型常給某一答案 0.8 的信心，長期來看這群答案大約應有八成正確。論文把這種機率可信度（Calibration）和答案表現（Intelligence）一起納入 Capability。

### 主要評估結果

![[Assets/Note/Research/Releasing the Inner Jev - A Simple Recipe for Open Decision Models/01-主要評估結果-OpenDecisions與JevBench-Open.png]]

OpenDecisions 有 2,352 題；JevBench-Open 是 JevBench 公開的 231 題子集。論文也在 7 個未納入訓練的公開決策資料集（共 9,212 題）評估，並報告 6 個一般知識選擇題基準。官方 JevBench v1.7 排行榜的 81.5（Jeff-1.0-Large）與 77.1（Jev）來自另一個 1,500 題測試抽樣，不是上表的 231 題 JevBench-Open 結果。

### Held-out Public Decision Datasets

這七組是 OpenDecisions 與 JevBench-Open 以外的公開測試資料，用來檢查泛化；題數是論文實際評估的樣本數。表中數字是準確率（%），Jeff-1.0-Large 的訓練結果採 seed 1。

| 測試集                                                    |        題數 | 測試內容                    | Gemma 4 31B 零樣本 | Jeff-1.0-Large | Jev 1.13.0 |
| ------------------------------------------------------ | --------: | ----------------------- | --------------: | -------------: | ---------: |
| [TREC](https://aclanthology.org/C02-1150/)             |       500 | 問句類型分類，例如人物、地點或數量       |            92.8 |           92.8 |       92.4 |
| [CB（CommitmentBank）](https://arxiv.org/abs/1905.00537) |        56 | 自然語言推論：判斷前提與假設的關係       |            85.7 |           87.5 |       82.1 |
| [WANLI](https://arxiv.org/abs/2201.05955)              |       256 | 自然語言推論；論文採用 SemIf 的抽樣   |            78.9 |           78.5 |       76.6 |
| [Auth.（SemIf authored）](https://github.com/TheoLeeCJ/SemIf-OpenJev)                                  |       144 | SemIf 公開的作者撰寫決策題        |            95.1 |           95.8 |       97.2 |
| [Pert.（SemIf perturbation）](https://github.com/TheoLeeCJ/SemIf-OpenJev)                              |       108 | SemIf 公開決策題的擾動版本        |            97.2 |          100.0 |      100.0 |
| [TypeSafe（論文描述）](https://arxiv.org/html/2610.17205v1)                                               |       348 | 公開決策題；原文未列這 348 題的獨立下載連結，與最可能參考答案比對 |            87.5 |           85.8 |       87.0 |
| [decider](https://github.com/Mapika/decider)                                                |     7,800 | 29 個從訓練混合資料中保留的分類任務     |            81.9 |           82.8 |       80.6 |
| **Macro（七組等權平均）**                                      | **9,212** | **七個資料集的準確率等權平均**       |        **88.4** |       **89.0** |   **88.0** |

平均 ECE（TypeSafe 不計入；越低越好）：Gemma 4 31B 零樣本為 0.077、Jeff-1.0-Large 為 0.047、Jev 為 0.066。論文提醒，Jev 在 Auth. 得 97.2，但這組公開資料可能曾出現在 Jev 的訓練資料中。

## Methodology

### Method 1：零樣本 Letter Readout

把狀態、問題與選項整理成多選題提示，選項依序標成 A、B、C 等。模型不生成解釋文字，而是取最後位置對應選項字母的 logits，僅對有效選項做 softmax：

$$
p_k=\frac{\exp(z_k)}{\sum_{j=1}^{K}\exp(z_j)}
$$

因此一次 forward pass 就能取得 K 個答案機率。論文以每種問題類型各自的 temperature scaling 降低過度自信；溫度縮放機率，不改變最高分選項。Gemma 4 31B 校準後在 OpenDecisions 的 Capability 為 79.6，Jev 為 80.1。

#### 範例：從最後位置取出 A／B／C 的 logits

把完整 prompt 輸入模型後，取最後一個 prompt 位置的輸出 logits；這是一個涵蓋整個詞彙表的向量。Tokenizer 可把答案字母對應到 token ID；再用這些 ID 從全詞彙 logits 向量取出 A、B、C 的分數：

```python
letters = ["A", "B", "C"]
vocab_logits = outputs.logits[0, -1, :]  # 最後 prompt 位置、整個詞彙表的 logits
encoded = {
    letter: tokenizer.encode(letter, add_special_tokens=False)
    for letter in letters
}
assert all(len(ids) == 1 for ids in encoded.values())

letter_to_id = {letter: ids[0] for letter, ids in encoded.items()}
abc_logits = [vocab_logits[letter_to_id[letter]] for letter in letters]
```

實際 token ID 由模型的 tokenizer 決定，不能假設每個模型都相同；論文使用的答案字母各自是一個 token。以 Appendix A 的客服單選項為例，對應 logits 是 A（billing）= 26.41、B（technical）= 19.65、C（refund）= 23.77。先對三個值做 softmax：

$$
\begin{aligned}
z &= [26.41,19.65,23.77] \\
p &= \mathrm{softmax}(z) \\
  &= \frac{[e^{26.41-26.41},e^{19.65-26.41},e^{23.77-26.41}]}{e^0+e^{-6.76}+e^{-2.64}} \\
  &\approx [0.933,0.001,0.066]
\end{aligned}
$$

這是未做 temperature scaling 的 raw probability。該題是 choice 類型，論文使用 $T=1.52$；將 logits 除以溫度後再做 softmax：

$$
\mathrm{softmax}(z/T)
=\mathrm{softmax}([26.41,19.65,23.77]/1.52)
\approx[0.842,0.010,0.148]
$$

注意：這組客服單 logits 來自微調後的 Jeff-1.0-Large，是用來展示「取出 A/B/C logits → softmax → 溫度校準」的計算；零樣本版本使用相同的 token 讀取流程，但直接取預訓練語言模型 head 對應字母的 logits，不包含微調後的 decision head 修正。

### Method 2：LoRA + Decision Head 微調
![[Assets/Note/Research/Releasing the Inner Jev - A Simple Recipe for Open Decision Models/LoRA與Decision Head微調流程.png]]

保留預訓練模型的答案字母讀取作為起點，加入可訓練的決策修正項，並同時訓練 LoRA adapter：

$$
z=W_0h+A\,\mathrm{std}(h)+b
$$

其中 $h$ 是最後位置的隱藏向量，$W_0$ 來自語言模型原本的字母輸出列，$A$ 與 $b$ 是從零開始學習的 head 修正項。

### Gemma 4 31B 訓練設定

| 參數                     | 設定                                                                 |
| ---------------------- | ------------------------------------------------------------------ |
| 起始模型                   | 指令微調版 Gemma 4 31B；原本語言模型權重凍結                                       |
| 訓練方法                   | 監督式微調，同時訓練 LoRA adapter 與 decision head 修正項                        |
| LoRA                   | rank 32、$\alpha=64$、dropout 0；套用在 attention 與 MLP 投影層              |
| Decision head          | 使用預訓練模型 A–P 字母輸出列作為固定的 $W_0$；可訓練修正項 $A,b$ 從 0 初始化                  |
| 損失與標籤                  | 交叉熵；有軟標籤時對目標機率分布計算，否則對正確答案計算；訓練時打亂選項順序                             |
| Optimizer              | AdamW                                                              |
| Learning rate          | LoRA：$2\times10^{-5}$；decision head：$1\times10^{-4}$               |
| Learning rate schedule | 前 250 steps warmup，接著維持至 step 300，再衰減至 step 400 的 0                |
| 訓練長度與每步資料量             | 400 steps；每步 65,536 padded tokens，約 190 題；整次約處理 76,000 題（訓練池的 18%） |
| Training pool            | 417,825 題，其中 117,783 題使用軟標籤；OpenDecisions 題目不納入訓練                  |
| 計算資源                   | 4 張 H200，約 78 分鐘（約 5 GPU-hours）                                    |

#### 計算範例（示意數字）

假設只有 A、B、C 三個選項，模型最後位置的隱藏向量是 $h=[3,6]^T$。假設訓練資料統計出的平均值為 $\mu=[1,2]^T$、標準差為 $\sigma=[2,4]^T$：

$$
\mathrm{std}(h)=\frac{h-\mu}{\sigma}
=\left[\frac{3-1}{2},\frac{6-2}{4}\right]^T
=[1,1]^T
$$

再假設這個玩具例子的權重為：

$$
W_0=\begin{bmatrix}0.4&0.1\\0.1&0.3\\0.2&0.1\end{bmatrix},\quad
A=\begin{bmatrix}0.1&0\\0&-0.1\\-0.05&0.05\end{bmatrix},\quad
b=\begin{bmatrix}0\\0\\0\end{bmatrix}
$$

先計算原本答案字母 head 的分數：

$$
W_0h=\begin{bmatrix}
0.4\times3+0.1\times6\\
0.1\times3+0.3\times6\\
0.2\times3+0.1\times6
\end{bmatrix}
=\begin{bmatrix}1.8\\2.1\\1.2\end{bmatrix}
$$

再計算訓練學到的修正：

$$
A\,\mathrm{std}(h)+b
=\begin{bmatrix}0.1\\-0.1\\0\end{bmatrix}
$$

兩者相加得到三個選項的 logits：

$$
z=W_0h+A\,\mathrm{std}(h)+b
=\begin{bmatrix}1.8\\2.1\\1.2\end{bmatrix}
+\begin{bmatrix}0.1\\-0.1\\0\end{bmatrix}
=\begin{bmatrix}1.9\\2.0\\1.2\end{bmatrix}
$$

最後對 logits 做 softmax：

$$
\mathrm{softmax}(z)
=\frac{[e^{1.9},e^{2.0},e^{1.2}]}{e^{1.9}+e^{2.0}+e^{1.2}}
\approx[0.384,0.425,0.191]
$$

所以模型選 B，機率約為 0.425。若標準答案是 B，one-hot 標籤為 $y=[0,1,0]$，這筆資料的交叉熵為：

$$
L=-\sum_i y_i\log p_i=-\log(0.425)\approx0.856
$$

這些權重與數字只用來示範計算，不是論文公布的模型權重或某題實際輸出。真實模型會用訓練資料估計標準化統計量，並透過交叉熵更新決策 head 與 LoRA 參數。

#### Ablation Results

![[Assets/Note/Research/Releasing the Inner Jev - A Simple Recipe for Open Decision Models/03-消融實驗設定比較.png]]
![[Assets/Note/Research/Releasing the Inner Jev - A Simple Recipe for Open Decision Models/04-訓練步數與Capability.png]]

Gemma 4 31B 使用 LoRA 後，OpenDecisions Capability 從零樣本的 79.6 提升到三次訓練平均 84.4。消融顯示，LoRA 是主要提升來源；額外訓練 decision head 修正項、調整 rank 或標籤形式，都沒有呈現明確而穩定的額外收益。400–800 steps 的結果接近，論文採用 400 steps 作為主。

## OpenDecisions 驗證流程

### 資料如何建立

| 項目     | 設計                                                              |
| ------ | --------------------------------------------------------------- |
| 規模     | 2,352 題、11 個題族、23 種語言、4 個難度層級                                   |
| 題型     | 選擇題 1,178 題、是非題 586 題、順序分級題 588 題                               |
| 程式產生   | 599 題，由程式產生並精確計算答案                                              |
| 模型撰寫   | 1,443 題由模型撰寫，再交由三個模型組成的標註 panel 核對；全數同意，或至少兩個同意且平均答案機率達 0.5，才保留 |
| 公開真實請求 | 310 題取自公開資料集；三個標註模型必須全部同意才保留                                    |

選項順序或一個細節不同的成對題目，只有在成組題目都通過核對時才保留。作者也將候選題與訓練資料及公開 JevBench 題目比對，移除 43 題重疊項目。

### 如何計分與交叉檢查

- **Capability = (Intelligence + Calibration) / 2**。Intelligence 是扣除隨機猜測表現後、按題型與難度分層計算的準確率；Calibration 根據各題型的 ECE／RPS 衡量預測機率與實際表現是否相符。
- **ECE（Expected Calibration Error，期望校準誤差）**：按預測信心把題目分組，比較每組的平均信心和實際正確率。==例如一組題平均信心是 0.8、實際答對率是 0.6，該組的差距是 0.2；把各組差距加權平均後得到 ECE，越低越好==。
- **RPS（Ranked Probability Score，排序機率分數）**：用於有順序的分數選項，評估整個機率分布與實際分數的差距。它會考慮選項的遠近：==真實結果是「高」時，把機率放在「中」通常比放在「低」錯得輕；RPS 越低越好==。
- **Calibration 的直覺例子：** 如果模型在很多題都給最高選項 0.8 的機率，這一群題長期大約八成答對，才算信心和實際表現相符。若它常報 0.99、實際卻只答對七成，就屬於過度自信。Calibration 衡量機率可信度；Intelligence 衡量答案表現，兩者不是同一件事。
- 95% 區間合併題目抽樣誤差與訓練 seed 差異；抽樣以題組為單位做 1,000 次 paired bootstrap。
- 14 個公開決策模型在 OpenDecisions 與 JevBench 上的排名，Spearman 相關為 0.90，支持這份資料大致能反映排行榜排序。





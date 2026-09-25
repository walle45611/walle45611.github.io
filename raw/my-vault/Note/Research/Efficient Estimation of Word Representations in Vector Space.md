# Efficient Estimation of Word Representations in Vector Space

[[Assets/Note/Research/Papers/Efficient Estimation of Word Representations in Vector Space/Efficient Estimation of Word Representations in Vector Space.pdf|論文 PDF]]

## 初讀摘要

- Word2Vec 的原始工作提出高效率的詞向量學習架構，讓相似語境中的詞具有可比較的連續表示。
- CBOW 由上下文預測中心詞，Skip-gram 由中心詞預測周圍詞，藉由簡化模型降低大語料訓練成本。
- 原文以語法與語意關係測試展示表示品質。這裡的詞向量是詞彙層級的固定表示；negative sampling 與片語擴充主要由後續論文進一步提出。

摘要依據：PDF 摘要與導論。


> 論文來源：*Efficient Estimation of Word Representations in Vector Space*，arXiv:1301.3781v3 [cs.CL]

> [!abstract] 核心概念
> Word2Vec 從 words 周圍的 contexts 學習低維、dense 的 distributed representations。
>
> $$
> \boxed{\text{Context similarity} \rightarrow \text{Vector similarity}}
> $$

## 目錄

- [[#1. From One-hot to Distributed Representation|From One-hot to Distributed Representation]]
- [[#2. CBOW and Skip-gram|CBOW and Skip-gram]]
- [[#3. Network and Training|Network and Training]]
- [[#4. Cosine Similarity|Cosine Similarity]]
- [[#5. Computational Efficiency|Computational Efficiency]]
- [[#6. Limitations and Evolution|Limitations and Evolution]]
- [[#7. Whole Picture|Whole Picture]]

---

## 1. From One-hot to Distributed Representation

**Why Word2Vec?**

在 1990s–2000s，N-gram language models 被廣泛使用。基本概念是利用前面的幾個 words 預測下一個 word：

$$
P(w_t \mid w_{t-n+1}, \dots, w_{t-1})
$$

隨著 training data 增加，N-gram language models 的表現可以持續改善，但也逐漸遇到限制：

- Vocabulary 很大時，可能的 word combinations 數量非常巨大。
- 很多 N-gram 在 training corpus 中從來沒有出現過。
- 無法很好地表示 words 之間的 semantic similarity。
- 每個 word 基本上被視為獨立的 symbol。

例如 `cat` 和 `dog` 對人類來說語意很接近，但傳統 symbolic representation 並不知道它們之間的關係。

因此開始使用 **Distributed Representation**：把 word 表示成 dense vector。這進一步發展成 **Word Embedding**，而 Word2Vec 就是其中一種學習 word embeddings 的方法。

---

**One-hot Encoding**

假設 vocabulary：

$$
V = \{cat, dog, apple, car\}
$$

每個 word 可以表示成 dimension 等於 vocabulary size 的 one-hot vector：

$$
cat=[1,0,0,0], \qquad dog=[0,1,0,0]
$$

$$
apple=[0,0,1,0], \qquad car=[0,0,0,1]
$$

$$
x \in \mathbb{R}^{|V|}
$$

**Problem**

> [!warning] One-hot encoding 不包含 semantic information
> One-hot vector 當然可以計算 cosine similarity，但不同 words 的 vectors 互相正交，因此幾乎都得到 0，沒有語意上的實用價值。

例如：

$$
\cos(cat,dog)=0, \qquad \cos(cat,car)=0
$$

所以在 one-hot representation 中，`cat` 跟 `dog` 並不會比 `cat` 跟 `car` 更相似。

---

**Distributed Representation**

Word2Vec 的核心概念是將每個 word 學成一個低維、dense 的 vector。例如原本的高維表示：

$$
cat \in \mathbb{R}^{100000}
$$

可能變成：

$$
v_{cat} \in \mathbb{R}^{300}
$$

簡化成 2 維表示：

$$
v_{cat}=[0.7,0.8], \qquad v_{dog}=[0.6,0.75]
$$

$$
v_{car}=[-0.4,0.1]
$$

這時 `cat` 和 `dog` 可能會在 vector space 中比較接近。

> Words with similar contexts tend to have similar vector representations.

這也就是 Word2Vec 背後的重要概念：

> You shall know a word by the company it keeps.

一個 word 的 meaning，可以部分從它周圍經常出現的 words 得知。

---

**Word2Vec**

Word2Vec 是一種 neural network-based word embedding 方法。它不會由人工直接指定 `cat` 應該靠近 `dog`，而是透過大量 corpus 中的 **word-context relationships** 自動學習 embedding。

Word2Vec 主要有兩種 training objectives：

1. **CBOW**
2. **Skip-gram**

---

## 2. CBOW and Skip-gram

**CBOW**

CBOW 是 **Continuous Bag of Words**。

> [!info] CBOW
> 使用 surrounding context words 預測 center word。

例如：

```text
A cute ___ is reading
```

假設 center word 是 `boy`，context words 可以是 `A`、`cute`、`is`、`reading`：

```text
context words
      ↓
    CBOW
      ↓
 center word
```

$$
\text{context} \rightarrow \text{target}
$$

**CBOW 的 Context Representation**

假設 context 有 $N$ 個 words，每個 word embedding 為：

$$
v_i \in \mathbb{R}^{D}
$$

例如：

$$
v_{\text{I}}=[0.2,0.4], \qquad v_{\text{coffee}}=[0.6,0.8]
$$

CBOW 可以把 context vectors 做平均：

$$
h=\frac{1}{N}\sum_{i=1}^{N}v_i
$$

因此：

$$
h=\frac{[0.2,0.4]+[0.6,0.8]}{2}=[0.4,0.6]
$$

這個 $h\in\mathbb{R}^{D}$ 就是 context representation，接著利用 $h$ 預測 center word。

---

**Skip-gram**

Skip-gram 剛好跟 CBOW 相反。

> [!info] Skip-gram
> 使用 center word 預測 surrounding context words。

例如：

```text
A cute teddy bear is reading
```

如果 `teddy bear` 是 center word，模型可能需要預測附近的 `A`、`cute`、`is`、`reading`：

```text
 center word
      ↓
  Skip-gram
      ↓
context words
```

$$
\text{target} \rightarrow \text{context}
$$

---

**CBOW vs. Skip-gram**

![[Assets/Note/Research/Papers/Efficient Estimation of Word Representations in Vector Space/Word2Vec CBOW vs Skip-gram.png|578]]

*CBOW 使用周圍 words 預測 center word；Skip-gram 使用 center word 預測周圍 words。*

| 比較項目 | CBOW | Skip-gram |
| --- | --- | --- |
| Input | Context words | Center word |
| Output | Center word | Context words |
| Direction | Context → Target | Target → Context |
| Training samples | 較少 | 較多 |
| Speed | 通常較快 | 通常較慢 |
| Rare words | 相對較弱 | 通常較好 |

> [!tip] 最簡單的記法
> - **CBOW：**周圍 → 中間
> - **Skip-gram：**中間 → 周圍

---

## 3. Network and Training

**Word2Vec Network**

假設：

- Vocabulary size = $V$
- Embedding dimension = $D$

Word2Vec 可以想成：

```text
One-hot word
     ↓
Input Embedding Matrix
     ↓
Dense embedding
     ↓
Output Layer
     ↓
Vocabulary probability
```

Input embedding matrix：

$$
W \in \mathbb{R}^{V\times D}
$$

如果 word $w_i$ 的 one-hot vector 是 $x_i$，則：

$$
h=x_i^T W
$$

因為 $x_i$ 是 one-hot vector，這實際上就是從 embedding matrix $W$ 中取出第 $i$ 列：

$$
h=v_{w_i}
$$

因此 implementation 不需要真的建立超大的 one-hot vector，而是直接進行 embedding lookup：

```text
word ID → embedding lookup → word vector
```

---

**Output Layer 與訓練**

假設 hidden representation 為 $h\in\mathbb{R}^{D}$，輸出 vocabulary 有 $V$ 個 words，則 output matrix 為：

$$
W' \in \mathbb{R}^{D\times V}
$$

計算：

$$
u=hW', \qquad u\in\mathbb{R}^{V}
$$

再透過 softmax 得到每一個 vocabulary word 的 probability：

$$
P(w_j\mid w_i)=\frac{\exp(u_j)}{\sum_{k=1}^{V}\exp(u_k)}
$$

Training 的目的就是讓正確的 context / target word probability 越來越高：

```text
Forward
   ↓
Loss
   ↓
Backpropagation
   ↓
Optimizer
   ↓
Update W
```

一開始 $W$ 是 random initialization。經過大量 corpus training 後，$W$ 逐漸學會 semantic structure。

---

**為什麼 Word2Vec 可以學到相似度？**

假設 corpus 中有：

```text
I drink coffee
I drink tea

hot coffee
hot tea

coffee tastes good
tea tastes good
```

`coffee` 和 `tea` 經常出現在類似的 context，因此在 training 過程中會收到類似的 gradient updates。久而久之：

$$
v_{\text{coffee}} \approx v_{\text{tea}}
$$

它們在 embedding space 中就會比較接近。

> Semantic similarity 並不是人工設定的，而是從 context distribution 中學出來的。

---

## 4. Cosine Similarity

學到 word vectors 後，可以用 cosine similarity 比較兩個 vectors 的方向。假設 $u,v\in\mathbb{R}^{D}$：

$$
\cos(u,v)=\frac{u^T v}{\|u\|\|v\|}
$$

其中 $u^T v$ 是 dot product，$\|u\|$ 與 $\|v\|$ 是 vector magnitude。

**Interpretation**

| Cosine similarity | 意義 |
| --- | --- |
| 接近 $1$ | 方向接近 |
| 接近 $0$ | 接近 orthogonal |
| 接近 $-1$ | 方向相反 |

因此可以使用：

$$
\cos(v_{\text{cat}},v_{\text{dog}})
$$

來衡量 `cat` 與 `dog` 的 embedding 有多相似。

| Representation | `cat` vs. `dog` | `cat` vs. `car` |
| --- | --- | --- |
| One-hot | $0$ | $0$ |
| Word2Vec | high similarity | lower similarity |

---

## 5. Computational Efficiency

**Computational Complexity**

早期 Neural Language Model 的一個重要問題是 vocabulary 太大，使 output softmax 的 computational cost 很高。

假設：

- $N$ = context words
- $D$ = embedding dimension
- $H$ = hidden layer size
- $V$ = vocabulary size

傳統 Neural Network Language Model 的計算量可以近似寫成：

$$
Q=N\times D+N\times D\times H+H\times V
$$

真正麻煩的是：

$$
H\times V
$$

因為 $V$ 可能有幾十萬甚至更多 words。

---

**Hierarchical Softmax**

Hierarchical Softmax 是一種降低 output computation 的方法：把 vocabulary 建成一棵 binary tree。

原本需要比較 $V$ 個 vocabulary words；現在只需要沿著 tree 從 root 走到 target word。Binary tree 的 path length 大約是：

$$
\log_2(V)
$$

所以 computation 可以由：

$$
O(V) \rightarrow O(\log V)
$$

Word2Vec 常搭配 **Huffman Tree**，讓出現頻率較高的 words 有比較短的 path，因此 average computational cost 可以進一步降低。

---

## 6. Limitations and Evolution

**Static Embeddings**

Word2Vec 的一個重要限制是每一個 word 只有一個固定 embedding。

```text
I deposited money in the bank.
I sat on the river bank.
```

這兩個 `bank` 的 meaning 不一樣，但 Word2Vec 中只有一個 $v_{\text{bank}}$：

> Word2Vec learns static word embeddings.

它沒有辦法像 Transformer 一樣，根據 sentence context 動態改變 representation。Transformer 則會得到：

$$
h_{\text{bank}}^{(sentence1)} \neq h_{\text{bank}}^{(sentence2)}
$$

因此稱為 **Contextualized Representation**。

---

**Language and Morphology**

不同 language 的 vocabulary、morphology 和 corpus distribution 都不同。

Inflectional language 是指：

> A language in which the form or ending of a word changes depending on how it is used in a sentence.

例如英文：

```text
cat → cats
walk → walked
```

因此不同語言通常需要使用對應語言的 corpus 來 train Word2Vec：

```text
English corpus → English Word2Vec
Chinese corpus → Chinese Word2Vec
```

不能期待只用大量 English corpus 訓練的 Word2Vec，自然就學會完整的 Chinese semantic relationships。

---

**Limitations of Word2Vec**

1. **Static word embedding**  
   同一個 word 無論 context 如何，embedding 都相同。

2. **OOV（Out-of-Vocabulary）**  
   沒有出現在 vocabulary 裡的 word 通常沒有 embedding。

3. **Word-level representation**  
   傳統 Word2Vec 主要以完整 word 作為單位，對 morphology 的處理能力有限。

4. **Context window 有限**  
   主要從附近的 words 學習 semantic information。

5. **不是真正理解 sentence structure**  
   Word2Vec 學的是 statistical co-occurrence relationships，而不是完整的 syntax / long-range context。

這些問題後來也促使其他 representation methods 發展：

```text
Word2Vec
   ↓
GloVe / FastText
   ↓
ELMo
   ↓
BERT / GPT / Transformer
```

---

## 7. Whole Picture

```text
Corpus
   ↓
建立 word-context training pairs
   ↓
CBOW / Skip-gram
   ↓
Neural Network Training
   ↓
Update Embedding Matrix
   ↓
Dense Word Embeddings
   ↓
Similar contexts → Similar vectors
   ↓
Cosine Similarity
```

核心思想：

$$
\boxed{\text{Context similarity} \rightarrow \text{Vector similarity}}
$$

> Word2Vec learns distributed representations of words from their surrounding contexts.

---

**最重要的幾句**

**One-hot**

$$
\boxed{\text{One-hot encoding does not capture semantic similarity.}}
$$

**Word2Vec**

$$
\boxed{\text{Word2Vec learns dense distributed word representations.}}
$$

**CBOW**

$$
\boxed{\text{Context} \rightarrow \text{Center Word}}
$$

**Skip-gram**

$$
\boxed{\text{Center Word} \rightarrow \text{Context}}
$$

**Word similarity**

$$
\boxed{\cos(u,v)=\frac{u^T v}{\|u\|\|v\|}}
$$

**Fundamental intuition**

$$
\boxed{\text{Words used in similar contexts tend to obtain similar vectors.}}
$$

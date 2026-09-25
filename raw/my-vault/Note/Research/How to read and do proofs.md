
## Forward / Backward Process 與數學術語筆記

在證明一個命題時，通常可以把它看成：

$$
A \Rightarrow B
$$

其中 $A$ 是 **hypothesis**，也就是已知條件；$B$ 是 **conclusion**，也就是要證明的結論。

**Backward process** 是先看 conclusion $B$，然後問一個 **key question**：

> How can I show that $B$ is true?

也就是先從「我要證明什麼」開始想。它不是一開始就做計算，而是先把目標變成一個可以被回答的問題。

例如要證明：

$$
\text{If } n \text{ is even, then } n^2 \text{ is even.}
$$

這裡的 conclusion 是：

$$
B: n^2 \text{ is even}
$$

所以 key question 可以問：

> How can I show that an integer, namely $n^2$, is even?

意思是：我要怎麼證明一個整數，也就是 $n^2$，是偶數？

**Forward process** 是從 hypothesis $A$ 開始，利用 definition、axiom、theorem、lemma 或已知事實推出新的 statement。

例如 hypothesis 是：

$$
A: n \text{ is even}
$$

根據 even 的 definition：

$$
n = 2k
$$

for some integer $k$。

接著可以推導：

$$
n^2 = (2k)^2 = 4k^2 = 2(2k^2)
$$

因為 $2k^2$ 是整數，所以 $n^2$ 可以表示成 $2$ 乘以某個整數，因此 $n^2$ is even。這就回答了 backward process 裡的 key question。

所以在 F/B method 裡，可以這樣理解：

Backward process：從 $B$ 產生 key question。

Forward process：從 $A$ 出發，使用 definition 或其他已知結果產生新的 statement，直到可以回答 key question。

整個證明會靠 implication 的遞移性接起來：

$$
A \Rightarrow A_1 \Rightarrow A_2 \Rightarrow \cdots \Rightarrow B
$$

也就是如果：

$$
A \Rightarrow A_1
$$

且：

$$
A_1 \Rightarrow A_2
$$

且：

$$
A_2 \Rightarrow B
$$

那麼可以得到：

$$
A \Rightarrow B
$$

---

**Proposition（命題）** 是一個你正在證明、且值得關注的 true statement。

英文定義：

> A proposition is a true statement of interest that you are proving.

例如：

$$
\text{If } n \text{ is even, then } n^2 \text{ is even.}
$$

這就是一個 proposition。

**Theorem（定理）** 是重要的 proposition。

英文定義：

> A theorem is an important proposition.

Theorem 通常比 proposition 更重要，常常是某個章節或理論的主要結果。

**Lemma（引理）** 是用來證明 theorem 的 proposition。

英文定義：

> A lemma is a proposition that is used in the proof of a theorem.

Lemma 本身也要證明，但它的主要功能是幫助證明更大的 theorem。

**Corollary（推論）** 是幾乎可以直接從 theorem 推出來的 proposition。

英文定義：

> A corollary is a proposition that follows almost immediately from a theorem.

Corollary 的證明通常很短，因為它是 theorem 的直接結果。

**Axiom（公理）** 是不用證明，直接被接受為真的 statement。

英文定義：

> An axiom is a statement that is accepted as being true without proof.

例如：

> The shortest distance between two points is a straight line.

**NOT $A$** 也可以寫成：

$$
\sim A
$$

意思是 $A$ is false。

如果 $A$ 是 true，那麼 $\sim A$ 是 false。

如果 $A$ 是 false，那麼 $\sim A$ 是 true。

---
title: 資料結構-二元樹篇-1
date: 2024-08-26 23:31:59
tags: [Algorithm, Binary Tree]
categories: 資料結構
mathjax: true
---

![圖 1. : Blog Image](https://imgur.com/6Yyz7mV.png)

## Binary Tree 和 Tree 的討論

所有的樹是可以表示成二元樹的 (樹轉二元樹)，所以有很多人會把 Tree 跟 Binary Tree 搞混，有很重要的重點就是，樹不可以為空，但是 Binary Tree 可以為空，除此之外樹是沒有順序之分。

<!--more-->

* Tree 和 Binary Tree degree 也有不同，因為 Tree Node's $degree >= 0$，但是 Binary Tree Node's $0 <= degree <=2$。

* 像是以下這個圖就是一個例子，在 Binary Tree 中這兩個是不同的樹，但是在樹裡面就是相同的樹。
![圖 2. : different tree from binary tree](https://imgur.com/lNmehwp.png)

## Binary Tree 定義

* A binary tree is a finite set of nodes that is either empty or consists of a root and two disjoint binary trees called theleft subree an the right subtree. (也就是說二元樹可以是空，那麼由一個 root 和兩個不相交的兩個子樹，這兩個子樹分別為右子樹、左子樹)

## Binary Tree ADT

**結構**: `Binary_Tree` (簡寫為 `BinTree`)

**物件**: 一組有限的節點，可以是空的，或包含根節點、左子樹，以及右子樹。

$$
\forall bt, bt1, bt2 \in \text{BinTree}, \; item \in \text{element}
$$

### 函數定義

以下是最小的操作步驟也就是說你的一個 Binary Tree 需要這些操作才能正常運作，當然你的ADT可ㄧ有很多自定義的操作，但是以 `horowitz` 的書中是這樣寫的。

* **BinTree Create()** :
    建立一個空的二元樹

* **Boolean IsEmpty(bt)** :
    if (bt == 空二叉樹) 則返回 TRUE，否則返回 FALSE

* **BinTree MakeBT(bt1, item, bt2)** :
    建立一個二元樹，左子樹為 `bt1`，右子樹為 `bt2`，根節點包含資料項 `item`

* **BinTree Lchild(bt)** :
    if (IsEmpty(bt)) 則返回錯誤，否則返回 bt 的左子樹

* **element Data(bt)** :
    if (IsEmpty(bt)) 則返回錯誤，否則返回 bt 根節點中的資料

* **BinTree Rchild(bt)** :
    if (IsEmpty(bt)) 則返回錯誤，否則返回 bt 的右子樹

## Binary Tree Types

* *skewed* :

    ![圖 3. : skewed binary tree](https://imgur.com/PoIn0py.png)

* *complete binary tree* :
    用直白的話說 complete 其實就是你在 n-1 層是滿的且你在第 n 層是一定要按照 binary tree 的順序生成樹的不能有位置跳號的情況。

    ![圖 4. : complete binary tree](https://imgur.com/I3jugml.png)

  * *not complete binary tree* :

    ![圖 5. : not complete binary tree](https://imgur.com/YQOyIog.png)

  * *full binary tree* :
    這代表了是把所有編號都長滿那麼這個就是一個 full binary tree，那 full binary tree 也滿足 complete binary tree。

    ![圖 6. : full binary tree](https://imgur.com/yD6e0g4.png)

* *Strict binary tree* :
    Each non-leaf node will havw two children. There is no degree-1 nodes exist.

    ![圖 7. : Strict binary tree](https://imgur.com/0dtk7qI.png)

## Binary Tree 的個 Lemma

以下會介紹 Binary Tree 的 4 個 Lemma 其中有兩個屬於 Maxiumm number of nodes 的證明，和一個 Relation between number of leaf node and nodes of degree 2，和一個是從 Maxiumm number of nodes 變化出來的特別針對於 complete binary tree。

### Maxiumm number of nodes

1. The maximum number of nodes on level $i$ of a binary tree is $2^{i-1}$, $i >= 1$.

    * *Proof*:

        *Induction base* : 當這個 root node 是唯一的時候，因此第一層的最大節點數是 $1$, by $2^{1-1} = 2^0 = 1$。

        *Induction hypothesis* : $\forall j, 1 <= j < i$，第 $j$ 層對大節點樹為 $2^{j-1}$。

        *Induction step* : 根據 Induction hypothesis 所以可以知道最多 nodes 是在 $i-1$ 層得 $2^{i-2}$，那我們又可以知道每個節點最多有兩個子節點，而得知在第 $i$ 我們可以知道 $2^{i-1}$。

2. The maximum number of nodes in a binary tree of depth k is $2^k - 1$, $k >= 1$.

    * *Proof* :

        *Clam* :
        由 *Lemma 1* 我們可以知道每層最大的節點數目是 $2^{i-1}$ 那麼我們把所有層的最大總點數加總。

    $$
    \text{幾何級數公式 :} \quad \sum_{i=0}^{n-1} a \cdot r^i = a \cdot \frac{1 - r^n}{1 - r}
    $$

    $$
    \text{對應到此處 :} \quad a = 1, \, r = 2, \, n = k
    $$

    $$
    \text{因此, 我們有 :} \quad \sum_{i=1}^k 2^{i-1} = \frac{1 \cdot (2^k - 1)}{1} = 2^k - 1
    $$

    $$
    \text{所以 :} \quad \sum_{i=1}^k(每層最大節點數) = 2^k - 1
    $$

3. 如果有一個 complete binary tree 他有 n 個 nodes 那麼他的深度會是 $\lfloor log_2n + 1 \rfloor$
假設一棵完全二元樹有 $n$ 個節點，則這棵樹的高度可以表示為：

    * *Proof* :

        $$
        2^k - 1 \leq n < 2^{k+1} - 1
        $$

        對這個不等式兩邊取對數：

        $$
        \log_2(2^k - 1) \leq \log_2 n
        $$

        當 $k$ 很大時，$2^k - 1$ 可以近似為 $2^k$，因此：

        $$
        \log_2(2^k) \approx \log_2 n
        $$

        所以：

        $$
        k \approx \log_2 n
        $$

        由於樹的高度是從 1 開始計算的，所以最終的高度公式為：

        $$
        k = \lfloor \log_2(n+1) \rfloor  \, or \, \lfloor log_2n \rfloor + 1
        $$

* 以下很重要
  * $i$ is nodes index
  * *parent(i)* is at $\lfloor i/2 \rfloor , if \not ={1}, if = 1 \,\text{it is root}$
  * *left_child(i)* is at $2i,if \, 2i <= n$ 如果超過這個界線就是沒有 left_child。
  * *right_child(i)* is at $2i+1, if \, 2i+1 <= n$ 如果超過這個界線就是代表他沒有 right_child。

### Relation between number of leaf node and nodes of degree 2

在 Binary Tree 有一個現象，leaf 和 degree 2 的點數有些關係，這個證明滿重要的。

$\forall \, T \, \text{is a nonempty Binary Tree}$, if $n_0$ is leaf nodes and $n_2$ is two degree nodes, then $n_0 = n_2 + 1$.

* *Proof* :

    let $n_1$ is degree one nodes and n the is total number of nodes，那麼我們可以很清楚知道以下公式。

    $$
    n=n_0 + n_1 + n_2
    $$

    那麼我們又可以知道 $u=v-1 \Rightarrow n=B+1$, $B=barnch \, number$ 那麼我們又可以知道以下公式
    $$
    n=1+n_1 + 2n_2
    $$

    那麼我們由兩式整理得以下公式

    $$
    n_0+n_1+n_2 = 1 + n_1 + 2n_2
    \Rightarrow n_0 = 1 + n_2
    $$

### 總結

* Lemma 2 可以得知一個 full binary tree 他就會有 $2^k-1$ 個 nodes，這個應該滿好理解就不說了，在 `horowitz` 書中有寫到第 195 頁底下。

## Binary Tree representations

### Array method

* 需要 $2^k -1$ 個 Array 空間。
* 優點
    1. 容易存取 parent 畢竟 $\lfloor i/2 \rfloor$ 就可以存取到了。
    2. 對於 Full Binary tree 可以充分利用，沒有浪費
* 缺點
    1. 不容易刪除解點，但是 dynamic array 可以解決。
    2. 對於 skewed binary tree 浪費空間如果高度為`H`則浪費$2^{H}-1-H$，可以這樣想畢竟你每個點的數量就 $1$ 那麼當然就是減高度就可以知道了。
* 範例 `(A(B,E),C)` 為例

    ![圖 8. : Array 表示 Binary Tree](https://imgur.com/4pqOcBG.png){width=30%}

### Link list method

* 優點
  1. 容易新增和刪除節點
  2. 對於 skewed binary tree 比 Array 的方式省空間。
* 缺點
  1. 不容易存取父點。
  2. Link space 浪費約一半的空間。如果有 $N$ 個節點，則有 $N+1$ 條的 Nil link，由此 $2N-(N-1)$ 得知。
* 範例這邊就不畫了，手快斷掉了哈哈哈：

  ![圖 9. : Link list 表示 Binary Tree](https://imgur.com/RFaOm1L.png)

## Binary Tree Traversal

大家想想，*right subtree* 和 *left subtree* 和 *root* 有幾種走訪方式 `3!` 種對吧，但是為了方便，有的就不討論了，核心就是一定要 *left subtree* 先走，然後才走 *right subtre*，那麼這樣就分成了，*VLR (preorder)*、*LVR (inorder)*、*LRV (postorder)*，反正就看 *V* 在哪裡就是叫什麼樣的走法，那麼還有一個比較特別的 *Level-Order*，在 `horowitz` 書中有寫到那我覺得他就是 *BFS* 其實，那麼走訪的 complexity 都是 $O(n)$，畢竟就是看你有幾個點對吧。

* Determin the Unique Binary Tree，那我總結就是你有 *inorder* 的走訪路徑，都可以還原成一棵樹，但是有些特例是可以不需要 *inorder* 就可以還原原本的樹了，*complete* 包含 *full binary tree* 這兩種樹都可以不需要 *inorder* 就可以還原了，當然你給也是可以還原。

* 範例題目 108 中山資管資料結構

    ![圖 10. : 108 中山資管資料結構](https://imgur.com/vg6uL6O.png)

    那麼看到這個題目基本就秒殺哈哈哈哈，可以先知道 Preorder 就是第一個是 root 所以去 inorder 從 A 的位置切一刀你就知道左半邊是 *left subtree*，右半邊是 *right subtree*，然後一直反覆反覆就可以得知答案了。

    ![圖 11. : 108 中山資管資料結構解題](https://imgur.com/1C2D1QO.png){width=50%}

---
blog: true
blog_title: "二元樹 (Binary Tree)"
blog_date: 2024-08-26
blog_url: https://blog.walle4561.com/articles/posts/binary-trees-1/
---

## 前情提要

這篇文章是資料結構系列的第二篇，主要討論 Binary Tree（二元樹） 的基礎概念與相關理論。今天整理了 Binary Tree 的定義、基本性質、常見類型，以及一些重要的數學性質與推導，像是節點數、樹的高度等。此外，也介紹了二元樹的實現方式（陣列與鏈結串列）和走訪方法，並分享了一些經典範例題的解法，希望幫助讀者快速掌握二元樹的核心知識。

## Binary Tree 和 Tree 的討論

所有的樹是可以表示成二元樹的 (樹轉二元樹)，所以有很多人會把 Tree 跟 Binary Tree 搞混，有很重要的重點就是，樹不可以為空，但是 Binary Tree 可以為空，除此之外樹是沒有順序之分。

* Tree 和 Binary Tree degree 也有不同，因為 Tree Node's $degree >= 0$，但是 Binary Tree Node's $0 <= degree <=2$。

* 像是以下這個圖就是一個例子，在 Binary Tree 中這兩個是不同的樹，但是在樹裡面就是相同的樹。

  

![圖 2. : different tree from binary tree](https://imgur.com/lNmehwp.png)

  

## Binary Tree 定義

* A binary tree is a finite set of nodes that is either empty or consists of a root and two disjoint binary trees called theleft subree an the right subtree. (也就是說二元樹可以是空，那麼由一個 root 和兩個不相交的兩個子樹，這兩個子樹分別為右子樹、左子樹)

## Binary Tree ADT

**結構** : `Binary_Tree` (簡寫為 `BinTree`)

**物件** : 一組有限的節點，可以是空的，或包含根節點、左子樹，以及右子樹。

$$
\forall bt, bt1, bt2 \in \text{BinTree}, \; item \in \text{element}
$$

### 函數定義

以下是最小的操作步驟也就是說你的一個 Binary Tree 需要這些操作才能正常運作，當然你的ADT可ㄧ有很多自定義的操作，但是以 `horowitz` 的書中是這樣寫的。

* **BinTree Create()** : 建立一個空的二元樹

* **Boolean IsEmpty(bt)** : if (bt == 空二元樹) 則返回 TRUE，否則返回 FALSE

* **BinTree MakeBT(bt1, item, bt2)** : 建立一個二元樹，左子樹為 `bt1`，右子樹為 `bt2`，根節點包含資料項 `item`

* **BinTree Lchild(bt)** : if (IsEmpty(bt)) 則返回錯誤，否則返回 bt 的左子樹

* **element Data(bt)** : if (IsEmpty(bt)) 則返回錯誤，否則返回 bt 根節點中的資料

* **BinTree Rchild(bt)** : if (IsEmpty(bt)) 則返回錯誤，否則返回 bt 的右子樹

## Binary Tree Types
  
* *skewed* :
![[Assets/Note/Research/二元樹 (Binary Tree)/skewed binary tree.png#center]]

  

* *complete binary tree* : 用直白的話說 complete 其實就是你在 n-1 層是滿的且你在第 n 層是一定要按照 binary tree 的順序生成樹的不能有位置跳號的情況。
![[Assets/Note/Research/二元樹 (Binary Tree)/complete binary tree.png#center]]

* *not complete binary tree* :
![[Assets/Note/Research/二元樹 (Binary Tree)/not complete binary tree.png]]

  

* *full binary tree* : 這代表了是把所有編號都長滿那麼這個就是一個 full binary tree，那 full binary tree 也滿足 complete binary tree。

![圖 6. : full binary tree](https://imgur.com/yD6e0g4.png)



* *Strict binary tree* : Each non-leaf node will have two children. There is no degree-1 nodes exist.
![圖 7. : Strict binary tree](https://imgur.com/0dtk7qI.png)

  

## Binary Tree 的 4 個 Lemma

以下會介紹 Binary Tree 的 4 個 Lemma 其中有兩個屬於 maximum number of nodes 的證明，和一個 Relation between number of leaf node and nodes of degree 2，和一個是從 maximum number of nodes 變化出來的特別針對於 complete binary tree。

### Maximum number of nodes

1. The maximum number of nodes on level $i$ of a binary tree is $2^{i-1}$, $i \ge 1$.
	- **Proof (by induction)**
		1. **Base Case ($i = 1$)**  
		
			   - 樹中僅有根節點，此時第 1 層節點數為 1。  
			   
			   - 而 $2^{1-1} = 2^0 = 1$，與實際一致。
		
		2. **Induction Hypothesis**  
		
		   - 假設對所有 $j$ 滿足 $1 \le j < i$，第 $j$ 層最多有  $2^{j-1}$ 個節點。
		
		3. **Inductive Step**  
		
			1. 由假設可知，第 $i-1$ 層最多有  $2^{(i-1)-1} = 2^{i-2}$  個節點。  
				
			2. 每個節點最多有 2 個孩子，因此第 $i$ 層最多有  $2 \times 2^{i-2} = 2^{i-1}$  個節點。
		
		4. 因此，對所有 $i \ge 1$，第 $i$ 層節點數的上界皆為 $2^{i-1}$

2. The maximum number of nodes in a binary tree of depth k is $2^k - 1$, $k \ge 1$.
	 - **Proof** :  $$
	\begin{aligned}
	&\text{由 Lemma 1 可知第 }i\text{ 層最多有 }2^{i-1}\text{ 個節點；}\\
	&\text{又幾何級數公式 }
	  \sum_{i=0}^{n-1}a\,r^i
	  =a\frac{1-r^n}{1-r},
	  \quad a=1,\ r=2,\ n=k;\\
	&\text{故}
	  \sum_{i=1}^k2^{\,i-1}
	  =\sum_{i=0}^{k-1}2^i
	  =\frac{1-2^k}{1-2}
	  =2^k-1.\\
	&\text{因此}
	  \sum_{i=1}^k(\text{每層最大節點數})
	  =2^k-1.
	\end{aligned}
	$$

3. 如果有一個 complete binary tree 他有 n 個 nodes 那麼他的深度會是 $\lfloor log_2n + 1 \rfloor$ 假設一棵完全二元樹有 $n$ 個節點，則這棵樹的高度可以表示為：
	- **Proof** : $$
		\begin{aligned}
		&2^k - 1 \le n < 2^{k+1} - 1,\quad\text{由最大節點數界限得}\\
		&\log_2(2^k - 1) \le \log_2 n,\quad\text{對不等式兩邊取對數}\\
		&\log_2(2^k) \approx \log_2 n,\quad\text{當 $k$ 很大時，$2^k - 1 \approx 2^k$}\\
		&k \approx \log_2 n,\quad\text{因此}\\
		&k = \lceil \log_2(n+1)\rceil \quad\text{或}\quad \lfloor \log_2 n\rfloor + 1,\quad\text{考慮高度從 1 開始計算}
		\end{aligned}
		$$

- 以下事項非常重要：
	- **節點編號**  
		設 $i$ 為節點在陣列中的索引，且滿足  
		$$1 \le i \le n.$$
	
	- **父節點**  
		$$
		\mathrm{parent}(i) =
		\begin{cases}
		  \lfloor i/2\rfloor, & i > 1,\\
		  \text{根節點（無父節點）}, & i = 1.
		\end{cases}
		$$
	
	- **左子節點**  
		$$
		\mathrm{left\_child}(i) =
		\begin{cases}
		  2i, & 2i \le n,\\
		  \text{無}, & 2i > n.
		\end{cases}
		$$
	
	- **右子節點**  
		$$
		\mathrm{right\_child}(i) =
		\begin{cases}
		  2i + 1, & 2i + 1 \le n,\\
		  \text{無}, & 2i + 1 > n.
		\end{cases}
		$$


### Relation between number of leaf node and nodes of degree 2

在 Binary Tree 有一個現象，leaf 和 degree 2 的點數有些關係，這個證明滿重要的。

$\forall \, T \, \text{is a nonempty Binary Tree}$, if $n_0$ is leaf nodes and $n_2$ is two degree nodes, then $n_0 = n_2 + 1$.

**Proof :**

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
#### 總結

* Lemma 2 可以得知一個 full binary tree 他就會有 $2^k-1$ 個 nodes，這個應該滿好理解就不說了，在 `horowitz` 書中有寫到第 195 頁底下。

### Binary Tree 的最小高度推導

#### Q1. 若二元樹有 $n$ 個節點，最小高度是多少？

一棵**滿二元樹**擁有 $n$ 個節點時，高度是最小的。滿二元樹是指所有層次都被完全填滿的一棵二元樹。
對於滿二元樹，其節點數 $n$ 與高度 $h$ 的關係是：
$$
n = 2^h - 1
$$
這可以解釋為，高度為 $h$ 的滿二元樹，其節點數量是 $2^h - 1$。

反過來，最小高度 $h$ 與節點數 $n$的關係為：
$$
h = \lceil \, \log_2(n+1) \, \rceil
$$

因此，如果有 $n$ 個節點，二元樹的**最小高度**為：
$$
h_{\text{min}} = \lceil \, \log_2(n+1) \, \rceil
$$

這公式表明了給定 $n$ 個節點時的最小高度，對應於每一層節點都儘量填滿的情況。

#### Q2. 若有 $n$ 個葉節點，最小高度是多少？

假設我們有 $n$ 個葉節點，我們要構造一棵高度最小的二元樹。在二元樹中，葉節點的數量決定了樹的最小高度。最小高度的二元樹是一棵滿二元樹，其中所有的葉節點都位於最後一層或倒數第二層。
若我們將 $l$ 代表葉節點的數量，滿二元樹的葉節點數與高度 $h$ 的關係為 :
$$
2^{h-1} \leq l < 2^h
$$
這表示高度 $h$ 的二元樹最多可以容納 $2^{h-1}$ 到 $2^h - 1$ 個葉節點。

對於最小高度，我們取 :
$$

h_{\text{min}} = \lceil \, \log_2 l \, \rceil + 1

$$

因此，若有 $n$ 個葉節點，二元樹的**最小高度**為：
$$
h_{\text{min}} = \lceil \, \log_2 n \, \rceil + 1
$$

這表示給定 $n$ 個葉節點，最小高度是 $\lceil \log_2 n \rceil$ 再加一層 root node。

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
![[Assets/Note/Research/二元樹 (Binary Tree)/Array 表示 Binary Tree.png#center| 200]]

### Link list method

* 優點

	1. 容易新增和刪除節點

	2. 對於 skewed binary tree 比 Array 的方式省空間。

* 缺點

	1. 不容易存取父點。

	2. Link space 浪費約一半的空間。如果有 $N$ 個節點，則有 $N+1$ 條的 Nil link，由此 $2N-(N-1)$ 得知。

* 範例這邊就不畫了，手快斷掉了哈哈哈：
  

![[Assets/Note/Research/二元樹 (Binary Tree)/Link list 表示 Binary Tree.png]]

  


### 兩種方式比較

|        | Array                                                                          | Link List                                                                                   |
| ------ | ------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------- |
| **優點** | 1. 容易存取左右子點與父點<br>2. 對於 Full/Complete Binary tree 表示，空間充分利用，無浪費<br>是 1 的 space | 1. 容易增刪節點<br>2. 對於 skewed binary tree 比 Array 節省空間                                          |
| **缺點** | 1. 不容易儲節點點<br>2. 對於 skewed binary tree 表示，極度浪費空間。若高度 H，浪費格數為 $2^H - 1$         | 1. 不容易存取父點<br>2. Links space 浪費約一半。如果有$N$ 個節點，則有 $N+1$ 條的 Nil links，有用的<br>$2N - (N - 1)$ 條 |

## Binary Tree Traversal

大家想想，*right subtree* 和 *left subtree* 和 *root* 有幾種走訪方式 `3!` 種對吧，但是為了方便，有的就不討論了，核心就是一定要 *left subtree* 先走，然後才走 *right subtre*，那麼這樣就分成了，*VLR (preorder)*、*LVR (inorder)*、*LRV (postorder)*，反正就看 *V* 在哪裡就是叫什麼樣的走法，那麼還有一個比較特別的 *Level-Order*，在 `horowitz` 書中有寫到那我覺得他就是 *BFS* 其實，那麼走訪的 complexity 都是 $O(n)$，畢竟就是看你有幾個點對吧。

* Determin the Unique Binary Tree，那我總結就是你有 *inorder* 的走訪路徑，都可以還原成一棵樹，但是有些特例是可以不需要 *inorder* 就可以還原原本的樹了，*complete* 包含 *full binary tree* 這兩種樹都可以不需要 *inorder* 就可以還原了，當然你給也是可以還原。

* 範例題目 108 中山資管資料結構

![[Assets/Note/Research/二元樹 (Binary Tree)/108 中山資管資料結構.png]]

那麼看到這個題目基本就秒殺哈哈哈哈，可以先知道 Preorder 就是第一個是 root 所以去 inorder 從 A 的位置切一刀你就知道左半邊是 *left subtree*，右半邊是 *right subtree*，然後一直反覆反覆就可以得知答案了。

![[Assets/Note/Research/二元樹 (Binary Tree)/108 中山資管資料結構解題.jpg#center|300]]
### Binary Tree Traversal Code
```cpp
#include <bits/stdc++.h>

using namespace std;

struct Node {
  char val;
  Node* left;
  Node* right;
  Node(char v): val(v), left(nullptr), right(nullptr){}
};

void preorder(Node* root){
  if (!root) return;
  cout << root->val << " ";
  preorder(root->left);
  preorder(root->right);
}

void inorder(Node* root){
  if(!root) return;
  inorder(root->left);
  cout << root->val << " ";
  inorder(root->right);
}

void postorder(Node* root){
  if(!root) return;
  postorder(root->left);
  postorder(root->right);
  cout << root->val << " ";
}

int main() {
    Node* A = new Node('A');
    Node* B = new Node('B');
    Node* C = new Node('C');
    Node* D = new Node('D');
    Node* E = new Node('E');
    Node* F = new Node('F');

    A->left  = B;
    A->right = C;
    B->left  = D;
    B->right = E;
    C->right = F;

    cout << "Preorder:  ";
    preorder(A);
    cout << "\n";

    cout << "Inorder:   ";
    inorder(A);
    cout << "\n";

    cout << "Postorder: ";
    postorder(A);
    cout << "\n";

    delete A; delete B; delete C;
    delete D; delete E; delete F;

    return 0;
}

```

---

## 樹（Tree）與二元樹（Binary Tree）的轉換

### Tree → Binary Tree

- **規則：**
  1. 每個節點的 **第一個子節點** 成為其 **左子節點（left child）**。
  2. 其餘兄弟節點串接成 **右子鏈（right sibling）**。
  3. 遞迴處理每個節點。

- **應用：** 用於將多叉樹轉換為二元樹，方便儲存與實作。

![[Assets/Note/Research/二元樹 (Binary Tree)/Tree to Binary Tree 範例.png]]

---

### Binary Tree → Tree

- **規則：**
  1. 將二元樹節點 **逆時針旋轉 45°**。
  2. 還原 **父子之間的 link**。
  3. 移除只為表示兄弟關係的 **右鏈（sibling link）**。

- **效果：** 還原為原始的多叉樹結構。

![[Assets/Note/Research/二元樹 (Binary Tree)/Binary Tree to Tree 範例.png]]

---

## 森林（Forest）與二元樹（Binary Tree）的轉換

### Forest → Binary Tree

- **規則：**
  1. 將多棵樹的根節點視為兄弟，以 **右子鏈** 串接。
  2. 每棵樹使用 **Tree → Binary Tree** 的方式轉換。
  3. 合併所有轉換後的樹，形成一棵二元樹。

- **時間複雜度：** $O(n)$，其中 $n$ 為節點數。

![[Assets/Note/Research/二元樹 (Binary Tree)/Forest To Binary Tree 範例.png]]

---

### Binary Tree → Forest

- **規則：**
  1. 根節點間的 **右子鏈** 對應到森林中多棵樹。
  2. 將每棵子樹使用 **Binary Tree → Tree** 的方式轉換。
  3. 每一棵還原後的樹構成森林。

- **效果：** 還原為森林（多棵無序的多叉樹）。

![[Assets/Note/Research/二元樹 (Binary Tree)/Binary Tree to Forest.png]]
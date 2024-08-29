---
title: 資料結構-樹篇
date: 2024-08-26 15:15:34
tags: [Data structures, Tree]
categories: 資料結構
---

![圖 1. : Blog Image ](https://imgur.com/Sm9aRdD.png)

## Tree 討論

最近因為在補習班上資料結構，所以想說寫一下 blog 有參考幾本 bible 分別有 `clrs`、 `horowitz` 這兩本書來撰寫這篇 blog，內容可能跟 `horowitz` 的內容比較接近，因為 `clrs` 這本還是 algo 說的比較多哈哈哈。

<!--more-->

### Tree 定義

* A tree is a finite set of one or more nodes such that: (也就是說樹一定是由一個或是多個節點組成的有限集)

  1. There is aspecially designated node called the root. (也就是說有一個點必為 root)

  2. There remaining nodes are partioned into `n >= 0` disjoint sets T1,...,Tn, where each of these sets is a tree. We call T1,...Tn the subtrees of the root. (也就是說其他節點被劃分了 `n>=0` 個不相交的集合，每個集合都是一棵樹。稱之這些集合為 root 的 subtrees of the root.)

### Tree Terminology Introduction

![圖 2. : A sample tree](https://imgur.com/LzDtjZ9.png)

* *degree* : For example `A` the degree of A is 3, B the degree of B is 2. The degree of a tree is the maximum degree of the nodes in the tree. (也就是這顆樹的 degree 為某個節點且是最大 degree)

* *leaf* : 像是這顆樹中 K,L,F,G,M,I,J 就為 leaf，換句話說就是 degree 為 0 就是 leaf。

* *children*,*parent*,*siblings* : 例如 B 為 E,F 的 *parent*，反之 E,F 就是 B 的 *children*，像是 H,I,J 就是 *siblings*

* *ancestors* (祖先) : 例如 M 的 *ancestors* 是 A,D,H。

* *descendants* (後代) : 例如 E,F,K,L 都是 B 的 *descendants*

* level : 就是為這顆樹最深的深度例 (圖1) 就是 4，有的書會從 0 起算。

### Representation Of Trees

以下介紹兩種，樹的表示法 `List Representation` 和 `Left Child-Right Sibling Representation`。

### List Representation

* 可以使用括號來做表示 *(A (B (E (K, L), F), C(G), D(H(M), I, J)))* 以 (圖1) 為例。
  * 這種寫法比較清晰，大部分的書和刷題的題目會使用這樣的結構當作樹的結構。

* 也可以使用以下方法 *(A(B, C(E, F), D))* 下圖為資料結構中真實的樣子，以該樹的 degree 為三，所以要開四格一格為自己的資料如下圖：
    ![圖 3. : List Representtation](https://imgur.com/hiqw0DL.png)

### Left Child-Right Sibling Representation

* 簡單來說就是每個節點開三個欄位分別存最左邊的*children*，然後連接右邊的 siblings，和自己的資料。

* 以 *(A(B, C(E, F), D))* 為例如下圖：
    ![圖 4. : ](https://imgur.com/scqhzkk.png)
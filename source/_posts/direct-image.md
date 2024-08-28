---
title: Direct Image 的性質與應用
date: 2024-08-22 14:45:05
tags: [數學, 集合論, 函數]
categories: 數學
mathjax: true
---

![1. blog image](https://imgur.com/XpRufSg.png)

## 前言

在集合論中，直接像（Direct Image）是函數在兩個集合之間映射時所產生的一個重要概念。它描述了如何從一個集合 $A$ 的子集經過函數映射到另一個集合 $B$ 的子集中。本篇文章將探討直接像的定義與性質，並說明其在數學中的應用。

<!--more-->

## 定義

給定一個函數 $f: A \rightarrow B$，以及 $A$ 的一個子集 $U$，則直接像 $f(U)$ 是 $B$ 的一個子集，由 $f(u)$ 構成，其中 $u \in U$。簡單來說，$f(U)$ 包含了所有 $B$ 中對應於 $U$ 中元素的結果。

數學表示為：

$$
\begin{equation} \label{eq1}
f(U) = \{ f(u) \ | \ u \in U \}
\end{equation}
$$

## Direct Image 的性質

### 1. 聯集性質

對於 $A$ 的任意子集集合 $\{U_i\}_{i \in I}$，有：

$$
\begin{equation} \label{eq2}
f\left(\bigcup_{i \in I} U_i\right) = \bigcup_{i \in I} f(U_i)
\end{equation}
$$

這表示 $f$ 保留了聯集的結構。

### 2. 交集性質

對於 $A$ 的任意子集集合 $\{U_i\}_{i \in I}$，有：

$$
\begin{equation} \label{eq3}
f\left(\bigcap_{i \in I} U_i\right) \subseteq \bigcap_{i \in I} f(U_i)
\end{equation}
$$

交集的直接像包含於直接像的交集中。

### 3. 差集性質

對於 $A$ 的任意子集 $U$ 和 $V$，有：

$$
\begin{equation} \label{eq4}
f(V \setminus U) \supseteq f(V) \setminus f(U)
\end{equation}
$$

差集的直接像包含於直接像的差集中。

### 4. 補集性質

若 $U^C$ 為 $U$ 的補集，則：

$$
\begin{equation} \label{eq5}
f(U^C) \supseteq f(A) \setminus f(U)
\end{equation}
$$

補集的直接像包含於直接像的補集中。

### 5. 子集性質

若 $U \subseteq V \subseteq A$，則：

$$
\begin{equation} \label{eq6}
f(U) \subseteq f(V)
\end{equation}
$$

### 6. 逆像的直接像

對於 $A$ 的任意子集 $U$，有：

$$
\begin{equation} \label{eq7}
f^{-1}(f(U)) \supseteq U
\end{equation}
$$

當 $f$ 是單射時，等式成立。

### 7. 逆像的直接像

對於 $B$ 的任意子集 $V$，有：

$$
\begin{equation} \label{eq8}
f(f^{-1}(V)) \subseteq V
\end{equation}
$$

當 $f$ 是滿射時，等式成立。

## 結論

直接像在集合論和數學的其他分支中有著重要的應用，了解其性質有助於更好地理解函數如何作用於集合之間。透過這些性質，我們可以更深入地探討集合與函數的交互關係，為後續更複雜的數學理論打下基礎。

換句話說，$U$ 是 $A$ 的一小部分，而經過函數 $f$ 映射後，$U$ 變成了 $B$ 的一部分。這個過程揭示了集合之間的轉換關係，也是直接像這一概念的核心。

## 參考

1. [planetmath](https://planetmath.org/directimage)

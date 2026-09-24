---
title: "Proving NP-Completeness"
slug: proving-np-completeness
topic_section: algorithms
description: "My vault 演算法筆記：Proving NP-Completeness。"
date: 2026-09-24
blog: true
vault_source: "Note/Research/Proving NP-Completeness.md"
---

## 1. Polynomial-Time Reduction

### 定義 ($A \le_p B$)

若我們說語言 (Language) $A$ 可以 **Reduction** 到語言 $B$ ($A \le_p B$)，必須滿足以下條件：

1. 轉換函數 (Transformation)：
    
    存在一個函數 $f : \Sigma^* \to \Sigma^*$。
    
2. 效率 (Efficiency)：
    
    $f$ 必須是 多項式時間可計算的 (Polynomial-time computable)。
    
3. 若且唯若 (If and only if)：
    
    對於所有的輸入 $w$：
    
    $$w \in A \iff f(w) \in B$$
    
    - 如果 $w$ 是問題 $A$ 的 Yes 實例，則轉換後的 $f(w)$ 必須是問題 $B$ 的 Yes 實例。
        
    - 如果 $w$ 是問題 $A$ 的 No 實例，則轉換後的 $f(w)$ 必須是問題 $B$ 的 No 實例。
        

---

## 2. Steps to Prove NP-Completeness
![01-2. Steps to Prove NP-Completeness](/vault-assets/7931d9115b88597906ba.png)

### 重要前提 (Premise)

在證明開始前，我們必須確立證明的核心邏輯：

> 如果我們能證明一個已知的 NPC 問題 $C$ 可以 Reduction 到新問題 $L$ ($C \le_p L$)，且 $L$ 本身屬於 NP，那麼 $L$ 也是 NPC。
> 
> 邏輯：$L$ 至少跟 $C$ 一樣難。

![02-重要前提 (Premise)](/vault-assets/e3e855abf8e30b7f909a.png)

$L \in \text{NP-Complete} \iff (L \in \text{NP}) \text{ 且 } (L \in \text{NP-hard})$

意思：要證明 $L$ 是 NP-Complete，必須同時做到「$L$ 在 NP」與「$L$ 是 NP-hard」。

證明 $L$ 屬於 NPC 的步驟如下：

### Prove $L \in \text{NP}$

- **目標**：證明存在 certificate $y$ 與 verification algorithm $V$，使得對所有輸入 $x$：
    
    - 若 $x \in L$（YES instance），則存在某個 $y$ 讓 $V(x,y)=1$。
        
    - $V(x,y)$ 的執行時間是 polynomial time（多項式時間）。
        
- **直覺**：
    
    - $y$ 是「證據/候選解」（例如 SAT 的 assignment、HAM-CYCLE 的一條迴圈）。
        
    - $V$ 是「檢查器」，只負責快速檢查這個證據是否真的讓答案為 YES。
        
- **你在寫證明時通常要交代**：
    
    - $y$ 的格式是什麼、長度如何被 $|x|$ 的多項式上界住。
        
    - $V$ 怎麼檢查、為什麼是多項式時間。
        

### Prove $L \in \text{NP-hard}$ （$C \le_p L$）

- **核心**：從一個已知 NP-Complete 的問題 $C$ 出發，證明你能把 $C$ 的任意 instance 在 polynomial time 內轉成 $L$ 的 instance，且答案不變。
    

1. Select a known NPC problem $C$
    
    選擇一個已知 NP-Complete 的問題當起點（例如 CIRCUIT-SAT、3-CNF-SAT、CLIQUE、VERTEX-COVER）。
    
2. Construct a reduction $f$ transforming every instance of $C$ to an instance of $L$
    
    建構一個轉換函數 $f$：輸入 $x$（屬於 $C$ 的 instance），輸出 $f(x)$（屬於 $L$ 的 instance）。
    
    直覺：$f$ 就是一個「翻譯器」，把 $C$ 的題目翻成 $L$ 的題目。
    
3. Prove that $x \in C$ if and only if $f(x) \in L$ for all $x \in \{0,1\}^*$
    
    這一步是 reduction 正確性的核心（答案保持一致）：
    
    - $(\Rightarrow)$ 若 $x \in C$（原題 YES），則 $f(x) \in L$（翻譯後也 YES）。
        
    - $(\Leftarrow)$ 若 $f(x) \in L$（翻譯後 YES），則 $x \in C$（原題也 YES）。
        
    - 簡單講：翻譯前後的 YES/NO 不能被你翻到走鐘。
        
4. Prove that $f$ is a polynomial-time transformation
    
    證明 $f$ 的計算時間是 $\text{poly}(|x|)$。
    
    常見寫法：說明 $f(x)$ 的輸出大小是 $\text{poly}(|x|)$，且建構每個部件只花多項式時間，因此總時間是多項式。
    

### 結論

若第 3 點完成（$L \in \text{NP}$），且第 4 點完成（$C \le_p L \Rightarrow L$ NP-hard），則可推出 $L$ 是 NP-Complete。

---

## 3. 證明方向圖解

在證明 $B$ 是 NPC 時，Reduction 的方向至關重要，絕對不能搞反。

$$Known\ NPC\ Problem\ (A) \xrightarrow{Reduction\ f} New\ Problem\ (B)$$

- **正確**：$A \le_p B$ (證明 $B$ 至少跟 $A$ 一樣難 $\to$ $B$ 是 NPC)。
    
- **錯誤**：$B \le_p A$ (這只能證明 $B$ 屬於 NP，無法證明 $B$ 很難)。
## 4. 一些經典問題的 reduction 方向 
![03-4. 一些經典問題的 reduction 方向](/vault-assets/1f779bac14797268bc97.png)

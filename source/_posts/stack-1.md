---
title: 資料結構-Stack篇-1
date: 2024-08-28 17:32:30
tags: [Data structures, Stack]
categories: 資料結構
mathjax: true
---

![圖 1. : Blog Image](https://imgur.com/2Xe4Tou.png)

## Stack 基本的介紹

當談到資料結構時，Stack 是一個不可忽視的重要概念。即使在日常程式設計中不常使用它，理解 Stack 的原理仍然對其他應用場景大有助益。Stack 的經典應用之一是深度優先搜尋 (DFS)，它利用系統堆疊來記錄每次函數的調用過程。透過理解 Stack 的運作方式，我們能更好地掌握各種遞迴操作和函數調用的細節。接下來，我們將深入探討 Stack 的基本原理及其應用。

<!--more-->

## Stack 常見的應用

1. parsing context-free languages 文本剖析類型，例如 check 合法的括號配對，check 是不是 palindrome (回文)。
2. Evaluating arithmetic expressions (infix,postfix,prefix) and transformation to each other.
    * 中序表示法（Infix）：運算符號位於兩個操作數之間，如 A + B。
    * 後序表示法（Postfix）：運算符號位於操作數之後，如 A B +。
    * 前序表示法（Prefix）：運算符號位於操作數之前，如 + A B。
3. Function/recursive call management.
4. DFS 走訪圖.
5. Eight queen problem (八皇后) backtracking.
6. Maze (or mazing problem) 迷宮問題.

### Stack Frame 介紹

#### Stack Frame 是什麼？

1. 簡單來說，每當你的程式執行過程中使用一個 function（invoke a function）時，系統（通常是操作系統）會為這個 function 分配一個專門的空間，此空間稱為 stack frame。

2. Stack Frame 包含了該函數的局部變數、參數、返回地址（return address），以及一些相關的執行信息。

#### Stack Frame 的結構

* 區域變數 : 也就是每個每個 function 他都會將 local varaible 儲存在自己的 stack frame 中。

* return address : 當 function 執行完畢後，program 需要返回到執行該 function 的下一行位置，繼續執行其他指令。

* 前一個 stack frame 的 pointer (frame pointer) : OS 通常會保存一個 pointer，指向前一個被使用的 function，這樣就可以在 function 調用結束後，恢復原先狀態。

  * 如果有學過 x86 ISA 的讀者應該就會知道有兩個 register 分別為 `ebp`、`rbp`，在大多的 OS 中，一個 function 被使用，會創建一個 stack frame 並且 push 到 stack 中，那麼就會包含一個 frame pointer。

  * frame pointer 可以讓系統能夠在 function 執行完畢後，正確地回到之前的狀態並繼續執行。例如，當你從函數 B 返回到函數 A 時，需要知道函數 A 的 stack frame 位於哪裡，以便能夠繼續在 A 中執行。

    ![圖 2. : Stack frame pointer](https://imgur.com/i0U2n1M.png)

## Stack 的定義

* A stack has the LIFO (Last-In-First-Out) property. When inserting an element, we perform the operation from the top, which is called "push." Similarly, when removing an element, we also operate from the top, which is called "pop." Thus, both push and pop operations are performed at the top of the stack.

## Stack ADT

**結構** : Stack
**物件** : a finite ordered list with zero or more elemtns

### 函數定義

以下是最小的操作步驟也就是說你的一個 Stack 需要這些操作才能正常運作，當然你的 ADT 可ㄧ有很多自定義的操作，但是以 `horowitz` 的書中是這樣寫的，那麼這些操作的複雜度也都是 $O(1)$。

* **Stack CreateS(max_stack_size)** :
    建立一個空的 stack 且最大容量為 max_satck_size 之 value。

* **Boolean IsEmpty(stack)** :
  
  * 以下是書上的寫法他就是拿現有的 stack 去比較新的 stack 新的 stack 一定是空所就看判斷了，但是實物上不會這樣寫

    ```text
    if (satck == CreateS(max_stack_size)) return TRUE
    else return FALSE
    ```

* **Boolean IsFull(stack,max_stack_size)** :

  ```text
  if (size(stack) == max_satck_size)
  return TRUE
  else return FALSe
  ```

* **Stack Add(stack,item)** 也就是 push :

    ```text
    if (IsFull(stack,max_stack_size)) stack_full
    else insert item into top of stack and return 
    ```

* **Stack Delete(stack)** 也就是 pop :

    ```text
    if (IsEmpty(stack)) stack_empty
    else remove and return the item on the top of the stack
    ```

## Stack permutatoins

* 也就是說有幾種可能是正常的 `pop` `push` 操作會有的可能輸出題目可能會先給你一個序列。
* 那麼這邊介紹一個常用的公式，`catalan number` :

$$\dfrac{1}{n+1}\binom{2n}{n}$$

* e.x.
  * **123**,**132**,**213**,**231**,**321** => **5** 種。
  * **312** 是不可能的排列。
* 這邊就不証明了，以下為有幾種應用
  * 節點數為 $n$ 的二元樹結構數量。
  * 包含 $n$ 對括號的有效括號序列數量。
  * $n+1$ 個矩陣相乘的鏈結方法數量。
  * 有 $n$ 個火車進站時的輸出順序數量。

* 範例題目

    ![圖 3. : 107 中央資管資料結構](https://imgur.com/sdxFTPH.png)

## Evaluating arithmetic expressions

* 那麼這個是中序轉後序的寫法括弧寫好，全部都補上，然後後續就是把運算子都放在括弧後面，
注意不能使得放的位置有交錯，那麼你就會得到一個後序了，反之你會得到前序。

![圖 4. : 108 台大資工資料結構與演算法](https://imgur.com/jrJIbqR.png)

## Implementation code by 1-dim Array

```cpp
#include <bits/stdc++.h>

using namespace std;

#define MAX_SIZE 100  // 定義 Stack 的最大容量

template <typename T>
class Stack {
private:
    T arr[MAX_SIZE];  // 一維陣列來儲存 Stack 元素
    int top;          // 指向 Stack 的頂端

public:
    // 建構函式初始化 Stack
    Stack() {
    top = -1;  // 初始時 Stack 是空的
    }

    // 判斷 Stack 是否為空
    bool isEmpty() {
    return top == -1;
    }

    // 判斷 Stack 是否已滿
    bool isFull() {
    return top == MAX_SIZE - 1;
    }

    // Push 操作，將元素 push Stack
    void push(T value) {
    if (isFull()) {
        cout << "Stack overflow, cannot push " << value << endl;
        return;
    }
    arr[++top] = value;
    }

    // Pop 操作，將元素從 Stack pop
    T pop() {
    if (isEmpty()) {
        cout << "Stack underflow, cannot pop" << endl;
        return T();  // 回傳預設值
    }
    return arr[top--];
    }

    // Peek 操作，查看 Stack top
    T peek() {
    if (isEmpty()) {
        cout << "Stack is empty" << endl;
        return T();  // 回傳預設值
    }
    return arr[top];
    }

    // 顯示 Stack 內容
    void display() {
    if (isEmpty()) {
        cout << "Stack is empty" << endl;
        return;
    }
    cout << "Stack elements: ";
    for (int i = 0; i <= top; i++) {
        cout << arr[i] << " ";
    }
    cout << endl;
    }
};

int main() {
Stack<int> intStack;  // 創建一個整數型別的 Stack

intStack.push(10);
intStack.push(20);
intStack.push(30);

intStack.display();

cout << "Top element is: " << intStack.peek() << endl;

cout << "Popped element is: " << intStack.pop() << endl;

intStack.display();

Stack<string> stringStack;  // 創建一個字串型別的 Stack

stringStack.push("Hello");
stringStack.push("World");

stringStack.display();

cout << "Top element is: " << stringStack.peek() << endl;

cout << "Popped element is: " << stringStack.pop() << endl;

stringStack.display();
}
```

## Implementation code by link list

```cpp
#include <bits/stdc++.h>

using namespace std;

// 節點結構
template <typename T>
class Node {
public:
    T data;        // 節點儲存的數據
    Node* next;    // 指向下一個節點的指標

    Node(T value) : data(value), next(nullptr) {}
};

// Stack 類別
template <typename T>
class Stack {
private:
    Node<T>* top;  // 指向 Stack top

public:
    // 建構函式初始化 Stack
    Stack() : top(nullptr) {}

    // 判斷 Stack 是否為空
    bool isEmpty() {
        return top == nullptr;
    }

    // Push 操作，將元素 push Stack
    void push(T value) {
        Node<T>* newNode = new Node<T>(value);
        newNode->next = top;
        top = newNode;
    }

    // Pop
    T pop() {
        if (isEmpty()) {
            cout << "Stack underflow, cannot pop" << endl;
            return T();  // 回傳預設值
        }
        Node<T>* temp = top;
        T poppedValue = top->data;
        top = top->next;
        delete temp;
        return poppedValue;
    }

    // Peek 操作，查看 Stack top
    T peek() {
        if (isEmpty()) {
            cout << "Stack is empty" << endl;
            return T();  // 回傳預設值
        }
        return top->data;
    }

    // 顯示 Stack 內容
    void display() {
        if (isEmpty()) {
            cout << "Stack is empty" << endl;
            return;
        }
        Node<T>* current = top;
        cout << "Stack elements: ";
        while (current != nullptr) {
            cout << current->data << " ";
            current = current->next;
        }
        cout << endl;
    }

    // 解構函式清除 Stack 所有節點
    ~Stack() {
        while (!isEmpty()) {
            pop();
        }
    }
};

int main() {
    Stack<int> intStack;  // 創建一個整數型別的 Stack

    intStack.push(10);
    intStack.push(20);
    intStack.push(30);

    intStack.display();

    cout << "Top element is: " << intStack.peek() << endl;

    cout << "Popped element is: " << intStack.pop() << endl;

    intStack.display();

    Stack<string> stringStack;  // 創建一個字串型別的 Stack

    stringStack.push("Hello");
    stringStack.push("World");

    stringStack.display();

    cout << "Top element is: " << stringStack.peek() << endl;

    cout << "Popped element is: " << stringStack.pop() << endl;

    stringStack.display();
}
```

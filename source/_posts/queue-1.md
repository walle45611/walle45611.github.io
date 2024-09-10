---
title: 資料結構-Queue篇-1
date: 2024-09-09 11:58:09
tags: [Data structures,queue]
categories: 資料結構
mathjax: true
---

![圖 1. : Blog Image](https://imgur.com/lHiUPLR.png)

## Queue 的定義

* A queue is an ordered list in which all insertions take place at one end and all deletions take place at the opposite end. Given a queue $Q = (a_0, a_1, \dotsb, a_{n-1})$, $a_0$ 我們稱之為 front element, $a_{n-1}$ 我們稱之為 rear element，那麼 Queue 有 $\text{First-In-First-Out (FIFO)}$ 的性質。

<!--more-->

## Queue ADT

**結構** : Queue  
**物件** : a finite ordered list with zero or more elements

### 函數定義

* **Queue CreateQ(max_queue_size)** : 建立一個空的 queue 且最大容量為 max_queue_size。

* **Boolean IsFullQ(queue, max_queue_size)** :

    ```text
    if (number of elements in queue == max_queue_size) 
        return TRUE
    else 
        return FALSE
    ```

* **Boolean IsEmptyQ(queue)** :

    ```text
    if (queue == CreateQ(max_queue_size)) 
        return TRUE
    else 
        return FALSE
    ```

* **Queue 中新增 element**

  * Queue AddQ(queue, item) 此為 `Horowitz` 的寫法 :

      ```text
      if (IsFullQ(queue)) 
          queue_full
      else 
          insert item at rear of queue and return queue
      ```

  * **ENQUEUE(Q, x)** 此為 `CLRS` 中的寫法，此為 `circular queue`，稍後會說明到:

    ```text
    Q[Q.tail] = x
    if Q.tail == Q.length
        Q.tail = 1
    else 
        Q.tail = Q.tail + 1
    ```

* **Queue 中刪除 element**

  * Element DeleteQ(queue) 此為 `Horowitz` 的寫法:

      ```text
      if (IsEmptyQ(queue)) 
          queue_empty
      else 
          remove and return the item at front of queue
      ```

  * **DEQUEUE(Q)** 此為 `CLRS` 中的寫法 :

    ```text
    x = Q[Q.head]
    if Q.head == Q.length
        Q.head = 1
    else 
        Q.head = Q.head + 1
    return x
    ```

## Queue 的實作方式

### Linear Array

* **實作程式碼**

    ```cpp
    #include <iostream>
    using namespace std;

    class Queue {
    private:
        int front, rear, size;
        int* arr;

    public:
        Queue(int maxSize) {
            size = maxSize;
            arr = new int[size];
            front = -1;
            rear = -1;
        }

        ~Queue() {
            delete[] arr;
        }

        bool isFull() {
            return rear == size - 1;
        }

        bool isEmpty() {
            return front == rear;
        }

        void enqueue(int item) {
            if (isFull()) {
                cout << "Queue is full, shifting elements to the left..." << endl;
                shiftLeft();
            }
            if (rear == -1) {
                front = 0;
            }
            arr[++rear] = item;
            cout << "Enqueued: " << item << endl;
        }

        int dequeue() {
            if (isEmpty()) {
                cout << "Queue is empty." << endl;
                return -1;
            }
            int item = arr[front];
            front++;
            cout << "Dequeued: " << item << endl;
            return item;
        }

        void shiftLeft() {
            for (int i = front; i <= rear; i++) {
                arr[i - front] = arr[i];
            }
            rear = rear - front;
            front = 0;
        }
    };

    int main() {
        Queue q(3);

        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);
        q.enqueue(4);
        q.dequeue();
        q.dequeue();
        q.enqueue(5);
        q.enqueue(6);

        return 0;
    }
    ```

* **輸出內容**

    ```text
    Enqueued: 1
    Enqueued: 2
    Enqueued: 3
    Queue is full, shifting elements to the left...
    Enqueued: 4
    Dequeued: 1
    Dequeued: 2
    Enqueued: 5
    Enqueued: 6
    ```

* 這個方法有一個嚴重的問題如下圖 (圖 2)，當 Job 進入或離開時，Queue 的索引（rear 和 front）會隨著作業的加入或刪除逐漸向右移動。當 $rear$ 等於 $\text{MAX_QUEUE_SIZE} - 1$ 時，這意味著隊列已滿。在這種情況下，必須通過 `queue_full` 操作將整個隊列左移，這樣 $front$ 就會回到 $-1$，而 $rear$ 也需要重新計算，以確保隊列正確排列。  
這是一個典型的線性隊列，而不是循環隊列。當隊列滿了並且刪除了一些元素時，隊列的空間無法立即重用，因此需要整體左移來重新排列元素，這也是該隊列的限制之一。這樣花的時間是 $O(\text{MAX_QUEUE_SIZE})$，因為每次 `queue_full` 都要將所有元素往左移動。

  ![圖 2. : Linear Array 問題](https://imgur.com/cznWHPR.png)

## Array Circular queue

![圖 3. : Array Circular queue](https://imgur.com/wRXfv0X.png)

### 法一

* **實作程式碼**

    ```cpp
    #include <bits/stdc++.h>
    using namespace std;

    class CircularQueue {
    private:
        int* queue;
        int size;
        int front, rear;

    public:
        CircularQueue(int n) {
            size = n;
            queue = new int[size];
            front = 0;
            rear = 0;
        }

        bool isFull() {
            return (rear + 1) % size == front;
        }

        bool isEmpty() {
            return front == rear;
        }

        void enqueue(int value) {
            if (isFull()) {
                cout << "Queue is full" << endl;
                return;
            }
            queue[rear] = value;
            rear = (rear + 1) % size;
        }

        int dequeue() {
            if (isEmpty()) {
                cout << "Queue is empty" << endl;
                return -1;
            }
            int value = queue[front];
            front = (front + 1) % size;
            return value;
        }

        void display() {
            if (isEmpty()) {
                cout << "Queue is empty" << endl;
                return;
            }
            int i = front;
            while (i != rear) {
                cout << queue[i] << " ";
                i = (i + 1) % size;
            }
            cout << endl;
        }

        ~CircularQueue() {
            delete[] queue;
        }
    };

    int main() {
        ios_base::sync_with_stdio(false);
        cin.tie(nullptr);

        CircularQueue q(5);
        q.enqueue(10);
        q.enqueue(20);
        q.enqueue(30);
        q.enqueue(40);

        q.display();

        q.dequeue();
        q.display();

        q.enqueue(50);
        q.display();

        q.enqueue(60);
    }
    ```

* **輸出結果**

  ```text
    10 20 30 40
    20 30 40
    20 30 40 50
    Queue is full
  ```

* 使用這個方法最多可以利用 $n-1$ 的空間。

### 法二

* **實作程式碼**

    ```cpp
    #include <iostream>
    using namespace std;

    class CircularQueue {
    private:
        int* queue;
        int size;
        int front;
        int rear;
        int tag;  

    public:
        CircularQueue(int n) {
            size = n;
            queue = new int[size];
            front = 0;
            rear = 0;
            tag = 0;  
        }

        ~CircularQueue() {
            delete[] queue;
        }

        bool isFull() {
            return (front == rear && tag == 1);
        }

        bool isEmpty() {
            return (front == rear && tag == 0);
        }

        void enqueue(int value) {
            if (isFull()) {
                cout << "Queue is full" << endl;
                return;
            }
            queue[rear] = value;
            rear = (rear + 1) % size;
            tag = 1;  
        }

        int dequeue() {
            if (isEmpty()) {
                cout << "Queue is empty" << endl;
                return -1;  
            }
            int value = queue[front];
            front = (front + 1) % size;
            if (front == rear) {  
                tag = 0;
            }
            return value;
        }

        void display() {
        if (isEmpty()) {
            cout << "Queue is empty" << endl;
            return;
        }
        int i = front;
        cout << "Queue contents: ";
        while (i != rear || (i == rear && tag == 1)) {
            cout << queue[i] << " ";
            i = (i + 1) % size;
            if (i == front) break;  
        }
            cout << endl;
        }
    };

    int main() {
    CircularQueue q(5);  

    q.enqueue(10);
    q.enqueue(20);
    q.enqueue(30);
    q.enqueue(40);
    q.enqueue(50);
    q.display();

    cout << "Dequeued: " << q.dequeue() << endl;
    q.display();

    q.enqueue(60);
    q.display();

    return 0;
    }
    ```

* **輸出內容**

    ```text
    Queue contents: 10 20 30 40 50
    Dequeued: 10
    Queue contents: 20 30 40 50 10
    Queue contents: 20 30 40 50 60
    ```

## Link List queue

* **實作程式碼**

    ```cpp
    #include <iostream>
    using namespace std;

    class Node {
    public:
        int data;
        Node* next;

        Node(int val) {
        data = val;
        next = nullptr;
        }
    };

    class Queue {
    private:
        Node* front;
        Node* rear;

    public:
        Queue() {
            front = rear = nullptr;
        }

        bool isEmpty() {
            return front == nullptr;
        }

        void enqueue(int val) {
            Node* newNode = new Node(val);
            if (rear == nullptr) {
                front = rear = newNode;
                return;
            }
            rear->next = newNode;
            rear = newNode;
        }

        int dequeue() {
        if (isEmpty()) {
            cout << "Queue is empty" << endl;
            return -1;
        }
        Node* temp = front;
        front = front->next;

        if (front == nullptr) {
            rear = nullptr;
        }

        int dequeuedData = temp->data;
        delete temp;
        return dequeuedData;
        }

        int peek() {
        if (isEmpty()) {
            cout << "Queue is empty" << endl;
            return -1;
        }
        return front->data;
        }

        void display() {
        if (isEmpty()) {
            cout << "Queue is empty" << endl;
            return;
        }
        Node* current = front;
        while (current != nullptr) {
            cout << current->data << " -> ";
            current = current->next;
        }
        cout << "NULL" << endl;
        }
    };

    int main() {
        Queue q;
        q.enqueue(10);
        q.enqueue(20);
        q.enqueue(30);

        q.display();

        cout << "Dequeue: " << q.dequeue() << endl;
        q.display();
    }
    ```

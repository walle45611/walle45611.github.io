---
blog: true
blog_title: "Linear Search and Binary Search"
blog_date: 2026-09-24
blog_url: https://blog.walle4561.com/articles/posts/linear-and-binary-search/
---

# Linear (Sequential) Search

**特點：**

* 搜尋前不需要排序。

* 可用於 Array (隨機存取) 或 Linked List (順序存取)。

* 時間複雜度：$O(n)$。

**平均比較次數：**

* 假設機率平均分布：$\frac{1 + 2 + \dots + n}{n} = \frac{(n+1)}{2}$

* 所以平均需要 $\frac{n+1}{2}$ 次比較，仍為 $O(n)$。

## 一般 Linear Search

```c
int search(int arr[], int n, int x) {
	int i;
	
	for (i = 0; i < n; i++){
		if (arr[i] == x)
		return i;
	}
	return -1; // 找不到
}
```

* 優點：程式直觀、易懂。

* 缺點：每次迴圈要同時檢查索引是否越界 (`i < n`) 與值是否相等 (`arr[i] == x`)。

### Linear Search with Sentinel

```c
int search(int arr[], int n, int x) {
	int i = n;
	arr[0] = x; 
	while (arr[i] != x) i--;
	return i;
}
```

* 優點：避免每次檢查邊界，程式更簡潔。

* 缺點：

	* 需要額外保留 `arr[0]` 作為哨兵，否則會覆蓋資料。
	
	* 實際上只少掉「一次邊界檢查」，效能提升有限。

### 比較次數分析

| 情境       | 普通搜尋               | 哨兵搜尋     |
| -------- | ------------------ | -------- |
| 找到第一個元素  | 1 次值比較 + 邊界檢查      | 1 次值比較   |
| 找到最後一個元素 | n 次值比較 + n 次邊界檢查   | n 次值比較   |
| 找不到      | n 次值比較 + n+1 次邊界檢查 | n+1 次值比較 |

👉 哨兵法最多省掉邊界檢查，效能差異不大。

# Binary Search

**前提條件：**

* 資料必須事先排序 (例如由小到大)。

* 必須能隨機存取 (Array)。

**時間複雜度：** $O(\log n)$。

**概念：**

1. 每次取中點 `m`，比較 `arr[m]` 與目標值 `x`。

2. 若相等 → 找到。

3. 若 `x` 較小 → 在左半部繼續搜尋。

4. 若 `x` 較大 → 在右半部繼續搜尋。

### 程式碼

### 迴圈版本
```c
int binary_search(int arr[], int l, int r, int x) {
	while (l <= r) {
		int m = l + (r - l) / 2; // 避免溢位
		if (arr[m] == x) return m;
		else if (arr[m] > x) r = m - 1;
		else l = m + 1;
	}
	return -1; // 找不到
}
```

### 遞迴版本
```cpp
int binary_search(int arr[], int l,int r,int x){
	if(l <= r){
		int m = l + (l - r) / 2;
		if (arr[m] == x) return m;
		else if(x > arr[m]) binary_search(arr,m+1,r,x);
		else binary_search(arr,l,m-1,x);
	}

}
```
### 注意事項：避免整數溢位

* **錯誤寫法：** 若 `l` 和 `r` 很大，`l + r` 可能超出整數範圍。

	```c
	int m = (l + r) / 2;
	```

* **正確寫法：** 這樣保證不會溢位。
	```c
	int m = l + (r - l) / 2;
	```

### 時間複雜度遞迴式分析

* 遞迴式：$T(n) = T(n/2) + 1, \quad T(1) = 1$

* 根據 Master Theorem：

	* $a = 1, b = 2$
	
	* $n^{\log\_b a} = n^0 = 1$
	
	* $f(n) = 1 = O(1)$

	* 符合 **Case 2** 因此：$T(n) = \Theta(\log n)$

## Linear vs Binary Search

| 特性 | Linear Search | Binary Search |
| ------- | ------------------- | ------------- |
| 前提 | 無需排序 | 必須排序 |
| 適用資料結構 | Array / Linked List | Array (隨機存取) |
| 平均時間複雜度 | $O(n)$ | $O(\log n)$ |
| 最壞時間複雜度 | $O(n)$ | $O(\log n)$ |
| 最佳情況 | 找到第一個元素只需 1 次比較 | 找到中點即可 1 次比較 |
## 總結

* **Linear Search**：簡單通用，但效能差，平均 $O(n)$。

* **Linear Search with Sentinel**：只少掉邊界檢查，提升有限。

* **Binary Search**：需要排序資料，但效能大幅提升到 $O(\log n)$；要注意避免中點計算溢位；遞迴式 $T(n)=T(n/2)+1$ → $\Theta(\log n)$。

# 比較兩個 list 的 key value

## verify1 

- $O(n\times m)$
- 簡單來說就是用 list1 去掃描 list2 有沒有相同 key 並且確認有相同 key 後 `marked[i]=1`，並且判斷 value 數值相不相同，如果不相同的話那麼就輸出
- 最後會知道再把 list1 不在 list2 的 key 都輸出出來

```cpp
#include <bits/stdc++.h>

#define MAX_SIZE 100

using namespace std;

typedef struct {
  int key;
  int value;
} element;

int seqsearch(element a[],int len, int key){
  for (int i =0 ;i < len ; i++){
    if((a[i].key) == key)
      return i;
  }
  return -1;
}

void verify1(element list1[],element list2[],int n,int m){
  bool marked[MAX_SIZE];
  for(int i=0;i<m;++i) marked[i]=0;

  for (int i=0;i<n;i++){
    int j = seqsearch(list2,m,list1[i].key);
    if(j < 0) cout << list1[i].key << " key is not in list2\n";
    else{
      if (list1[i].value != list2[j].value) cout << "payload differs on key " << list1[i].key << "\n";
      marked[j] = 1;
    }
  }
  for(int i=0;i<m;++i){
    if (!marked[i]) cout << list2[i].key << "is not in list1\n";
  }
}

int main(){
  element list1[] = {{1, 10}, {2, 20}, {3, 30}};
  element list2[] = {{2, 20}, {3, 99}, {4, 40}};
  int n = sizeof(list1) / sizeof(list1[0]);
  int m = sizeof(list2) / sizeof(list2[0]);
  verify1(list1,list2,n,m);
  return 0;
}

```

## verify2 two point

- 這個的複雜度就是幾 $O(n\lg n)$ 或是 $O(m \lg m)$ 就看哪個比較長，主要是排序花的時間已經是 $O(n \lg n)$
- 核心做法就是 sort 後比較 key 有沒有相同如果沒有相同那麼就是代表有少 key 並 ++ 往前走

```cpp
#include <bits/stdc++.h>
using namespace std;

struct element {
  int key;
  int value;
};

bool cmp(const element &a, const element &b) {
  return a.key < b.key;
}

void verify2(element list1[], element list2[], int n, int m) {
  sort(list1, list1 + n, cmp);
  sort(list2, list2 + m, cmp);

  int i = 0, j = 0;
  while (i < n && j < m) {
    if (list1[i].key < list2[j].key) {
      printf("%d is not in list 2\n", list1[i].key);
      i++;
    } else if (list1[i].key == list2[j].key) {
      i++; j++;
    } else {
      printf("%d is not in list 1\n", list2[j].key);
      j++;
    }
  }
  for (; i < n; i++) printf("%d is not in list 2\n", list1[i].key);
  for (; j < m; j++) printf("%d is not in list 1\n", list2[j].key);
}

int main() {
  element list1[] = {{1,10}, {2,20}, {3,30}};
  element list2[] = {{2,20}, {3,99}, {4,40}};
  int n = sizeof(list1) / sizeof(list1[0]);
  int m = sizeof(list2) / sizeof(list2[0]);
  verify2(list1, list2, n, m);
  return 0;
}
```
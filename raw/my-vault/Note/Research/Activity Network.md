---
blog: true
blog_title: "活動網路（Activity Network）"
blog_date: 2026-09-24
blog_url: https://blog.walle4561.com/articles/posts/activity-network/
---

# AOV（Activity on Vertex）

- 定義：AOV 網路是有向圖 $G$。頂點＝任務或活動；邊＝先後關係。
    
- 前驅 / 後繼：若從 $i$ 有有向路徑到 $j$，則 $i$ 是 $j$ 的前驅，$j$ 是 $i$ 的後繼。
    
- 直接前驅：若邊 $\langle i,j\rangle\in E$，則 $i$ 為 $j$ 的「直接前驅」。
    
- 遞移（transitive）：若 $i\to j$ 且 $j\to k$，則 $i\to k$。
    
- 反自反（irreflexive）：對所有 $i$，不成立 $i\to i$。
    
- 偏序（partial order）：同時具備「傳遞」與「反自反」的先後關係。
    
- 可行性：若網路無有向環（即為 DAG），則可行。
    
## 拓撲序（Topological Order）

- 定義：一個線性序，使得任意兩頂點 $i,j$，若 $i$ 是 $j$ 的前驅，則 $i$ 在序中先於 $j$。
    
- 一個 AOV 可能有多個拓撲序。
    
### 拓撲排序

目標：輸出一個拓撲序；若偵測到環則回報「不可行」。

資料結構：

- 以**adj list**表示圖；每個頂點保留**in-degree 計數**（count）。
    
- 維護一個**堆疊**存放入度為 $0$ 的頂點。
    

步驟：

1. 計算所有頂點的 in-degree。將 in-degree $=0$ 的頂點依序推入堆疊 $S$。
    
2. 重複 $n$ 次：
    
    - 若 $S$ 為空，則每個頂點都有前驅 ⇒ 圖含環 ⇒ 報告失敗。
        
    - 否則彈出一頂點 $j$ 並輸出之；
        
    - 對 $j$ 的每個後繼 $k$：將 $k$ 的入度 $count[k]{-}{-}$；若變為 $0$，則把 $k$ 推入 $S$。
        
3. 全部輸出完即得一個拓撲序。
    
> 時間複雜度：$O(e+n)$。空間：鄰接表與計數陣列 $O(e+n)$。
![[Assets/Note/Research/Activity Network/01-拓撲排序 - 全部輸出完即得一個拓撲序。.png]] 
### 參考樣式（貼近課本 C 風格）

```c
void topsort(Graph g, int n){
  int i, j, k; Node* p;
  int top = -1;              // 堆疊 top（用整數鏈結法）
  for(i=0;i<n;i++)           // 建立初始堆疊：入度為 0 的頂點
    if(g[i].count==0){ g[i].count = top; top = i; }

  for(i=0;i<n;i++){
    if(top==-1){ error("Network has a cycle"); return; }
    j = top;                 // 彈出
    top = g[top].count;
    print(j);                // 輸出 j
    for(p=g[j].link; p; p=p->link){
      k = p->vertex;
      if(--g[k].count==0){   // 新成為入度 0
        g[k].count = top;
        top = k;             // 推入堆疊
      }
    }
  }
}
```

# AOE （Activity on Edge）

- 模型：頂點=事件；邊=活動；權重 $w_{uv}$=工期；必要時加「虛擬活動」（工期 $0$）表額外依賴。
	
- 目標：專案最短完工時間、關鍵路徑、各活動最早/最遲時刻與浮時。
	
- 流程（先取事件的拓撲序）：
		
	- 正向傳播（最早事件時間）：$ve[s]=0$，$ve[v]=\max_{(u,v)\in E}(ve[u]+w_{uv})$；工期 $T^*=ve[t]$。
		
	- 反向傳播（最遲不延誤時間）：$vl[t]=T^*$，$vl[u]=\min_{(u,v)\in E}(vl[v]-w_{uv})$。
		
	- 活動指標：
			
		- 最早開工 $e_{uv}=ve[u]$。
			
		- 最遲開工 $l_{uv}=vl[v]-w_{uv}$。
			
		- $s_{uv}=l_{uv}-e_{uv}=(vl[v]-w_{uv})-ve[u]$。
		
	- 關鍵活動與路徑：$s_{uv}=0$ 的邊與其連成的 $s\to t$ 路徑（可多條）。
		
- 複雜度：鄰接表＋拓撲序實作皆為 $O(|V|+|E|)$。
## 範例
![[Assets/Note/Research/Activity Network/02-範例.png]]
- **Q3｜Critical task 判定**
    
    - 定義：位於任一關鍵路徑的活動；浮時 $s=0$。
        
    - 作法：跑 CPM，找所有邊 $(u,v)$ 使 $s_{uv}=0$。
        
    - 例：關鍵路徑 $A1\to a4 \to a7\to a10$、$A1\to a4 \to a8\to a11$  
        → critical tasks $={A1,a4,a7,a8,a10,a11}$。
        
- **Q4｜縮短工期要加速哪裡**
    
    - 先列出全部關鍵路徑，取其交集＝**bottleneck tasks**。
        
    - 只要縮短交集中的活動，專案總工期一定下降；只縮某一路徑上的非交集活動，可能不降。
        
- **Q5｜哪些活動可延遲？可延遲多久**
    
    - 非 critical task 可延遲；可延遲量＝浮時 $s$。
        
    - 事件最早時間（正向）：
        
        - $ee(s)=0$
            
        - $ee(v)=\max_{(u,v)\in E}\big(ee(u)+w_{uv}\big)$
            
    - 事件最遲時間（反向）：
        
        - $T^*=ee(t)$，$le(t)=T^*$
            
        - $le(u)=\min_{(u,v)\in E}\big(le(v)-w_{uv}\big)$
            
        - 例：$le(y)=15,\ le(z)=16,\ w_{xy}=6,\ w_{xz}=10 \Rightarrow le(x)=\min(15-6,16-10)=6$
            
    - 活動指標與浮時：
        
        - 最早開工：$e_{uv}=ee(u)$
            
        - 最遲開工：$l_{uv}=le(v)-w_{uv}$
            
        - 浮時：$s_{uv}=l_{uv}-e_{uv}=(le(v)-w_{uv})-ee(u)$
            
        - 例：$ee(x)=5$，若某活動 $l=7$，則 $s=7-5=2$（可延 2 天不影響完工）
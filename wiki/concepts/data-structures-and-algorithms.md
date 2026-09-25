# 資料結構與演算法

## Current View

資料結構決定資料如何表示與存取；演算法則描述在這些表示上如何搜尋、走訪、排序或求解。選型時要同時確認操作需求、輸入規模與限制，不能只看單一時間複雜度。這裡整合概念與方法；題號及練習紀錄由[刷題總覽摘要](../summaries/my-vault-dashboard-overview-4e3b3b280.md)對應的原始筆記脈絡管理，不混作概念定義。

## 主題地圖

- **線性資料**：陣列筆記從索引與元素大小推導位址，顯示表示方式如何影響直接定位。[陣列摘要](../summaries/my-vault-research-array-d69e88713.md)
- **樹與搜尋**：BST 依鍵值維持搜尋順序，中序走訪可輸出升冪結果；若形狀偏斜，操作可能退化。AVL 以高度平衡限制樹高，補足一般 BST 的最壞情形。[BST](../summaries/my-vault-research-binary-search-tree-00dff8faa.md) · [AVL](../summaries/my-vault-research-adelson-velsky-and-landis-tree-6eaf640b5.md)
- **圖與走訪**：以頂點和邊表示關係，再依問題選 DFS、BFS 或其他圖演算法；路徑、連通性與方向性需先定義。[圖論摘要](../summaries/my-vault-research-graph-dfs-bfs-6d2b56ce5.md)
- **排序**：除了執行成本，也需分辨穩定性、是否原地排序，以及資料能否放入記憶體。[排序摘要](../summaries/my-vault-research-sorting-algo-e54a59046.md)
- **最佳化問題**：0/1 背包限制每件物品只能取或不取，需在容量限制內選擇組合；這是問題模型，不能直接視為某一種資料結構。[背包摘要](../summaries/my-vault-research-01-01-knapsack-problem-a4021e9c5.md)

## Boundaries

上述來源是不同主題的學習筆記，不構成完整的演算法教科書；複雜度仍須按特定操作、實作與前提判斷。BST 摘要採「不允許重複 key」的版本，其他實作可另設重複鍵政策。刷題分類與解題過程不應取代概念頁的定義與比較。

## Sources

- [陣列記憶體位址](../summaries/my-vault-research-array-d69e88713.md)
- [二元搜尋樹](../summaries/my-vault-research-binary-search-tree-00dff8faa.md)
- [AVL 樹](../summaries/my-vault-research-adelson-velsky-and-landis-tree-6eaf640b5.md)
- [圖與 DFS/BFS](../summaries/my-vault-research-graph-dfs-bfs-6d2b56ce5.md)
- [排序](../summaries/my-vault-research-sorting-algo-e54a59046.md)
- [0/1 背包](../summaries/my-vault-research-01-01-knapsack-problem-a4021e9c5.md)

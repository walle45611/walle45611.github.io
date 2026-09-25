# Adelson-Velsky and Landis tree(AVL Tree)

- source: `raw/my-vault/Note/Research/Adelson-Velsky and Landis tree(AVL Tree).md`
- source_sha256: `2e1fd829eb28745551ec58286ffe2a93e3d9561cb39bb4546c2d47690fc0614a`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 基本定義、判斷是不是 balanaced binary tree、Ex1：根節點失衡 → Not AVL、Ex2：內部節點失衡（根平衡但子樹不平衡）→ Not AVL、Ex3：違反 BST 順序 → Not BST、Horowitz 調整原則。

## Source Notes

- 在一般 BST 中可能退化為 skewed，致使 $\text{search/insert/delete} = O(n)$。
- AVL Tree 維持 height balanced，保證 $O(\log n)$。
- 平衡因子：$\mathrm{BF}(v) = H_L - H_R \in {-1, 0, 1}$。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

# Red-Black tree

- source: `raw/my-vault/Note/Research/Red-Black tree.md`
- source_sha256: `1c90d420b725cca1b570e48a12a4810f2ab9c02b04d897e8a723218661fd7e99`
- ingested_at: 2026-09-26
- reviewed_at: 2026-10-04
- type: my-vault note summary
- collection: Research

## Summary

筆記涵蓋紅黑樹五項性質、黑高與高度上界證明、Top-down 插入、CLRS Bottom-up 插入，以及與 2-3-4 Tree 的對應。

## Source Notes

- CLRS 黑高不計起點，但計入終點 NIL；NIL 的黑高為 0。子節點為黑時，其黑高比父節點少 1；為紅時則相同。含 $n$ 個內部節點的樹滿足 $h \le 2\lg(n+1)$。
- 共享 NIL 哨兵為黑色，不存放有效 key；刪除修復會使用目前 NIL 位置的父指標，不能把所有其他欄位都視為不會使用。
- Top-down 在向下搜尋時變色與修復；CLRS 先插入再向上修復。CLRS 插入至多兩次基本旋轉，刪除至多三次；此界限不能直接套用到其他版本。雙旋轉由兩次基本旋轉組成。
- AVL 插入至多修復一個失衡位置；AVL 刪除可能沿祖先進行 $O(\log n)$ 次旋轉。
- 紅色 link 連接同一個 2-3-4 節點內的 key，黑色 link 連接不同節點。高度公式以 key 總數與根到外部 NIL 的邊數為計數基準。

## Revision

2026-10-04 依使用者要求直接修正來源中的黑高矛盾、哨兵用途、旋轉次數、紅色 link 對應與高度計數約定；原始手寫圖片保留，圖片內容未逐張校正。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

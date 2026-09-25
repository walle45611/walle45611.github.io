# 引線二元樹 (Threaded Binary Tree)

- source: `raw/my-vault/Note/Research/引線二元樹 (Threaded Binary Tree).md`
- source_sha256: `4cc5c76cc49fe61baad7f4b219e777498640fb8bfa1dc0ee917c4b8e24929fe0`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 ✅ 背景說明、📌 中序線索二元樹規則（Threading Rules）、📆 Thread Binary Tree 節點結構、判斷規則、🔧 Head 節點（特殊節點）、🔸 Case 1：空樹（empty）。

## Source Notes

- 有 $n$ 個節點，就有 $n−1$ 條實際存在的 child links（邊）。
- 若每個節點使用指標儲存左右子節點，則需要 $2n$ 個指標。
- ==其中會有：$2n - (n - 1) = n + 1$ 個是 null 指標（空指標）。==

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

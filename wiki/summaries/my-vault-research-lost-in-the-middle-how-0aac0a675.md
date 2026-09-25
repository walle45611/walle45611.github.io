# Lost in the Middle: How Language Models Use Long Contexts

- source: `raw/my-vault/Note/Research/Lost in the Middle How Language Models Use Long Contexts.md`
- source_sha256: `a99cb04fcd6bd30044ddec0c385469eea154fbe37779fd458dccb9d8c1d3f9fc`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

這篇檢驗模型是否真的能有效使用長上下文，而不只是在介面上接受更多 token。

## Source Notes

- 透過多文件問答及 key-value 檢索，控制相關資訊在輸入中的位置與上下文長度。
- 模型通常較容易使用開頭或結尾的資訊，中間位置表現較差；因此 context window 長度不能直接代表資訊利用能力，RAG 文件排序也可能影響答案。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

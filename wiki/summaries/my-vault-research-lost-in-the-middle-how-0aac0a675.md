# Lost in the Middle: How Language Models Use Long Contexts

- source: `raw/my-vault/Note/Research/@lostmiddlehow2024.md`
- source_sha256: `5d77d92ae9e612ff46bbeea029a4e4c7375457fa893dc17289d9ff9b621ae208`
- source_reviewed: 2026-10-09
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

## 2026-10-09 來源複核

來源已記錄讀完狀態，完整筆記仍待補。Table 1 區分 Closed-book 與只含答案文件的 Oracle；Figure 5 比較混入干擾文件後的位置效應。UUID key-value 檢索也可出現中間位置劣化，不能只歸因於 QA 語意推理。既有初讀摘要保留，新增結果補強 [[long-context-position-effects]]。

目前來源章節：Lost in the Middle: How Language Models Use Long Contexts、初讀摘要。

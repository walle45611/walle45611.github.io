# Benchmarking Retrieval-Augmented Generation for Medicine

- source: `raw/my-vault/Note/Research/Benchmarking Retrieval-Augmented Generation for Medicine.md`
- source_sha256: `78dc601035a0b4bfdd1f16a06d571e9d7025f0434ab61c4552fc289ec4f20c1f`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

這篇建立 MIRAGE 醫療問答評測集與 MedRAG 工具，系統比較語料庫、retriever 與生成模型的組合。

## Source Notes

- MIRAGE 包含五個資料集的 7,663 題，作者比較 41 種配置，分析多語料檢索與增加檢索內容的影響。
- RAG 在所測醫療 QA 中可提升正確率，但仍出現 lost-in-the-middle 現象。重要啟示是需整體評估檢索配置，而非假定加入更多文件一定更好。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

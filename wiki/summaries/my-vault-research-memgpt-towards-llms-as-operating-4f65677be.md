# MemGPT Towards LLMs as Operating Systems

- source: `raw/my-vault/Note/Research/MemGPT Towards LLMs as Operating Systems.md`
- source_sha256: `7913f63073c944e3dedcf11d163a518db3a61e30493c458ff0309b4e286bdf87`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 這篇論文想解決什麼問題？、核心類比：LLM context window = RAM、MemGPT 的記憶體階層、Main Context：模型當下真的看得到的內容、4.1 System Instructions、4.2 Working Context。

## Source Notes

- MemGPT 借用作業系統記憶體階層的概念，讓有限 context window 的模型管理更大的外部資訊。
- 系統在工作上下文與外部儲存之間調用、保存和檢索內容，藉由工具控制哪些資訊當下進入模型。
- 在長文件分析與跨多次對話中展示持續使用記憶的能力；它增加的是可管理的資訊範圍，並非一次把所有資料放入模型上下文。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

# vLLM + Gemma4 LoRA 安裝與啟動流程

- source: `raw/my-vault/Note/Tech/vLLM + Gemma4 LoRA 安裝與啟動流程.md`
- source_sha256: `edd20e3a63b844e6bf9d24d5600a481955b18d09a201e15ebd1f2033653802e9`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 安裝 Miniforge / Conda、建立 vLLM 環境、安裝 PyTorch CUDA 12.9、安裝 / 更新 vLLM 0.27.1 CUDA 12.9 wheel、4.1 vLLM 0.27.x 主要變更、登入 Hugging Face。

## Source Notes

- 本環境使用 CUDA 12.9 對應的 PyTorch wheel
- 如果目前環境是 vLLM 0.26.0，先移除舊版，再安裝固定的 0.27.1 CUDA 12.9、aarch64 wheel
- v0.27.1 是 v0.27.0 上的 patch release，新增量化 DSpark Markov heads 支援。

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)

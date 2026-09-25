# vLLM + Gemma 4 + LoRA：我踩過的兩個坑

- source: `raw/my-vault/Note/Tech/vllm-gemma-4-lora-我踩過的兩個坑.md`
- source_sha256: `f172595a2d49ca3a6428e59a57852c666316d30458ef70dae85dbf222279ae85`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 第一個坑：不是 LoRA rank 設錯，而是模型根本還不支援、第二個坑：升到 vLLM 0.20.0 後，換成 CUDA wheel 問題、最後的解法：照官方 Gemma 4 recipe 走 nightly/cu129、結論。

## Source Notes

- 最近在部署 Gemma 4 E4B + LoRA adapter 時，我連踩了兩個坑。第一個是 vLLM 版本本身還沒支援 Gemma 4 的 runtime LoRA ；第二個是 升版後又碰到 CUDA wheel 與環境不相容 。這篇記錄下來，給之後的自己少走一點冤枉路。
- 但後來確認都不是。真正原因是： 當時的 vLLM 版本還沒有支援 Gemma4ForConditionalGeneration 的 runtime LoRA 。
- 後來我查到相關 issue 是 vllm-project/vllm#39246 ，修正 PR 是 #39291 。主軸就是補上 Gemma4ForConditionalGeneration 的 LoRA support。

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)

# LLM Serving Compatibility

## Current View

LLM serving compatibility 指的是模型、adapter、inference runtime 與本機環境能否一起正常運作。某個配置在理論上可行，不代表在特定 runtime 或 CUDA 環境中真的能 serve；模型類別支援、runtime adapter support、以及 native extension / wheel 相容性都必須一起成立。

## Stable Conclusions

1. Serve base model 和 serve `base model + runtime adapter` 是兩個不同的能力檢查。
2. 如果 runtime 尚未實作特定模型類別的 LoRA / adapter hook，調參也不會讓它可用。
3. CUDA 相容性不能只看 `torch.cuda.is_available()`，還要驗證 native extension 是否能載入。
4. wheel 若對應到較新的 CUDA ABI，可能在 driver 與 PyTorch 看似正常的機器上仍然失敗。
5. 官方安裝 recipe 常常是版本約束的一部分，不只是安裝建議。
6. 權重之外的 tokenizer、config、tool parser、模型格式與 MTP metadata 也是 serving contract；缺少其中一項，模型可能載入卻無法正常工具使用。
7. 推論引擎、容器 image 與硬體架構必須一起匹配；在 ARM 或 unified-memory 平台上，image 是否存在與是否能在目標架構執行是獨立檢查項。
8. OpenAI-compatible endpoint 能把本地 serving 暴露給區域網路上的 harness，但 endpoint 的網路暴露、認證與工具權限仍需另外治理。
9. 模型支援 MTP 或 tool use，不代表所有 inference engine 都能使用；模型、runtime 與 parser 三者要用實際 smoke test 驗證。

## Working Heuristics

- 在調 LoRA rank 之前，先確認 serving stack 已支援該模型與 adapter 路徑。
- Debug CUDA 問題時，直接測 native extension import，比只看 GPU 可用性更可靠。
- 把 runtime、PyTorch、CUDA wheel 當成一組版本矩陣來檢查。
- 特定模型家族若有官方 recipe，優先照 recipe 驗證可用性。
- 先從 model card 建立檔案、格式、引擎、CUDA/架構與 tool parser 的 compatibility checklist。
- 在目標硬體上至少測一次普通生成、工具呼叫、OpenAI-compatible endpoint 與長 context/併發配置。
- 對 Docker 部署確認 image 架構、啟停、cache 位置與清理方式，避免一次性試驗污染主機環境。

## Open Questions

- 新模型家族的 runtime adapter support 在各個 vLLM 版本間會維持多久的穩定性？
- 這類相容性檢查最適合放在 CI、部署腳本，還是 runtime health check？

## Related Concepts

- [parameter-efficient-fine-tuning](./parameter-efficient-fine-tuning.md)
- [local-llm-deployment](./local-llm-deployment.md)
- [model-quantization](./model-quantization.md)
- [vllm-gemma-4-lora-two-pitfalls](../summaries/vllm-gemma-4-lora-two-pitfalls.md)

## Sources

- [vllm-gemma-4-lora-two-pitfalls](../summaries/vllm-gemma-4-lora-two-pitfalls.md)
- [qwen-3-8-27b-dgx-spark-agent-harness](../summaries/qwen-3-8-27b-dgx-spark-agent-harness.md)

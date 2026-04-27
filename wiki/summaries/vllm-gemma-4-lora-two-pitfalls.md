# vLLM + Gemma 4 + LoRA：我踩過的兩個坑

- source: `raw/vLLM + Gemma 4 + LoRA：我踩過的兩個坑.md`
- source link: https://blog.walle4561.com/20260427/2316/#more
- original title: vLLM + Gemma 4 + LoRA：我踩過的兩個坑
- author: [[Walle]]
- published: 2026-04-28
- created: 2026-04-28
- type: blog post

## Summary

這篇文章記錄作者在部署 `Gemma 4 E4B + LoRA adapter` 時踩到的兩個坑：其一是 vLLM 版本尚未支援 `Gemma4ForConditionalGeneration` 的 runtime LoRA；其二是升版後又碰到 CUDA wheel 與本機環境不相容。文章的重點不是 LoRA 參數，而是 serving runtime 與 binary 環境本身是否真的支援。

## Key Claims

1. `--max-lora-rank` 不是這次問題的根因。
2. vLLM 0.19.1 當時不支援 `Gemma4ForConditionalGeneration` 的 runtime LoRA。
3. `Gemma 4` base model 可以 serve，不代表 `Gemma 4 + runtime LoRA` 也一定可以。
4. 升到 vLLM 0.20.0 後，可能會遇到 CUDA 13 wheel 與 CUDA 12.8 環境不相容的問題。
5. `torch.cuda.is_available() == True` 仍不足以證明 vLLM native extension 可正常載入。
6. 驗證 vLLM 時，應額外測 `import vllm._C`。
7. 官方 Gemma 4 recipe 建議走 nightly / cu129 路線。

## Important Terms

- `Gemma4ForConditionalGeneration`
- runtime LoRA
- vLLM
- CUDA wheel
- `libcudart.so.13`
- `vllm._C`
- nightly
- cu129

## Related Concepts

- [parameter-efficient-fine-tuning](../concepts/parameter-efficient-fine-tuning.md)
- [llm-serving-compatibility](../concepts/llm-serving-compatibility.md)

## Alignment With Current Wiki

這篇來源補上了 PEFT 之外的實作層：LoRA 是否可用，不只看 rank 與 adapter 設定，也要看 serving runtime 是否支援該模型，以及 CUDA wheel 是否和環境真正相容。

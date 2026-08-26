# Model Quantization

## Current View

Model quantization reduces numeric precision of model weights or activations to lower memory usage and sometimes improve deployment feasibility. In QLoRA, quantization is used as part of the fine-tuning workflow so a large base model can be adapted on less hardware.

## Stable Conclusions

1. Lower precision generally reduces memory footprint.
2. Quantization is a systems trade-off, not a free compression trick; it can affect accuracy and training behavior.
3. QLoRA uses quantization to make fine-tuning cheaper while keeping the base model usable.
4. When the source's explanation is right, quantized representation can be temporary during training and the model can recover a higher-precision view later.
5. 作為粗略容量估算，FP8 約一個 byte/parameter、BF16 約兩個 byte/parameter；實際 serving 仍要加上格式開銷、KV cache、context、併發與系統保留空間。
6. MoE 的 active parameters 主要影響每個 token 的計算量，不代表載入時只需要 active parameters 對應的記憶體。
7. 量化格式的選擇應以目標任務的品質、延遲、context 與併發測試為準，不能只用壓縮比例或模型能否載入判斷。

## Working Heuristics

- Quantize when memory or deployment size is the limiting factor.
- Validate the downstream task, not just the compression ratio.
- Treat the exact bit-width as a task-specific choice, not a universal optimum.
- 先用權重大小加上 KV cache 與系統餘裕估算，再在代表性 prompt、工具呼叫和長 context 下比較品質與吞吐。

## Open Questions

- How aggressive quantization can be before quality degrades depends on model family and use case.
- The best quantization scheme may differ between training, inference, and storage.

## Related Concepts

- [parameter-efficient-fine-tuning](./parameter-efficient-fine-tuning.md)
- [local-llm-deployment](./local-llm-deployment.md)
- [llm-serving-compatibility](./llm-serving-compatibility.md)
- [lora-and-qlora-fine-tuning-explained-in-depth](../summaries/lora-and-qlora-fine-tuning-explained-in-depth.md)

## Sources

- [lora-and-qlora-fine-tuning-explained-in-depth](../summaries/lora-and-qlora-fine-tuning-explained-in-depth.md)
- [qwen-3-8-27b-dgx-spark-agent-harness](../summaries/qwen-3-8-27b-dgx-spark-agent-harness.md)

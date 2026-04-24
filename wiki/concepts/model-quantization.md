# Model Quantization

## Current View

Model quantization reduces numeric precision of model weights or activations to lower memory usage and sometimes improve deployment feasibility. In QLoRA, quantization is used as part of the fine-tuning workflow so a large base model can be adapted on less hardware.

## Stable Conclusions

1. Lower precision generally reduces memory footprint.
2. Quantization is a systems trade-off, not a free compression trick; it can affect accuracy and training behavior.
3. QLoRA uses quantization to make fine-tuning cheaper while keeping the base model usable.
4. When the source's explanation is right, quantized representation can be temporary during training and the model can recover a higher-precision view later.

## Working Heuristics

- Quantize when memory or deployment size is the limiting factor.
- Validate the downstream task, not just the compression ratio.
- Treat the exact bit-width as a task-specific choice, not a universal optimum.

## Open Questions

- How aggressive quantization can be before quality degrades depends on model family and use case.
- The best quantization scheme may differ between training, inference, and storage.

## Related Concepts

- [parameter-efficient-fine-tuning](./parameter-efficient-fine-tuning.md)
- [lora-and-qlora-fine-tuning-explained-in-depth](../summaries/lora-and-qlora-fine-tuning-explained-in-depth.md)

## Sources

- [lora-and-qlora-fine-tuning-explained-in-depth](../summaries/lora-and-qlora-fine-tuning-explained-in-depth.md)

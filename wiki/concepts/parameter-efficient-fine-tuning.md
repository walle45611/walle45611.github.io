# Parameter-Efficient Fine-Tuning

## Current View

Parameter-efficient fine-tuning (PEFT) is the class of methods that adapts a pre-trained model by training a small parameter set instead of updating all weights. In this wiki, LoRA is the main representative, and QLoRA is the memory-saving variant that combines low-rank adapters with quantization.

## Stable Conclusions

1. Full fine-tuning remains the most direct way to adapt a model but is expensive in memory and compute.
2. PEFT methods trade some flexibility for much lower training cost and easier experimentation.
3. LoRA expresses updates through low-rank matrices, so the choice of rank controls capacity and parameter count.
4. Which layers are adapted can matter as much as, or more than, the exact rank in some tasks.
5. Rank, alpha, and dropout should be treated as coupled hyperparameters rather than independent knobs.
6. PEFT is especially attractive when hardware is limited or when you want to try multiple task-specific adaptations cheaply.

## Working Heuristics

- Start with adapter-based fine-tuning before full fine-tuning when memory is constrained.
- Prefer adapting the relevant linear blocks broadly before spending effort on extreme rank increases.
- Treat rank as a capacity knob and alpha as a scale knob; validate both on the target task.
- Use dropout when overfitting appears or the dataset is narrow.

## Open Questions

- The best adapter placement and rank still depend heavily on architecture, dataset size, and task difficulty.
- Some tasks may benefit more from higher rank; others may mainly care about layer coverage.

## Related Concepts

- [model-quantization](./model-quantization.md)
- [lora-and-qlora-fine-tuning-explained-in-depth](../summaries/lora-and-qlora-fine-tuning-explained-in-depth.md)

## Sources

- [lora-and-qlora-fine-tuning-explained-in-depth](../summaries/lora-and-qlora-fine-tuning-explained-in-depth.md)

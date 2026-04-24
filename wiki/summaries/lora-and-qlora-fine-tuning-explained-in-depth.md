# LoRA & QLoRA Fine-tuning Explained In-Depth

- source: `raw/LoRA & QLoRA Fine-tuning Explained In-Depth.md`
- source link: https://www.youtube.com/watch?v=t1caDsMzWBk&t=139s
- original title: LoRA & QLoRA Fine-tuning Explained In-Depth
- author: [[Mark Hennings]]
- published: 2023-12-15
- created: 2026-04-25
- type: video

## Summary

This video explains why parameter-efficient fine-tuning exists, then contrasts full-parameter fine-tuning with LoRA and QLoRA. LoRA stores weight updates as low-rank matrices, which cuts trainable parameters and memory use; QLoRA adds quantization so the base model can be fine-tuned with even less memory. The video also argues that, in practice, adapting all linear transformer block layers matters more than chasing a high rank within the tested range, and it highlights rank, alpha, and dropout as the main LoRA knobs.

## Key Claims

1. Full-parameter fine-tuning updates every weight and is memory intensive.
2. LoRA decomposes updates into smaller matrices, so the trainable portion is much smaller than the full model.
3. For the QLoRA paper the speaker cites, rank between 8 and 256 had little effect on benchmark performance when LoRA was applied to all layers.
4. Alpha rescales the adapter update by `alpha / rank`.
5. Dropout is used to reduce overfitting; the video cites 0.1 for 7B/13B and 0.05 for 33B/65B models.
6. The video frames QLoRA as a further step beyond LoRA because it reduces memory further via quantization.

## Important Terms

- Pre-training
- Instruct tuning
- Safety tuning
- Full-parameter fine-tuning
- Low-rank adaptation
- Rank
- Alpha
- Dropout
- QLoRA
- Quantization

## Related Concepts

- [parameter-efficient-fine-tuning](../concepts/parameter-efficient-fine-tuning.md)
- [model-quantization](../concepts/model-quantization.md)

## Alignment With Current Wiki

This source expands the wiki's model-adaptation layer: it adds a reusable PEFT concept and a separate quantization concept, which are both likely to recur across future fine-tuning or deployment notes.

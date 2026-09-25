# QLoRA: Efficient Finetuning of Quantized LLMs

[[Assets/Note/Research/Papers/QLoRA - Efficient Finetuning of Quantized LLMs/QLoRA - Efficient Finetuning of Quantized LLMs.pdf|論文 PDF]]

## 初讀摘要

- QLoRA 將凍結的預訓練模型以 4-bit 儲存，再把梯度傳到可訓練 LoRA adapter，以降低微調記憶體需求。
- 主要設計包括 NF4、double quantization 與 paged optimizers，分別處理量化表示、量化常數及記憶體峰值。
- 原文展示大模型可在單張 GPU 上微調，並強調高品質指令資料的重要性；模型權重量化不表示所有計算及 adapter 都以 4-bit 執行。

摘要依據：PDF 摘要與導論。


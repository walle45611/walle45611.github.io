# LoRA: Low-Rank Adaptation of Large Language Models

[[Assets/Note/Research/Papers/LoRA - Low-Rank Adaptation of Large Language Models/LoRA - Low-Rank Adaptation of Large Language Models.pdf|論文 PDF]]

## 初讀摘要

- LoRA 針對大型模型全參數微調成本過高，改為只學習權重更新的低秩表示。
- 凍結原始權重，加入兩個較小可訓練矩陣，使更新寫成低秩矩陣乘積；可將更新合併回原權重以避免額外推論層。
- 原文在多種語言模型上取得接近或優於全量微調的結果，同時降低可訓練參數與記憶體需求；低秩大小及套用位置仍是重要設定。

摘要依據：PDF 摘要與導論。


# Neural Machine Translation by Jointly Learning to Align and Translate

[[Assets/Note/Research/Papers/Neural Machine Translation by Jointly Learning to Align and Translate/Neural Machine Translation by Jointly Learning to Align and Translate.pdf|論文 PDF]]

## 初讀摘要

- Bahdanau Attention 針對 encoder 將整句壓成固定長度向量的瓶頸，讓 decoder 每一步選取相關來源位置。
- 透過可微分的 soft alignment 對來源隱藏表示加權，產生當前目標詞所需的 context，並聯合學習對齊與翻譯。
- 原文英法翻譯結果顯示這種動態取用資訊的方式有效；理解它有助區分 encoder–decoder attention 與後來 Transformer 的 self-attention。

摘要依據：PDF 摘要與導論。


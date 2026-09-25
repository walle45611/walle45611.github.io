# LoRA: Low-Rank Adaptation of Large Language Models

- source: `raw/my-vault/Note/Research/LoRA - Low-Rank Adaptation of Large Language Models.md`
- source_sha256: `c72b2b6f698a59f62370730c82acc0fed708df19841ccc245ed2e8ae06781a46`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

LoRA 針對大型模型全參數微調成本過高，改為只學習權重更新的低秩表示。

## Source Notes

- 凍結原始權重，加入兩個較小可訓練矩陣，使更新寫成低秩矩陣乘積；可將更新合併回原權重以避免額外推論層。
- 原文在多種語言模型上取得接近或優於全量微調的結果，同時降低可訓練參數與記憶體需求；低秩大小及套用位置仍是重要設定。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

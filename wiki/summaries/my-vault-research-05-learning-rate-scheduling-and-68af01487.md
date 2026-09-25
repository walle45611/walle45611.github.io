# Learning Rate Scheduling and Warm-up

- source: `raw/my-vault/Note/Research/ML_DL_Obsidian_Notes/05 - Learning Rate Scheduling and Warm-up.md`
- source_sha256: `984ab567312230b3deee605c03c31f184675d19e3d35ea3fddd8e5164d2918ae`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 Decay 與計算例子、Linear warm-up 接 linear decay、手寫稿提到的 RAdam、Related、手寫原稿。

## Source Notes

- Scheduling 讓 learning rate 隨訓練步數改變。Decay 是下降階段；warm-up 是開始時逐漸升高。它們可以與 SGD、Momentum、Adam 等優化器搭配。
- 對 adaptive optimizer，還有每個參數的梯度尺度調整。全域 $\eta_t$ 與逐參數分母是兩個不同因素，不能把所有步長變化都叫 scheduling。
- 若 $\eta_0=0.1,\alpha=0.9$，前三個值為 $0.1,0.09,0.081$。當梯度固定為 $2$ 作為運算示例，對應移動量為 $0.2,0.18,0.162$。實際訓練會在每次更新後重新算梯度。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

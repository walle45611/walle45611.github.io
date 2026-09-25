# AdaGrad and Gradient Scaling

- source: `raw/my-vault/Note/Research/ML_DL_Obsidian_Notes/05 - AdaGrad and Gradient Scaling.md`
- source_sha256: `5ea75420cbfb5a78734ba946681cecc98714354dd92b59c4509dc60fcff380f8`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 手寫稿的 RMS 與標準 AdaGrad、兩步計算、與 RMSProp、Momentum、Adam 的關係、Related、手寫原稿。

## Source Notes

- AdaGrad 依每個參數過去累積的梯度平方調整步長。以下使用逐座標版本、固定全域 learning rate，初始累積量 $G_{0,i}=0$
- g_{t,i}=\frac{\partial\mathcal L}{\partial\theta_i},\qquad
- G_{t,i}=G_{t-1,i}+g_{t,i}^2

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

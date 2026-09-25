# Recurrent Neural Network (RNN)

- source: `raw/my-vault/Note/Research/Recurrent Neural Network (RNN).md`
- source_sha256: `3f2b16e78aa905c60deb5f2fe3065cd9903f93bb3ef01367e2455213d2a03561`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 遞迴更新與共享權重、手寫的兩步計算：線性簡化版、反向傳播：Backpropagation Through Time、衰減的數字、共享權重的梯度也要相加、與 Transformer 的連結。

## Source Notes

- 來源：手寫 RNN 筆記。RNN 依序讀入資料，將前一個時間步的 hidden state 與目前輸入結合，形成新的 hidden state。
- z_t=W_xx_t+W_hh_{t-1}+b_h,\qquad h_t=\tanh(z_t)
- $x_t$ 是目前輸入，$h_{t-1}$ 是前一步狀態。不同時間步共用同一組 $W_x,W_h,b_h$；不是每讀一個 token 就換一組權重。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

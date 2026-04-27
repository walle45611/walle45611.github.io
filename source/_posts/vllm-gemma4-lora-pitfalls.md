---
title: vLLM + Gemma 4 + LoRA：我踩過的兩個坑
tags:
  - vLLM
  - Gemma
  - LoRA
  - CUDA
  - LLM
categories:
  - AI
  - Machine Learning
abbrlink: '2316'
date: 2026-04-28 00:00:00
---

最近在部署 `Gemma 4 E4B + LoRA adapter` 時，我連踩了兩個坑。第一個是 **vLLM 版本本身還沒支援 Gemma 4 的 runtime LoRA**；第二個是 **升版後又碰到 CUDA wheel 與環境不相容**。這篇記錄下來，給之後的自己少走一點冤枉路。

<!--more-->

---

## 第一個坑：不是 LoRA rank 設錯，而是模型根本還不支援

我原本用 **vLLM 0.19.1** 去跑：

```bash
Gemma 4 E4B + LoRA adapter
```

結果直接報錯：

```text
ValueError: Gemma4ForConditionalGeneration does not support LoRA yet.
```

一開始我以為是這些地方有問題：

- `--max-lora-rank` 設錯
- LoRA adapter path 有誤
- adapter 轉換過程出了問題

但後來確認都不是。真正原因是：**當時的 vLLM 版本還沒有支援 `Gemma4ForConditionalGeneration` 的 runtime LoRA**。

換句話說：

| 情況 | 結果 |
|---|---|
| Gemma 4 base model | 可以 serve |
| Gemma 4 + runtime LoRA | 不一定可以 |

這兩件事不能混在一起看。

後來我查到相關 issue 是 [vllm-project/vllm#39246](https://github.com/vllm-project/vllm/issues/39246)，修正 PR 是 [#39291](https://github.com/vllm-project/vllm/pull/39291)。主軸就是補上 `Gemma4ForConditionalGeneration` 的 LoRA support。

---

## 第二個坑：升到 vLLM 0.20.0 後，換成 CUDA wheel 問題

升到 **vLLM 0.20.0** 後，我又遇到第二個錯：

```text
ImportError: libcudart.so.13
```

這次就不是 LoRA 了，而是 **CUDA wheel** 問題。

我的環境是：

- Driver: NVIDIA 550.x
- CUDA: 12.8
- torch: 2.11.0+cu128

但 vLLM 0.20.0 的安裝過程，可能會抓到 **CUDA 13** 版本的 wheel，於是 `vllm._C` 會去找 `libcudart.so.13`，而我的環境根本沒有。

這裡最重要的一點是：

```python
torch.cuda.is_available() == True
```

**不代表 vLLM 一定能跑。**

要再多測一個：

```bash
python -c "import vllm._C"
```

如果這一步都過了，才比較能確定 vLLM 的 CUDA extension 真的對上。

---

## 最後的解法：照官方 Gemma 4 recipe 走 nightly/cu129

後來比較正確的方向，是直接照官方 Gemma 4 的安裝建議，走 **nightly / cu129**：

```bash
uv pip install -U vllm --pre \
  --extra-index-url https://wheels.vllm.ai/nightly/cu129 \
  --extra-index-url https://download.pytorch.org/whl/cu129 \
  --index-strategy unsafe-best-match
```

官方安裝說明也可以參考這份：

- [vLLM GPU Installation Guide](https://docs.vllm.com.cn/en/latest/getting_started/installation/gpu.html#full-build-with-compilation)

---

## 結論

1. `--max-lora-rank 16` 沒錯，我的 LoRA rank 設定本身沒有問題。
2. 原始問題是 `Gemma4ForConditionalGeneration` 的 runtime LoRA support 還沒到位。
3. 後續問題是 vLLM wheel 抓到 CUDA 13，跟我的 CUDA 12.8 環境不合。
4. 驗證 vLLM 不要只看 `torch.cuda.is_available()`，要測 `import vllm._C`。

這次最大的心得是：**LLM 工程不是只有模型能不能跑，還有 wheel 到底是不是你環境真正能用的那個版本。**

有時候 bug 不在你的程式，而在你以為自己裝到的東西，其實不是你以為的那個東西。

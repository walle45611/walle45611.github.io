# vLLM + Gemma4 LoRA 安裝與啟動流程

> [!info] 目前版本基準（2026-08-15）
> 本流程以 **vLLM `0.27.1` + CUDA 12.9 + Linux `aarch64` + NVIDIA GH200** 為基準。`0.27.1` 是 `0.27.0` 上的 patch release；如果從 `0.26.0` 升級，請把它視為一次環境升級，並在安裝後重新驗證 PyTorch、Triton、CUDA extension 與 Gemma4 LoRA。
>
> 官方版本：[vLLM v0.27.1 Release Notes](https://github.com/vllm-project/vllm/releases/tag/v0.27.1)

## 1. 安裝 Miniforge / Conda

```bash
cd ~
wget https://github.com/conda-forge/miniforge/releases/latest/download/Miniforge3-Linux-aarch64.sh
bash Miniforge3-Linux-aarch64.sh
```

安裝過程最後詢問是否初始化 Conda，輸入：

```bash
yes
```

重新載入 shell：

```bash
source ~/.bashrc
```

確認 Conda 是否成功：

```bash
conda --version
```

## 2. 建立 vLLM 環境

```bash
conda create -n gemma-vllm python=3.12 -y
conda activate gemma-vllm
```

更新基本套件：

```bash
python -m pip install --upgrade pip setuptools wheel
```

## 3. 安裝 PyTorch CUDA 12.9

本環境使用 CUDA 12.9 對應的 PyTorch wheel：

```bash
python -m pip install torch torchvision torchaudio \
  --index-url https://download.pytorch.org/whl/cu129
```

檢查 PyTorch 與 CUDA：

```bash
python - <<'PY'
import torch
print("torch:", torch.__version__)
print("cuda:", torch.version.cuda)
print("available:", torch.cuda.is_available())
print("gpu:", torch.cuda.get_device_name(0))
PY
```

正常情況應看到類似：

```text
torch: 2.13.0+cu129
cuda: 12.9
available: True
gpu: NVIDIA GH200 480GB
```

## 4. 安裝 / 更新 vLLM 0.27.1 CUDA 12.9 wheel

如果目前環境是 vLLM `0.26.0`，先移除舊版，再安裝固定的 `0.27.1` CUDA 12.9、aarch64 wheel：

```bash
conda activate gemma-vllm

python -m pip uninstall -y vllm

python -m pip install --no-cache-dir --force-reinstall \
  "https://github.com/vllm-project/vllm/releases/download/v0.27.1/vllm-0.27.1+cu129-cp38-abi3-manylinux_2_28_aarch64.whl" \
  --extra-index-url https://download.pytorch.org/whl/cu129
```

不要直接使用未指定 CUDA backend 的：

```bash
pip install vllm
```

如果抓到不相容的 CUDA wheel，可能出現：

```text
ImportError: libcudart.so.13: cannot open shared object file: No such file or directory
```

### 4.1 vLLM 0.27.x 主要變更

- `v0.27.1` 是 `v0.27.0` 上的 patch release，新增量化 DSpark Markov heads 支援。
- 真正的大幅環境變更在 `v0.27.0`：PyTorch `2.13.0`、torchvision `0.28.0`、Triton `3.7.1`；官方明確標示這是 breaking environment change。
- `v0.27.0` 新增或擴充 Kimi K3、Qwen3.5、Model Runner V2 的 embedding / classification、FlashAttention 4，以及大型部署 fault tolerance 等能力。
- 因為 PyTorch / Triton 已切換，從 `0.26.0` 升級時保留 `--no-cache-dir --force-reinstall`，不要只執行 `pip install -U vllm`。

檢查 vLLM、PyTorch 與 CUDA extension 是否正常：

```bash
python - <<'PY'
import platform
import torch
import vllm
import vllm._C_stable_libtorch

print("vLLM:", vllm.__version__)
print("vLLM path:", vllm.__file__)
print("PyTorch:", torch.__version__)
print("CUDA:", torch.version.cuda)
try:
    import importlib.metadata
    print("Triton:", importlib.metadata.version("triton"))
except importlib.metadata.PackageNotFoundError:
    print("Triton: not installed")
print("Architecture:", platform.machine())
print("GPU:", torch.cuda.get_device_name(0))
print("CUDA available:", torch.cuda.is_available())
print("Native extension: vllm._C_stable_libtorch OK")
print("rms_norm op registered:", hasattr(torch.ops._C, "rms_norm"))
PY
```

正常情況應看到：

```text
vLLM: 0.27.1
PyTorch: 2.13.0+cu129
CUDA: 12.9
Triton: 3.7.1
Architecture: aarch64
GPU: NVIDIA GH200 480GB
CUDA available: True
Native extension: vllm._C_stable_libtorch OK
rms_norm op registered: True
```

如果 `vllm --version` 與 `python -c "import vllm; print(vllm.__version__)"` 顯示不同版本，代表系統中可能有多個 vLLM 指令。可以檢查：

```bash
which -a vllm
which python
python -c "import sys; print(sys.executable)"
python -c "import vllm; print(vllm.__version__); print(vllm.__file__)"
```

若 `/usr/local/bin/vllm` 是舊版殘留，且不是 dpkg 管理：

```bash
dpkg -S /usr/local/bin/vllm
```

如果顯示：

```text
dpkg-query: no path found matching pattern /usr/local/bin/vllm
```

可以刪除舊殘留：

```bash
rm -f /usr/local/bin/vllm
hash -r
```

## 5. 登入 Hugging Face

如果模型需要權限，先登入：

```bash
huggingface-cli login
```

## 6. 啟動 vLLM + Gemma4 + LoRA

先設定環境變數：

```bash
export VLLM_ALLOW_RUNTIME_LORA_UPDATING=True
export VLLM_USE_FLASHINFER_SAMPLER=0
```

啟動服務：

```bash
vllm serve google/gemma-4-31B-it \
  --host 0.0.0.0 \
  --port 8000 \
  --dtype bfloat16 \
  --quantization fp8 \
  --kv-cache-dtype fp8 \
  --max-model-len 65536 \
  --gpu-memory-utilization 0.92 \
  --max-num-seqs 1 \
  --max-num-batched-tokens 16384 \
  --enable-auto-tool-choice \
  --tool-call-parser gemma4 \
  --enable-chunked-prefill \
  --enable-prefix-caching \
  --async-scheduling \
  --enable-lora \
  --lora-modules cmu-sage-ai=/home/quanta/cmu-sage-ai/training/outputs/cmu-sage-ai-20260609-180428 \
  --max-loras 1 \
  --max-lora-rank 16 \
  --limit-mm-per-prompt '{"image":0,"audio":0,"video":0}' \
  --load-format fastsafetensors \
  --generation-config vllm \
  --reasoning-parser gemma4 \
  --default-chat-template-kwargs '{"enable_thinking": true}'
```

注意：

```bash
--enable-lora
```

只是開啟 LoRA 功能。

```bash
--lora-modules cmu-sage-ai=/path/to/lora/checkpoint
```

才是真的載入 LoRA adapter。

## 7. 測試 API

檢查模型清單：

```bash
curl http://localhost:8000/v1/models
```

如果有看到：

```text
google/gemma-4-31B-it
cmu-sage-ai
```

代表 base model 和 LoRA adapter 都有載入。

測試聊天：

```bash
curl http://localhost:8000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "cmu-sage-ai",
    "messages": [
      {
        "role": "user",
        "content": "請根據病例產生中醫處方"
      }
    ],
    "temperature": 0.7,
    "max_tokens": 1024
  }'
```

## 8. 常見問題

### 8.1 `ImportError: libcudart.so.13`

如果出現：

```text
ImportError: libcudart.so.13: cannot open shared object file: No such file or directory
```

代表 vLLM wheel 和 CUDA 版本不一致。通常是裝到了 CUDA 13 版本的 vLLM wheel，但目前環境使用的是 PyTorch cu129。

處理方式：

```bash
python -m pip uninstall -y vllm

python -m pip install --no-cache-dir --force-reinstall \
  "https://github.com/vllm-project/vllm/releases/download/v0.27.1/vllm-0.27.1+cu129-cp38-abi3-manylinux_2_28_aarch64.whl" \
  --extra-index-url https://download.pytorch.org/whl/cu129
```

然後確認：

```bash
python -c "import vllm._C_stable_libtorch; print('vllm._C_stable_libtorch ok')"
```

### 8.2 `Gemma4ForConditionalGeneration does not support LoRA yet`

如果出現：

```text
Gemma4ForConditionalGeneration does not support LoRA yet
```

代表目前 vLLM 對該模型架構的 LoRA 支援有問題。先確認環境已更新到本流程的 `0.27.1`，不要降回舊版：

```bash
vllm --version
```

建議使用：

```text
0.27.1
```

如果仍然出現錯誤，再檢查 Gemma4 model class、LoRA adapter 的 target modules，以及實際載入的 vLLM 路徑。

### 8.3 `audio_tower / vision_tower`

如果出現：

```text
audio_tower
vision_tower
```

代表 LoRA adapter 可能包含多模態 tower，或 adapter 結構不乾淨。建議確認 LoRA checkpoint 是否只包含文字模型需要的 adapter 權重。

### 8.4 `--limit-mm-per-prompt` 格式錯誤

不同 vLLM 版本的 `--limit-mm-per-prompt` 格式可能不同。

在目前 vLLM 0.27.1 環境中，使用 JSON 格式：

```bash
--limit-mm-per-prompt '{"image":0,"audio":0,"video":0}'
```

如果其他版本 vLLM 出現：

```text
Each item should be in the form KEY=VALUE
```

代表該版本可能要求其他格式。此時應優先確認 vLLM 版本與該版本的 CLI schema，不要直接沿用舊指令。

### 8.5 GPU 記憶體不足

如果 GPU 記憶體不夠，優先調低：

```bash
--max-model-len 31296
```

例如改成：

```bash
--max-model-len 24576
```

或調低：

```bash
--gpu-memory-utilization 0.95
```

### 8.6 檢查目前環境是否乾淨

```bash
which -a vllm
vllm --version
python -c "import vllm; print(vllm.__version__); print(vllm.__file__)"
python -c "import torch; print(torch.__version__); print(torch.version.cuda); print(torch.cuda.get_device_name(0))"
python -c "import importlib.metadata; print('Triton:', importlib.metadata.version('triton'))"
python -c "import vllm._C_stable_libtorch; print('vllm._C_stable_libtorch ok')"
```

理想結果：

```text
/root/miniforge3/envs/gemma-vllm/bin/vllm
0.27.1
0.27.1
torch: 2.13.0+cu129
cuda: 12.9
Triton: 3.7.1
NVIDIA GH200 480GB
vllm._C_stable_libtorch ok
```

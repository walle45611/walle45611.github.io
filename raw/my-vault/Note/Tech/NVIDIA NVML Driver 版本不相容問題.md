# NVIDIA NVML / Driver 版本不相容問題

## 問題現象

執行：

```bash
nvidia-smi
```

或：

```bash
nvitop
```

出現：

```text
NVML ERROR: RM has detected an NVML/RM version mismatch.
```

或：

```text
Failed to initialize NVML: Driver/library version mismatch
```

---

## 問題原因

目前系統載入的 NVIDIA Kernel Driver 版本是：

```text
NVRM version: 550.90.07
```

但 `nvidia-smi` / `nvitop` 實際載入的 NVML library 是：

```text
/usr/lib/aarch64-linux-gnu/libnvidia-ml.so.610.43.02
```

因此形成版本不相容：

| 元件 | 版本 |
| --- | --- |
| Kernel NVIDIA Driver / RM | `550.90.07` |
| NVML userspace library | `610.43.02` |

```text
Kernel NVIDIA Driver / RM
550.90.07
        │
        │ version mismatch
        ▼
NVML userspace library
610.43.02
```

也就是：

```text
Driver: 550.90.07
NVML:   610.43.02
```

兩者版本不同，導致 NVML 無法正常和 NVIDIA Kernel Driver 溝通。

---

## 確認方式

### 1. 查看目前 Kernel Driver 版本

```bash
cat /proc/driver/nvidia/version
```

目前結果：

```text
NVRM version: NVIDIA UNIX Open Kernel Module for aarch64 550.90.07
```

### 2. 查看 NVML library 實際版本

```bash
readlink -f /lib/aarch64-linux-gnu/libnvidia-ml.so.1
```

目前結果：

```text
/usr/lib/aarch64-linux-gnu/libnvidia-ml.so.610.43.02
```

### 3. 查看程式實際載入哪一份 NVML

```bash
LD_DEBUG=libs nvidia-smi 2>&1 | grep libnvidia-ml
```

原本實際載入：

```text
/lib/aarch64-linux-gnu/libnvidia-ml.so.1
```

而該 symbolic link 最後指向：

```text
/usr/lib/aarch64-linux-gnu/libnvidia-ml.so.610.43.02
```

---

## 解決方式

因為目前 Kernel Driver 是 `550.90.07`，因此讓程式優先載入對應的 `550.90.07` NVML library。

已建立相容 library 目錄：

```text
/home/quanta/.local/nvidia-550
```

然後設定：

```bash
export LD_LIBRARY_PATH=/home/quanta/.local/nvidia-550:${LD_LIBRARY_PATH}
```

這樣 dynamic linker 會優先從：

```text
/home/quanta/.local/nvidia-550
```

載入：

```text
libnvidia-ml.so.1
```

而不是系統預設的 `610.43.02`。

---

## 修正後結果

設定：

```bash
export LD_LIBRARY_PATH=/home/quanta/.local/nvidia-550:${LD_LIBRARY_PATH}
```

後執行：

```bash
nvidia-smi
```

正常顯示：

```text
NVIDIA-SMI 550.90.07
Driver Version: 550.90.07
CUDA Version: 12.8
```

`nvitop` 也恢復正常：

```text
NVITOP 1.3.2
Driver Version: 550.90.07
CUDA Driver Version: 12.8
```

因此目前正確的組合為：

```text
Kernel Driver
550.90.07
        │
        ▼
NVML
550.90.07
```

---

## 永久設定

如果只希望在 `vllm` Conda environment 中套用，建議使用 Conda activation hook。

建立目錄：

```bash
mkdir -p "$CONDA_PREFIX/etc/conda/activate.d"
```

建立設定：

```bash
cat > "$CONDA_PREFIX/etc/conda/activate.d/nvidia-550.sh" <<'EOF'
export LD_LIBRARY_PATH="/home/quanta/.local/nvidia-550:${LD_LIBRARY_PATH:-}"
EOF
```

之後每次：

```bash
conda activate vllm
```

都會自動載入 NVIDIA 550 相容 library。

---

## 注意事項

這個問題與以下項目無直接關係：

- Conda `conda-forge`
- `channel_priority strict`
- PyTorch
- vLLM
- `nvitop`
- pip index
- CUDA Toolkit 本身

真正問題是：

```text
NVIDIA Kernel Driver 550.90.07
與
NVML userspace library 610.43.02
版本不一致
```

目前採用 `LD_LIBRARY_PATH` 優先載入 NVIDIA 550 NVML library 的方式解決，不修改系統 `/usr/lib` 的 symbolic link。

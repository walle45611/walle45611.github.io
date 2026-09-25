# Miniconda 安裝疑難雜症

## 1. Miniconda 安裝與 Conda 初始化問題

### 1.1 下載 Miniconda

確認架構：

```bash
uname -m
```

如果是 GH200 / ARM64：

```text
aarch64
```

下載 ARM64 版本：

```bash
wget https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-aarch64.sh
```

如果是一般 x86_64：

```bash
wget https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh
```

---

### 1.2 安裝 Miniconda

ARM64：

```bash
bash Miniconda3-latest-Linux-aarch64.sh
```

x86_64：

```bash
bash Miniconda3-latest-Linux-x86_64.sh
```

安裝路徑建議：

```text
/home/$USER/miniconda3
```

安裝最後詢問：

```text
Do you wish the installer to initialize Miniconda3?
```

輸入：

```text
yes
```

---

### 1.3 安裝後找不到 conda

症狀：

```bash
conda: command not found
```

確認 Miniconda 是否存在：

```bash
ls ~/miniconda3
```

正常會看到：

```text
bin
condabin
envs
pkgs
```

代表安裝成功，只是 shell 沒有載入。

---

### 1.4 初始化 Conda

執行：

```bash
~/miniconda3/bin/conda init bash
```

重新載入：

```bash
source ~/.bashrc
```

確認：

```bash
conda --version
```

---

### 1.5 手動啟用 Conda

如果初始化沒有生效：

```bash
source ~/miniconda3/bin/activate
```

成功：

```text
(base) user@host:~$
```

---

### 1.6 Jupyter / Kubernetes 環境沒有載入 Conda

確認 shell：

```bash
echo $SHELL
```

如果：

```text
/bin/sh
```

切換 bash：

```bash
bash
source ~/.bashrc
```

---

# 2. Conda 建立環境與套件安裝問題

## 2.1 CondaToSNonInteractiveError

錯誤：

```text
CondaToSNonInteractiveError:
Terms of Service have not been accepted
```

原因：

新版 Conda 要求接受 Anaconda channel ToS。

---

### 解決方式

接受條款：

```bash
conda tos accept --override-channels --channel https://repo.anaconda.com/pkgs/main

conda tos accept --override-channels --channel https://repo.anaconda.com/pkgs/r
```

建立環境：

```bash
conda create -n fine-tuning python=3.12 -y
```

---

## 2.2 建議使用 conda-forge

AI / CUDA 環境建議避免 defaults 與 conda-forge 混用。

設定：

```bash
conda config --remove channels defaults

conda config --add channels conda-forge

conda config --set channel_priority strict
```

建立環境：

```bash
conda create -n fine-tuning python=3.12 -y
```

---

# uv 與 Conda 使用方式

## 2.3 在 Conda environment 安裝 uv

啟用環境：

```bash
conda activate fine-tuning
```

安裝：

```bash
pip install uv
```

確認：

```bash
uv --version
```

---

## 2.4 使用 uv 安裝 dependency group

如果 pyproject.toml：

```toml
[dependency-groups]

training = [
    ...
]
```

安裝到目前 Conda environment：

```bash
uv pip install --group training
```

---

## 2.5 不使用 uv sync

`uv sync` 會讓 uv 管理專案環境。

通常會建立：

```text
project
├── pyproject.toml
├── uv.lock
└── .venv
```

如果已經使用 Conda：

```text
Conda
└── fine-tuning
```

不要再建立：

```text
Conda
└── fine-tuning
    └── .venv
```

避免雙重環境。

---

# APT 套件安裝問題

## 3.1 apt install 出現 dependency error

例如：

```bash
apt install gh
```

錯誤：

```text
python3 :
Depends: libpython3-stdlib (= 3.12.3-0ubuntu2.1)

but 3.12.3-0ubuntu2 is to be installed
```

原因：

Ubuntu 系統套件版本不一致。

與 Conda 無關。

---

## 3.2 修復 APT

更新：

```bash
apt update
```

模擬修復：

```bash
apt-get -s --fix-broken install
```

確認後：

```bash
apt --fix-broken install
```

重新設定：

```bash
dpkg --configure -a
```

---

## 3.3 使用 Conda 安裝 GitHub CLI

如果只是需要 gh：

```bash
conda install -c conda-forge gh -y
```

確認：

```bash
gh --version
```

---

# 建議環境架構

推薦：

```text
Conda environment

fine-tuning
├── Python 3.12
├── PyTorch
├── CUDA libraries
├── uv
├── Transformers
├── vLLM
└── Training dependencies
```

避免：

```text
Conda environment

fine-tuning
└── .venv
    └── uv packages
```

適合 GPU Training、PyTorch、vLLM 的管理方式：

```text
Conda 管 Python 與 CUDA 環境
uv pip 管 Python dependencies
```

## 啟動與狀態

```bash
colima start --runtime containerd
colima status
colima stop
colima delete
```

## Docker 與 Kubernetes 支援

Colima 可以同時支援 Docker runtime 與 Kubernetes (k3s)：

```bash
colima start --runtime docker      # 使用 Docker runtime
colima start --kubernetes          # 啟用內建 k3s Kubernetes
```

## 安裝 nerdctl

```bash
colima nerdctl install
# 可能出現：
# INFO[0000] /usr/local/bin not writable, sudo password required to install nerdctl binary
# 直接用 sudo 執行：
sudo colima nerdctl install
```

## 基本操作（安裝前）

```bash
colima nerdctl -- ps
colima nerdctl -- images
colima nerdctl -- build -t myapp .
colima nerdctl -- run -it --rm ubuntu bash
```

## 基本操作（安裝後）

```bash
nerdctl ps
nerdctl images
nerdctl build -t myapp .
nerdctl run -it --rm ubuntu bash
```

## Compose 指令

```bash
nerdctl compose up -d
nerdctl compose down
nerdctl compose ps
nerdctl compose build
```

## 指定重建與停止容器

```bash
# 重建指定服務的映像與容器
nerdctl compose build <service_name>
nerdctl compose up -d --build <service_name>

# 停止並移除指定容器
nerdctl stop <container_id_or_name>
nerdctl rm <container_id_or_name>

# 重新啟動指定容器
nerdctl restart <container_id_or_name>

nerdctl compose up -d
nerdctl compose down
nerdctl compose ps
nerdctl compose build
```

---

## 為何不用 OrbStack，改用 Colima（containerd）

- **開源與可攜**：Colima 基於 Lima，設定透明、可版本控管；OrbStack 為封閉商業產品。
    
- **更貼近生產環境**：多數 K8s 節點採用 **containerd**。本地用 containerd + nerdctl，工具與行為與伺服器一致（映像倉庫、cgroup、鏡像管理）。
    
- **低干擾、資源單純**：Colima 不常駐重型 daemon，資源可精準配置（`--cpu/--memory/--disk`）。
    
- **Kubernetes 內建選項**：`colima start --kubernetes` 一鍵啟動 k3s，利於本地測試。
    
- **無需 Docker 相容層**：避免 Docker 封裝差異；以 `nerdctl compose` 覆蓋常見工作流。
    
- **授權成本**：Colima 免費；OrbStack 長期可能涉及授權費用與鎖定風險。
    

### 關於 containerd

- **核心角色**：containerd 是 CNCF 項目，由 Docker 捐贈，現為 Kubernetes 預設容器執行時 (Runtime)。
    
- **功能定位**：僅專注於容器的「拉取映像、建立、執行、監控、刪除」等核心功能，避免冗餘。
    
- **與 Docker 的差異**：Docker = CLI + API + build 工具 + containerd。直接使用 containerd 可以減少一層包裝，更輕量。
    
- **工具鏈**：`nerdctl` 提供 Docker 相容的 CLI 體驗（`nerdctl run`, `nerdctl compose`），降低學習成本。
    
- **生產一致性**：幾乎所有大型雲端 K8s 服務 (EKS, GKE, AKS) 都以 containerd 為預設 runtime，本地用 containerd 意味著與線上環境行為一致。
    
- **效能優勢**：容器啟動速度快、資源消耗低，適合開發機模擬大規模服務。
    

> 總結：Colima 搭配 containerd = 開源、輕量、與生產一致的本地容器方案，避免 Docker/OrbStack 額外封裝或授權依賴。
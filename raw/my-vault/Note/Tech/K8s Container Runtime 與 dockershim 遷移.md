# K8s Container Runtime 與 dockershim 遷移

## 移除的是 dockershim

Kubernetes 從 v1.24 移除內建 dockershim。Docker Engine 本身不提供 CRI，因此常改用 containerd 或 CRI-O；需要保留 Docker Engine 時可透過 cri-dockerd 銜接。這不影響 Docker／Buildx 建置相容映像，再推送至 registry 供 Kubernetes 使用。

```text
舊路徑：kubelet → CRI → dockershim → Docker Engine → containerd → runc
常見路徑：kubelet → CRI → containerd 的 CRI plugin → containerd → runc
保留 Docker：kubelet → CRI → cri-dockerd → Docker Engine
```

CRI 是 kubelet 與 runtime 的介面；containerd 管理映像及容器生命週期；runc 是依 OCI runtime specification 啟動容器的低階 runtime。「dockerx」若指 Docker Buildx，與 dockershim 是不同元件。

依據：[Migrating from dockershim](https://kubernetes.io/docs/tasks/administer-cluster/migrating-from-dockershim/)。

## 管理員盤點與遷移

| 檢查項目 | 要確認的內容 |
|---|---|
| 現有 runtime | 各 Node 的 runtime 名稱、版本與實際 CRI endpoint |
| Docker 依賴 | docker.sock、Docker API、Docker CLI、自動化腳本與 DaemonSet |
| cgroup | kubelet 與 runtime 的 cgroup driver 一致；systemd 主機通常使用 systemd |
| 映像來源 | registry mirror、私有 registry 憑證、imagePullSecrets、離線映像 |
| 整合元件 | logging、monitoring、安全 agent、GPU runtime 與 RuntimeClass |
| 可用性 | 剩餘容量、PodDisruptionBudget、單副本服務、local volume、emptyDir 資料 |

runtime 設定檔格式會隨版本改變，尤其 containerd 不同主版本的 plugin 設定路徑不可混用。參考 [Container Runtimes](https://kubernetes.io/docs/setup/production-environment/container-runtimes/)。

建議先完成一台測試節點，再逐台遷移或以新節點替換舊節點。以下為流程範例，並未對現有叢集執行：

```bash
kubectl get nodes -o wide
kubectl get pdb -A
kubectl cordon node1
kubectl drain node1 --ignore-daemonsets
# 依發行版程序更換 runtime、更新 endpoint，並重啟所需服務
# 在該 Node 上，以正確 endpoint 執行：
crictl info
crictl ps
kubectl get nodes
kubectl get pods -A -o wide
# 驗證應用、DNS、網路、儲存及監控後：
kubectl uncordon node1
```

若 drain 因 emptyDir 資料而停止，先確認資料能否丟棄；不要直接把 `--delete-emptydir-data` 當作固定參數。DaemonSet 不會被上述 drain 移除，也要另外驗證其相容性。

## 操作工具的差異

管理 Pod 優先使用 `kubectl logs`、`kubectl exec`、`kubectl describe`；Node runtime 排錯使用 `crictl ps`、`crictl logs`、`crictl inspect`、`crictl images`。

Docker 與 Kubernetes runtime 的 image store／namespace 不一定相同，`docker pull` 後不能假定 `crictl images` 一定看得到。建置流程通常可保留，依賴 Docker daemon 的節點管理流程則需要調整。

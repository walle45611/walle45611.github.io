---
title: MicroK8s 完整部署指南：從安裝到生產級應用
tags:
  - Kubernetes
  - MicroK8s
  - DevOps
  - Container
  - System Administration
categories:
  - System Administration
  - Kubernetes
abbrlink: ecf5
date: 2025-08-08 08:00:52
---

在最近專題開發的過程中，我反覆安裝與測試了多次 MicroK8s，期間踩了不少坑，也累積了許多實戰經驗。因此，我決定整理這篇 Blog，讓自己以後部署時可以有一份可直接複製使用的指令手冊，也幫助正在學習 Kubernetes 的朋友少走一些彎路。

本文是基於 MicroK8s 官方文件、Helm、Argo CD Image Updater 等官方說明整合的安裝流程，並修正了常見名詞與設定錯誤，方便直接在 Ubuntu 環境中快速部署高可用 Kubernetes 集群。

<!--more-->

---

## MicroK8s 能否用於 Production？

在 Canonical 官方文章中，MicroK8s 被明確描述為 **「強大、輕量、可靠的 production‑ready Kubernetes 發行版」**，並稱其為「企業級 Kubernetes 發行版」，內建多種生產等級插件（如 Istio、Knative、Grafana、Cilium 等）。官方寫道：

> 「無論你是在執行生產環境還是想探索 Kubernetes，MicroK8s 都能滿足你的需求。」

來源：[Ubuntu Blog - Introduction to MicroK8s](https://ubuntu.com/blog/introduction-to-microk8s-part-1-2)

Canonical 官方產品經理也在社群中表示：

> 「MicroK8s 可以用於生產環境，但具體是否適合仍取決於你的需求情景。」

來源：[ServerFault 討論](https://serverfault.com/questions/941857/is-microk8s-suitable-for-production-environments-or-is-it-just-for-development)

此外，社群中也有正面實戰經驗：

> 「We recommend microk8s. It's just solid and stable… I've run a 3‑node microk8s production cluster before. It worked well.」

來源：[Reddit - Experiences in production with k3s & microk8s](https://www.reddit.com/r/kubernetes/comments/zrn62i/experiences_in_production_with_k3s_microk8s/)

### 結論與建議

|情況|建議作法|
|---|---|
|淺層部署、邊緣設備、CI/CD、學習、PoC|非常適合使用 MicroK8s，快速上線|
|中型生產環境|MicroK8s 是可行選擇，建議搭配 HA、RBAC、MetalLB 等插件提升穩定性與安全性|
|大規模生產、多雲混合部署、雲原生整合需求|建議評估 kubeadm、RKE2 或雲端託管 Kubernetes 服務|

---

## 系統需求

- **作業系統**：Ubuntu/Linux（建議 Ubuntu 20.04+）

- **硬體**：至少 4GB RAM，2 CPU cores

- **網路**：可訪問 GitHub、Docker Hub、GHCR 等 registry

- **權限**：具備 sudo 權限

---

## 0. 安裝與基本設定

### 安裝 MicroK8s

```bash
sudo snap install microk8s --classic
sudo usermod -aG microk8s $USER
sudo chown -f -R $USER ~/.kube
newgrp microk8s
microk8s status --wait-ready
```

### 啟用必要插件

```bash
microk8s enable dns
microk8s enable hostpath-storage
microk8s enable metrics-server
microk8s enable ingress
microk8s enable metallb:192.168.1.200-192.168.1.250
microk8s enable dashboard
```

> **註**：`hostpath-provisioner` 為舊名稱，請使用 `hostpath-storage`。

### 匯出 kubeconfig

```bash
mkdir -p ~/.kube
microk8s config > ~/.kube/config
```

---

## 1. 節點管理

### 加入 Worker 節點

**Master 節點執行：**

```bash
microk8s add-node
# 範例：microk8s join 192.168.1.100:25000/xxxx --worker
```

**Worker 節點執行：**

```bash
sudo snap install microk8s --classic
sudo usermod -aG microk8s $USER
newgrp microk8s
microk8s join 192.168.1.100:25000/xxxx --worker
```

### 加入控制平面節點

```bash
microk8s add-node
microk8s join 192.168.1.100:25000/xxxx
```

### 節點標籤與污點

```bash
kubectl label nodes <node-name> node-role.kubernetes.io/control-plane=control-plane
kubectl taint nodes <node-name> node-role.kubernetes.io/control-plane=:NoSchedule
```

---

## 2. 安裝 Helm

### 方法一（獨立安裝）

```bash
curl https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 | bash
helm version
```

### 方法二（MicroK8s 內建）

```bash
microk8s enable helm3
microk8s helm3 version
```

> 二擇一使用，避免混用 `helm` 與 `microk8s helm3`。

---

## 3. 安裝 Argo CD（Helm）

```bash
kubectl create namespace argocd
helm repo add argo https://argoproj.github.io/argo-helm
helm repo update
helm install argocd argo/argo-cd -n argocd
kubectl get pods -n argocd
```

### 設定 NodePort 存取

```bash
kubectl -n argocd patch svc argocd-server \
  -p '{"spec":{"type":"NodePort","ports":[{"name":"https","port":443,"protocol":"TCP","targetPort":8080,"nodePort":30443}]}}'
```

### 取得初始密碼

```bash
kubectl -n argocd get secret argocd-initial-admin-secret \
  -o jsonpath="{.data.password}" | base64 -d && echo
```

---

## 4. Dashboard 存取

官方建議使用代理方式：

```bash
microk8s dashboard-proxy
```

會輸出本地訪問 URL 與 Token。

> 也可改成 NodePort（非官方推薦）：

```bash
kubectl -n kube-system patch svc kubernetes-dashboard \
  -p '{"spec":{"type":"NodePort","ports":[{"name":"https","port":443,"protocol":"TCP","targetPort":8443,"nodePort":30444}]}}'
```

---

## 5. 安裝 Sealed Secrets（Helm）

```bash
helm repo add sealed-secrets https://bitnami-labs.github.io/sealed-secrets
helm repo update
helm install sealed-secrets sealed-secrets/sealed-secrets -n kube-system
kubectl get pods -n kube-system | grep sealed-secrets
```

安裝 kubeseal CLI：

```bash
wget https://github.com/bitnami-labs/sealed-secrets/releases/download/v0.24.0/kubeseal-0.24.0-linux-amd64.tar.gz
tar -xvzf kubeseal-0.24.0-linux-amd64.tar.gz
sudo install -m 755 kubeseal /usr/local/bin/kubeseal
rm kubeseal-0.24.0-linux-amd64.tar.gz kubeseal
```

---

## 6. 安裝 Argo CD Image Updater

```bash
helm install argocd-image-updater argo/argocd-image-updater -n argocd
```

### 設定 GHCR 認證（修正版）

```bash
kubectl -n argocd create secret generic ghcr-creds \
  --from-literal=creds='<gh-username>:<gh-personal-access-token>'
```

### 修改 `registries.conf`

```yaml
data:
  registries.conf: |
    registries:
    - name: GitHub Container Registry
      api_url: https://ghcr.io
      prefix: ghcr.io
      credentials: secret:argocd/ghcr-creds#creds
```

重啟服務：

```bash
kubectl rollout restart deployment/argocd-image-updater -n argocd
```

---

## 7. 常用疑難排解

- **MetalLB IP 範圍重設**

```bash
microk8s disable metallb
microk8s enable metallb:192.168.1.200-192.168.1.250
```

- **節點無法加入**

```bash
sudo ufw allow 25000
sudo ufw allow 16443
```

- **檢查 MicroK8s 狀態**

```bash
microk8s status
sudo journalctl -u snap.microk8s.daemon -f
```

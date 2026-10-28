## AZ-104

[[Cloud Shell]]
[[Azure Blob Lifecycle Policy]]
[[Microsoft EntraID]]

## Windows Server 系統管理

### Active Directory 相關
[[Windows Server Active Directory 介紹]]
[[Window Server Active Directory Certificate Services]]
[[Windows Server AD Group User And OU]]
[[Windows Server Active Directory Operation Master（FSMO）角色整理]]
[[GPO Basic + PowerShell 操作整理]]
[[Windows Server Trust]]

### 網路服務
[[Windows Server DHCP 設定指南]]
[[Windows Server DNS 設定指南]]
[[Windows Server NTP 網路時間協定設定指南]]
[[Windows Server IPAM]]
[[網路位置與 Windows Defender 防火牆設定指南]] 
[[Windows Server DHCP 設定指南]]


### 安全性與認證
[[Windows Server Local security Policy]]
[[Windows Server DirectAccess VPN 設定指南]]
[[Windows AppLocker]]
[[Windows auditing]]

### VPN 與隧道技術
[[Windows Server IPSec 設定指南]]
[[Windows Server PPTP & GRE]]
[[Windows Server PPTP tunnel]]
[[Windows Server SSTP]]

### 檔案與儲存
[[Windows 檔案權限管理指南]]
[[Windows Server DFS namespace and DFS Replication 設定指南]]
[[Windows server Disk Array RAID]]

### Web 服務
[[Windows IIS Web 服務器配置指南]]

### 系統管理
[[Windows PowerShell Create New AD User 腳本]]
[[GPO Basic + PowerShell 操作整理]]
[[Windows PowerShell 基本語法與實用範例整理]]
[[Windows Server Core 管理指南]]
[[Windows local User and Group]]
[[Windows 11 安裝需求繞過指南]]

### 整合技術

[[Linux OpenSSL With Windows Web 憑證設定指南]]
[[windows server AD with Linux bind9]]

---

## Linux 系統管理

### 基礎系統管理
[[Linux 基本觀念和指令]]
[[Ubuntu 重設 root 與使用者密碼]]
[[Linux networking 常用指令]]
[[Linux Network Namespace：netns、veth 與 Bridge]]
[[Linux Fedora 設定 IP 指南]]
[[Linux Ubuntu or Debian Set hostname and IP address]]
[[Linux Debian DLBD ISO]]

### 網路服務
[[Linux ISC DHCP Server 與 ISC DHCP Relay 設定指南]]
[[Linux DNS Bind9 相關設定]]
[[Linux NTP Server and Client 設定與原理說明]]
[[Linux FreeRADIUS 設定指南]]
[[Linux Mail Server：Postfix、Dovecot 與 Maildir 設定指南]]

### Web 服務與代理
[[Linux Apache2 Web Server 設定指南]]
[[Linux Nginx Web Server 設定指南]]

### 安全性
[[iptables 防火牆完整指南]]
[[Linux OpenSSH 設定指南]]
[[Linux OpenLDAP 設定指南]]
[[Linux OpenSSL 憑證管理完整指南]]

### **VPN 與隧道技術**
[[Linux GRE tunnel 設定指南]]
[[Windows Server L2TP IPSec 與 IKEv2 VPN 設定指南]]
[[Linux L2TP over IPSec 設定指南]]
[[Linux OpenVPN 完整設定與憑證管理指南]]
[[NetBird Self-hosted and Access Policy]]
[[Linux PPTP VPN 安裝與設定教學]]

### 檔案系統與儲存
[[Linux iSCSI 設定指南]]
[[Linux NIS 設定指南]]

### 檔案服務

### 套件管理
[[Linux Fedora RPM package install]]

---

## HPC 與作業排程

[[Slurm 叢集架構與實作筆記|Slurm 安裝與雙節點叢集實作]]

## Kubernetes

[[K8s 考前快速總覽：易混淆觀念與重要指令|K8s 考前快速總覽]]

[[K8s 各 Component 說明]]
[[K8s 的 Certificate]]
[[K8s RBAC 與 ServiceAccount]]
[[K8s ConfigMap、Secret 與 Sealed Secrets]]
[[K8s CRD、Custom Resource 與 Controller]]
[[K8s 中的 Pod 是什麼]]
[[K8s Job 與 CronJob：批次工作、排程與 YAML 層級]]
[[K8s kubelet 與 Pod 執行流程]]
[[K8s Container Runtime 與 dockershim 遷移]]
[[K8s CNI 比較：Cilium、Calico 與 Flannel]]
[[K8s NetworkPolicy 網路存取控制]]
[[K8s Calico 安裝與網路判讀小抄]]
[[K8s CoreDNS、kubelet 與 resolv.conf]]
[[K8s Ingress 與 Gateway API]]
[[K8s Metrics Server、Prometheus 與 Heapster]]
[[K8s HPA 與水平、垂直擴縮]]
[[K8s Cilium 與 Containerlab 網路實驗]]
[[K8s CSI、PV、PVC 與雲端儲存]]
[[K8s kubectl 常用指令]]
[[K8s kubeconfig、Context 與多叢集管理]]
[[K8s kubectl 常用指令#Rollout 與回滾：不要只看 roll|Rollout 與回滾]]
[[K8s kubelet 與 Pod 執行流程#ImagePolicyWebhook：映像政策驗證小抄|ImagePolicyWebhook 映像政策驗證]]
[[K8s 排程：Scheduler、PriorityClass、Affinity、Taint 與 Toleration]]
[[K8s kubeadm 安裝：containerd、init 與 join]]
[[K8s kubeadm 叢集升級：Control Plane 與 Worker]]
[[K8s Helm：Chart、Template、Values 與 Release]]
[[K8s Kustomize：Base 與 Overlay]]
[[MicroK8s 高可用安裝與設定]]
[[K8s HA、Quorum 與 Split-brain]]

## 容器與系統部署

[[Docker ENTRYPOINT、CMD 與 K8s command、args]]

[[Colima + containerd 指令小抄]]
[[Oracle Database 19c 在 Oracle Linux 安裝指南]]

## 虛擬化平台

[[Windows Hyper-V]]
[[WSL2 安裝指南]]

---

## 基礎設施即代碼

### Terraform
[[Terraform (IaC) 和 Hyper-V指南]]

---

## 網路模擬器

### 模擬環境
[[EVE-NG 安裝與設定指南]]
[[GNS3 安裝與設定指南]]
[[Containerlab 安裝與設定指南]]
[[FortiGate 與 Cisco IOL：UTM OSPF Lab]]

---

## Other

[[Ansible 基本操作安裝]]
[[MinIO S3 硬碟吞吐量基準測試報告]]

## 開發與部署筆記

[[git-worktree|Git Worktree]]
[[django-uwsgi-nginx|Django、uWSGI 與 Nginx]]
[[domjudge-install-guide|DOMjudge 安裝]]
[[rest-with-api|RESTful 與 API]]
[[neovim-settings|LazyVim 設定]]
[[introduce-vim|Vim 快捷鍵]]
[[vscode-extension-profile|VS Code Profile]]
[[setting-hoex|Hexo 建站]]
[[setting-hexo-ga|Hexo 與 Google Analytics]]
[[setup-openssh-server-in-windows-with-guacamole-sftp|Windows OpenSSH 與 Guacamole SFTP]]

## Python

[[Python 專案初始化]]
[[Python 正規表達式 Regex]]
[[使用 Nuitka 與 Docker 打造高效能且安全的 Python 應用部署方案]]

### 開發習慣

[[style-guide-for-python-code|Python 程式碼風格]]

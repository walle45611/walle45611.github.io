---
title: "Kubernetes 設計、部署與 GitOps"
tags:
  - concept/cloud
aliases:
  - "microk8s-production-readiness"
  - "kubernetes-gitops-delivery"
---

# Kubernetes 設計、部署與 GitOps

Kubernetes 透過 API 保存期望狀態，由控制器及節點元件持續收斂。設計理由、叢集運維與應用交付是同一主題的不同層次，部署一次成功不表示已具長期可靠性。

## 宣告式設計

Scheduler、kubelet 等元件讀取共同 API 物件協作；level-triggered 指依目前狀態決定行動，不等於沒有事件。恢復後讀取狀態也不表示控制面中斷時仍有完整服務能力。

應用可由環境變數與檔案接收設定；PVC 表達需求、PV 表達資源、StorageClass 承接佈建選擇。可攜性以目標後端滿足需求為前提。CRD 與 controller 擴充 API，宣告新物件不會自動實作行為。

## 部署與運維

MicroK8s 來源涵蓋單機基線、DNS、storage、Ingress、MetalLB、metrics、多節點與角色／taint。是否適合生產取決於 HA、網路、權限、恢復及升級流程；來源對小中型叢集的描述不作通用容量上限。較大規模或多雲方案仍需比較實際需求。

Pod namespace、CNI、Service backend mapping 與 DNS 分開排錯；元件健康不證明 NetworkPolicy、入口流量及實際封包路徑已驗證。見 [[container-networking]]。

## GitOps 交付鏈

Helm 套件化、Argo CD 同步、Image Updater 推進映像、Sealed Secrets 管理可進版本控制的密文，分工須配合實際版本與憑證。固定 helm 操作語境，先叢集基線再交付工具，核對管理入口、registry 認證、namespace 與 secret 範圍。

密文入 Git 不取代金鑰備份與權限治理。NodePort／proxy 是來源的操作路徑，不是所有環境唯一正確入口；Image Updater 多 registry／多環境與外部 secret 系統的整合仍待更多案例。

## 限制與來源邊界

設計原則主要依 2018 年演講，網路入門依 2024 年示範，部署指南保存特定版本流程；它們不互相證明工具設定已實測。HA 恢復時間、Snap 通道及升級相容性需更多量化資料；映像打包見 [[nuitka-and-docker-deployment]]。

## 來源

- [[container-networking-from-scratch]]
- [[kubernetes-design-principles-understand-the-why]]
- [[microk8s-production-deployment-guide]]
- [[understanding-kubernetes-networking-in-30-minutes]]

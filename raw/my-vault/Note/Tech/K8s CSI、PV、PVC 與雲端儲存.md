# K8s CSI、PV、PVC 與雲端儲存

## 元件與宣告的分工

| 元件／資源                    | 角色                                 |
| ------------------------ | ---------------------------------- |
| Storage backend          | 真正提供儲存，例如 Persistent Disk、NFS、Ceph |
| CSI driver               | 依 CSI 介面整合儲存後端                     |
| StorageClass             | 定義 provisioner 與配置參數，本身不是磁碟        |
| PV                       | 叢集範圍的儲存資源描述                        |
| PVC                      | Namespace 內的儲存需求，綁定相符的 PV          |
| Pod volumes／volumeMounts | 指定使用哪個 PVC，以及容器內掛載路徑               |

動態配置的準備順序是「安裝／啟用 driver → 建立 StorageClass → 建立 PVC → Pod 使用 PVC」。實際 provisioning／binding 時機取決於 binding mode；`WaitForFirstConsumer` 會等待使用 PVC 的 Pod，配合排程與拓撲再配置／綁定，並非永遠先建立 PV 才開始排程。也可以預先建立 PV 做靜態配置。

參考：[Persistent Volumes](https://kubernetes.io/docs/concepts/storage/persistent-volumes/)。

## attach 與 mount 的差別

```text
雲端 block storage
    ↓ Controller 端經 cloud API attach
Node VM 看見 block device
    ↓ kubelet 協調 CSI Node plugin stage／publish
Node 上準備好 volume
    ↓ runtime 將掛載提供給容器
Container /data
```

CSI 的 controller-side attach 常由 external-attacher 與 driver 協作；不是 kubelet 直接負責整套雲端 attach 流程。Node side 執行 stage／publish，實際步驟依 driver 能力與 volume mode 而定。Pod 只宣告需求，不是 Pod 內的程式自行執行 mount。

NFS 通常沒有把 block device attach 到 VM 的步驟，而是透過網路掛載遠端檔案系統。

`ReadWriteOnce` 表示單一 Node 讀寫，不代表只能有一個 Pod；同一 Node 上可能有多個 Pod 使用。若需限制單一 Pod，要另看 `ReadWriteOncePod` 與 driver 支援。

## GKE 與本機叢集使用雲端儲存

| 情境 | 使用方式 |
|---|---|
| GKE + Compute Engine Persistent Disk | 啟用／使用 GKE 管理的 PD CSI driver，provisioner 為 `pd.csi.storage.gke.io` |
| 本機 Node + 雲端 NFS／Filestore | 建立私有網路連通及存取規則，再由 NFS CSI 掛載 |
| 本機 Node + Cloud Storage | 使用 SDK／API 存取 object storage；FUSE 與 POSIX 語意需另外評估 |

Compute Engine Persistent Disk 的 VM attach 模型不能直接套到本機 Mac／OrbStack VM。Helm 只是安裝 driver 的工具，不是執行 mount 的元件；GKE 的管理式 driver 與自行安裝 CSI 是不同管理途徑。參考：[GKE PD CSI driver](https://cloud.google.com/kubernetes-engine/docs/how-to/persistent-volumes/gce-pd-csi-driver)。

## 本機 Kubernetes 接遠端 NFS 範例

前提：已安裝 `nfs.csi.k8s.io` driver，所有可能使用 volume 的 Node 以及 provisioning controller 都能存取 NFS server。下列 IP 與 share 為示例，未在實際叢集執行；NFS 版本必須符合伺服器設定。

```yaml
apiVersion: storage.k8s.io/v1
kind: StorageClass
metadata:
  name: remote-nfs
provisioner: nfs.csi.k8s.io
parameters:
  server: 10.10.0.5
  share: /vol1
reclaimPolicy: Retain
volumeBindingMode: Immediate
---
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: cloud-data
spec:
  storageClassName: remote-nfs
  accessModes:
    - ReadWriteMany
  resources:
    requests:
      storage: 10Gi
---
apiVersion: v1
kind: Pod
metadata:
  name: storage-test
spec:
  containers:
    - name: app
      image: nginx:stable
      volumeMounts:
        - name: data
          mountPath: /data
  volumes:
    - name: data
      persistentVolumeClaim:
        claimName: cloud-data
```

此 driver 的動態配置通常是在既有 NFS share 建立子目錄，不是自動建立一台 Filestore，也不保證 `10Gi` 會成為伺服器端強制 quota。參考：[NFS CSI driver parameters](https://github.com/kubernetes-csi/csi-driver-nfs/blob/master/docs/driver-parameters.md)。

```bash
kubectl get csidriver
kubectl get storageclass
kubectl get pvc
kubectl get pv
kubectl describe pvc cloud-data
kubectl describe pod storage-test
```

PVC Pending 先查 provisioner 與 binding；Pod 卡在 mount 則查 driver、路由、防火牆、NFS export、協定版本及權限。TCP 2049 可達只是一項檢查，不能取代實際 mount 與讀寫驗證。

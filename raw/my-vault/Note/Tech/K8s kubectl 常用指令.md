# K8s kubectl 常用指令

以現場能記住、直接打出來為原則：優先使用內建縮寫，每個情境先記一個主要寫法。

## 先設定 k 與常用縮寫

```bash
alias k=kubectl
```

以下用 `k` 代表 `kubectl`。alias 只對目前 shell 生效；換到新環境時先設定，或直接改打 `kubectl`。

| 寫法 | 意義 |
|---|---|
| `po` | pods |
| `deploy` | deployments |
| `svc` | services |
| `ns` | namespaces |
| `pc` | priorityclasses |
| `cm` | configmaps |
| `sa` | serviceaccounts |
| `-n lab` | 指定 namespace 為 lab |
| `-A` | 所有 namespaces |
| `-o yaml` | 輸出 YAML |
| `-o wide` | 顯示 IP、Node 等額外資訊 |

資源縮寫不一定能套到每個子命令；例如建立 Service 使用 `k create service`。

## explain：忘記 YAML 欄位時查什麼

```bash
# 查看 Pod 的欄位說明
k explain po

# 展開欄位結構：-R 是大寫，等同 --recursive
k explain po -R

# 逐層查，通常比展開整棵結構更容易讀
k explain po.spec
k explain po.spec.containers
k explain po.spec.containers.resources

# 只展開 containers 底下的結構
k explain po.spec.containers -R
```

`explain` 查資源的欄位定義；`describe` 查某個現有資源的狀態與事件。`explain` 需要從 API Server 取得 OpenAPI schema。

參數忘記時先查 `k explain -h`。本機 kubectl v1.37.1 支援 `-R`；其他環境若沒有這個短參數，就用 `--recursive`。

## 產生 YAML：先產生、再修改、再套用

最值得練熟的組合是 `--dry-run=client -o yaml`：產生物件定義，不建立資源；加上 `> 檔名.yaml` 存成檔案。

```bash
k run nginx --image=nginx --dry-run=client -o yaml > pod.yaml
# 編輯 pod.yaml 後套用
k apply -f pod.yaml
```

client dry-run 不代表伺服器一定接受設定，也不保證 Pod 能正常啟動；部分命令仍可能查詢 API Server。`expose` 指定既有 Pod／Deployment 時，即使 dry-run 也要先讀取來源資源。

## Pod

```bash
# 直接建立單一 Pod
k run nginx --image=nginx

# 只在畫面輸出 Pod YAML
k run nginx --image=nginx --dry-run=client -o yaml
```

`run` 建立的是 Pod。需要副本管理與更新時，用 Deployment；Pod 概念見 [[K8s 中的 Pod 是什麼]]。

## Deployment

```bash
# 建立 Deployment
k create deploy nginx --image=nginx

# 建立時指定 4 個副本：-r 是 --replicas 的縮寫
k create deploy nginx --image=nginx -r 4

# 產生有 4 個副本的 YAML，修改後再 apply
k create deploy nginx --image=nginx -r 4 --dry-run=client -o yaml > deploy.yaml
k apply -f deploy.yaml

# 調整既有 Deployment：scale 沒有 -r，要寫 --replicas
k scale deploy nginx --replicas=4
```

以上是不同操作情境，依需求選用；不要對同名的既有 Deployment 重複執行 `create`。

## Rollout 與回滾：不要只看 roll

> [!tip] 先記操作順序
> **更新 image → 看 status → 失敗查 describe／logs → history 找版本 → undo → 再看 status。** Rollout 是部署版本的過程；RollingUpdate 是逐批替換策略；Rollback 是回滾。

### RollingUpdate 與 Recreate

| 策略 | 行為 | 注意 |
|---|---|---|
| `RollingUpdate`（Deployment 預設） | 逐批建立新 Pod、減少舊 Pod | 更新中可能同時存在新舊版本 |
| `Recreate` | 先停止舊版本的 Pod，再建立新版本 | 通常會有服務中斷空窗 |

```yaml
# Deployment 的 spec 片段
strategy:
  type: RollingUpdate
  rollingUpdate:
    maxUnavailable: 0
    maxSurge: 1
```

`maxUnavailable: 0` 表示更新時不允許可用副本數低於期望數；`maxSurge: 1` 表示最多多建 1 個 Pod。這組設定需要額外資源，新 Pod 必須 Ready 才能繼續替換。兩者不可同時為 0；百分比時 unavailable 向下取整、surge 向上取整。**不代表任何環境都保證零停機**，仍要有正確 readiness probe 與應用程式關閉處理。

### 更新版本：set image、edit、apply 三選一

以下用 Deployment `nginx`、container 名稱 `nginx` 示範；image 版本依題目替換，所有命令使用同一個 namespace（需要時加 `-n`）。

```bash
# 先查 container 名稱與原 image，名稱不一定與 Deployment 相同
k get deploy nginx -o jsonpath='{.spec.template.spec.containers[*].name}{"\n"}{.spec.template.spec.containers[*].image}{"\n"}'

# 最直接：格式為 container名稱=image
k set image deploy/nginx nginx=nginx:1.28.0
k rollout status deploy/nginx --timeout=120s
```

另外兩種是替代寫法，不必連續做：

```bash
# 現場修改：找到 spec.template.spec.containers[].image
k edit deploy nginx

# 已有受管理的 manifest：修改檔案中的 image 後套用
k apply -f deploy.yaml
```

只有 **`.spec.template` 改變**才會觸發 Deployment 的新 rollout／revision，例如 image、command、args 或 Pod labels。`scale` 改副本數、修改 Deployment 本身的 annotation，不會單獨產生新 revision。

### 看進度、找失敗原因

```bash
k rollout status deploy/nginx --timeout=120s
k get deploy nginx
k get rs
k get po -o wide
k describe deploy nginx
# 替換成 get po 查到的新版本 Pod 名稱
k describe po <新Pod名稱>
k logs <新Pod名稱>
```

`get rs` 可看到新舊 ReplicaSet 的副本變化；`describe po` 看 Events，區分拉不到映像、排程失敗或 readiness 不通過。`status` timeout／`ProgressDeadlineExceeded` 只表示更新未完成，**不會自動回滾**。

### History 與 Undo：真正的回滾

```bash
# 看有哪些 revision，並檢視指定版本的 Pod template
k rollout history deploy/nginx
k rollout history deploy/nginx --revision=2

# 回到上一個保留的 revision
k rollout undo deploy/nginx
k rollout status deploy/nginx --timeout=120s

# 或明確回到指定 revision；先用 history 確認它仍存在
k rollout undo deploy/nginx --to-revision=2
k rollout status deploy/nginx --timeout=120s
```

上面兩種 undo 是不同選擇。Rollback 把舊 Pod template 重新部署，會形成新的 revision；revision 編號不是 image tag。Deployment 的 `spec.revisionHistoryLimit` 控制保留供回滾的舊 ReplicaSet 數量，預設 10；清掉的版本無法直接 undo 回去。

`undo` 不會還原資料庫、外部系統或獨立 ConfigMap／Secret 的歷史內容，也不會把副本數恢復成舊值。若用 Git／manifest 管理版本，回滾後也要同步修改來源，否則下次 apply 可能又改回出錯版本。

### Pause、Resume、Restart

```bash
# 暫停 Deployment 的更新調和，方便一次累積多項 template 變更
k rollout pause deploy/nginx
k set image deploy/nginx nginx=nginx:1.28.0
# 完成其他所需修改後，恢復並部署
k rollout resume deploy/nginx
k rollout status deploy/nginx --timeout=120s

# 另一個情境：沿用目前 image／設定，重新建立 Pod
k rollout restart deploy/nginx
k rollout status deploy/nginx --timeout=120s
```

Pause 不會停止已執行的容器，也不會撤回已完成的更新；**Paused 的 Deployment 要先 resume 才能 rollback**。Restart 會改 Pod template 的重啟時間 annotation，觸發替換；不是恢復上一版，也不是原地重啟同一個 Pod。

若希望 history 顯示更新原因，可在更新前或更新時設定 `kubernetes.io/change-cause` annotation；不必背已棄用的 `--record`：

```bash
k annotate deploy/nginx kubernetes.io/change-cause='update nginx to 1.28.0' --overwrite
```

這個 annotation 用來記錄說明，不會單獨觸發 rollout。`rollout` 也支援 DaemonSet／StatefulSet 的部分操作，但本節以 Deployment 為準，pause／resume 等能力不能一概套用。

參考：[Deployment 更新策略、歷史與回滾](https://kubernetes.io/docs/concepts/workloads/controllers/deployment/)、[kubectl rollout](https://kubernetes.io/docs/reference/kubectl/generated/kubectl_rollout/)。

## Service

### 五種常見用法：四種 type ＋ Headless

| 用法 | 設定 | DNS／流量行為 | 適合情境 |
|---|---|---|---|
| ClusterIP | `type: ClusterIP`，預設 | 穩定的叢集內虛擬 IP，轉送至後端 | 叢集內服務互連 |
| NodePort | `type: NodePort` | 透過 `NodeIP:nodePort` 存取 | 從叢集外使用 Node 的連接埠 |
| LoadBalancer | `type: LoadBalancer` | 交給 LB 實作提供外部入口 | 對外發布服務 |
| ExternalName | `type: ExternalName` ＋ `externalName` | DNS 回 CNAME，指向另一個 DNS 名稱 | 用叢集內名稱代表外部服務 |
| Headless | `type: ClusterIP` ＋ `clusterIP: None` | 不配置虛擬 IP；有 selector 時 DNS 可直接回後端 Pod 位址 | StatefulSet、用戶端自行選擇後端 |

**官方 `spec.type` 只有前四種**；Headless 是特殊設定，不存在 `type: Headless`。Ingress／Gateway 是另外的資源，也不是第五種 Service type。

先記三種 port：`port` 是 Service 的入口，`targetPort` 是後端 Pod 的目的埠，`nodePort` 是 Node 對外提供的入口，預設配置範圍通常是 30000–32767。一般選擇 Pod 的 Service，`selector` 必須對得上 Pod labels；ExternalName 不靠 selector 選後端。

### ClusterIP：優先 expose 既有資源

```bash
# 前提：redis Pod 已存在；沿用它的 labels 選擇後端
k expose po redis --port=6379 --name=redis-service

# 若只要產生 YAML，加上 dry-run 與輸出參數
k expose po redis --port=6379 --name=redis-service --dry-run=client -o yaml > svc.yaml
```

預設為 ClusterIP；未指定 `--target-port` 時，這個例子的後端目的埠也是 6379。

### NodePort：產生 YAML，再指定 nodePort

```bash
# 前提：nginx Pod 已存在
k expose po nginx --type=NodePort --port=80 --name=nginx-service --dry-run=client -o yaml > svc.yaml
```

在產生的 YAML 中，於 `spec.ports` 的對應項目加上 `nodePort: 30080`：

```yaml
# svc.yaml 的 spec.ports 片段
ports:
  - port: 80
    targetPort: 80
    nodePort: 30080
```

修改後執行 `k apply -f svc.yaml`。`expose` 沒有直接指定 `--node-port` 的參數，這個做法能保留從來源推導的 selector。

### create service：另一種產生模板的方法

```bash
k create service clusterip redis --tcp=6379:6379 --dry-run=client -o yaml
k create service nodeport nginx --tcp=80:80 --node-port=30080 --dry-run=client -o yaml
```

`--tcp` 的順序是 `port:targetPort`。這兩條只輸出 YAML，沒有建立資源；產生的 Service 名稱分別是 redis 與 nginx。

`create service` 不會讀取 Pod labels，預設產生 `app=<Service 名稱>` 的 selector，也沒有直接指定 selector 的參數。相對地，`k run nginx --image=nginx` 預設產生的是 `run=nginx` label；兩者對不上，必須先改 YAML 的 `spec.selector`。因此平常優先記 `expose`。

### LoadBalancer：雲端與 MetalLB

```bash
# 前提：nginx Deployment 已存在
k expose deploy nginx --name=nginx-lb --type=LoadBalancer --port=80 --target-port=80
k get svc nginx-lb -w
```

看 `EXTERNAL-IP` 欄位，等 LB 控制器提供 IP 或主機名稱後，再依實際網路測試。`<pending>` 不一定是 Pod 壞了，可能是沒有 LB 實作、地址池不足，或供應商配置失敗；先查 `k describe svc nginx-lb`。

| 元件 | 負責的工作 |
|---|---|
| CNI，如 Calico／Cilium | Pod 網路與相關政策能力 |
| 雲端 LB 整合控制器 | 依 LoadBalancer Service 向雲端建立／設定負載平衡入口 |
| MetalLB | 在裸機／自架環境為 LoadBalancer Service 配發 IP，透過 L2 或 BGP 宣告可達性 |
| kube-proxy 或 Service 替代實作 | 實際將 Service 流量導向後端 Pod |

**自架叢集缺少的通常是 LoadBalancer 整合，不是「沒有雲端 CNI」**。已安裝 Calico 仍可能需要 MetalLB；若環境已有其他 LB 實作或 CNI 自帶並啟用相應功能，就不一定需要 MetalLB。

MetalLB 安裝後還需設定可用且不衝突的 `IPAddressPool`，並依模式配置 `L2Advertisement` 或 BGP peers／advertisement；只裝 Pod 不代表外部 IP 已能存取。L2 模式透過 ARP／NDP 宣告位址，BGP 模式需要路由器配合。MetalLB 不負責取代 Pod 的 CNI 網路。

LoadBalancer 通常會同時配置 ClusterIP 和 NodePort，但可依實作停用 NodePort 配置；不能死背所有 LB 都必須經過 NodePort。

### ExternalName：DNS 別名，不是代理

```bash
k create service externalname external-db --external-name=db.example.com
k get svc external-db
```

以 default namespace、cluster.local 為例，查 `external-db.default.svc.cluster.local` 會取得指向 `db.example.com` 的 CNAME，再由 DNS 解析目的位址。它沒有一般的 ClusterIP／Pod selector 轉送，也不會替你建立連線或改寫 HTTP Host／TLS 名稱。

`externalName` 填 **DNS 名稱**，不要把 IP 當成 DNS 別名填入；HTTP Host 或 HTTPS 憑證若不符合客戶端使用的名稱，仍可能失敗。

### Headless：直接找到後端

```bash
# 前提：nginx Deployment 已存在；沿用它的 selector
k expose deploy nginx --name=nginx-headless --port=80 --target-port=80 --cluster-ip=None
k get svc nginx-headless
```

`CLUSTER-IP` 顯示 `None` 是預期結果。一般帶 selector 的 Headless Service，DNS 回覆可用後端的位址，由客戶端決定連哪個，不透過 Service VIP 負載平衡。StatefulSet 常用它提供穩定的 Pod DNS 名稱，但 **Headless 本身不會建立 StatefulSet 或保證 Pod 永遠不換 IP**。

### Service 有建立，但連不到：先查什麼

```bash
k get svc
k describe svc nginx-service
k get endpointslices -l kubernetes.io/service-name=nginx-service
k get po --show-labels
```

`nginx-service` 換成實際名稱。沒有後端先看 selector／labels 與 Pod readiness；有後端再查 `targetPort`、應用程式是否監聽、NetworkPolicy 與網路路徑。ExternalName 的設計不同，不能要求它一定有 Pod endpoints。

DNS 與 Headless 的解析關係見 [[K8s CoreDNS、kubelet 與 resolv.conf]]；Pod 網路見 [[K8s CNI 比較：Cilium、Calico 與 Flannel]]。

參考：[Service 類型與 Headless](https://kubernetes.io/docs/concepts/services-networking/service/)、[MetalLB 的用途](https://metallb.io/)、[MetalLB 設定](https://metallb.io/configuration/)。

## ConfigMap：-f 和 --from-file 不一樣

```bash
# 檔案本身是 kind: ConfigMap 的完整資源定義
k create -f /root/my-scheduler-config.yaml -n kube-system

# 檔案是 KubeSchedulerConfiguration 等設定內容，把內容裝進 cm
k create cm my-scheduler-config --from-file=/root/my-scheduler-config.yaml -n kube-system
```

這兩條是不同情境，不要連續執行建立同名物件。`k create cm` 沒有 `-f`；`-f` 讓 create 讀取資源定義，`--from-file` 則把檔案內容存進 ConfigMap 的 data，預設 key 是檔名，須對上 Scheduler 掛載後的設定檔路徑。

```bash
k get cm my-scheduler-config -n kube-system -o yaml
k get sa -n kube-system
k get clusterrolebinding
k api-resources
```

ServiceAccount 用 `sa`；ClusterRoleBinding、ClusterRole、RoleBinding、Role 沒有內建資源短名，不能自行假定 `crb` 可用。忘記資源縮寫就查 api-resources 的 SHORTNAMES 欄；忘記子命令參數則用 `k create cm -h`、`k create pc -h`。

## 檢查與排錯

```bash
k get po
k get po -o wide
k get po -A
k get deploy,svc

# nginx 是範例 Pod 名稱，換成 get po 查到的名稱
k describe po nginx
k logs nginx
k logs nginx -f

# Deployment 的更新進度與重新啟動
k rollout status deploy nginx
k rollout restart deploy nginx
```

`describe` 先看 Events；`logs -f` 持續追蹤輸出，用 Ctrl-C 結束。Pod 執行流程見 [[K8s kubelet 與 Pod 執行流程]]。

### Argo CD Image Updater：原有操作

```bash
k rollout restart deploy argocd-image-updater -n argocd
k logs deploy/argocd-image-updater -n argocd
```

原本使用的 Pod 名稱是 `argocd-image-updater-6f6696b6c6-7224j`；重建後尾碼會變，因此平常直接指定 Deployment 查日誌。需要定位特定副本時，先用 `k get po -n argocd` 查當下名稱，再用 `k logs <Pod 名稱> -n argocd`。

## 排程：Scheduler、PriorityClass 與節點條件

PriorityClass 建立與指定、搶占、`priority: 0` 排錯，以及 Node Selector、Affinity、Taint／Toleration 的觀念和操作統一見 [[K8s 排程：Scheduler、PriorityClass、Affinity、Taint 與 Toleration]]。

## 資源配額與 QoS

ResourceQuota、LimitRange、requests／limits 與 QoS 比較，以及查詢指令見 [[K8s 中的 Pod 是什麼#Pod 的資源管理和限制資源]]。

## 參考

- [kubectl explain](https://kubernetes.io/docs/reference/kubectl/generated/kubectl_explain/)
- [kubectl run](https://kubernetes.io/docs/reference/kubectl/generated/kubectl_run/)
- [kubectl create deployment](https://kubernetes.io/docs/reference/kubectl/generated/kubectl_create/kubectl_create_deployment/)
- [kubectl expose](https://kubernetes.io/docs/reference/kubectl/generated/kubectl_expose/)
- [kubectl create service nodeport](https://kubernetes.io/docs/reference/kubectl/generated/kubectl_create/kubectl_create_service_nodeport/)
- [kubectl 指令參考](https://kubernetes.io/docs/reference/generated/kubectl/kubectl-commands)
- [kubectl 使用慣例](https://kubernetes.io/docs/reference/kubectl/conventions/)

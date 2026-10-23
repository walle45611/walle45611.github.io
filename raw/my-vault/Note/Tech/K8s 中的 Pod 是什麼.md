---
blog: true
blog_title: K8s 中的 Pod 是什麼
blog_date: 2024-11-28
blog_url: https://blog.walle4561.com/articles/posts/k8s-introduction-pods/
---

## Pod 是什麼

Pod 是可以在 K8s 創建和管理的最小單位。  
一個 Pod 是一個群組，裡面可以有一個或是多個 containers, 共享 storage 和 resource 和怎麼運行這個 Pod 的相關 specification (規格，設定)，Pod 內的容器在同一個節點被調度和運行，，形成一個應用程序的「邏輯主機」，類似於在非雲端環境下同一實體機或虛擬機中的應用。  
  

- Pod 可以包含以下幾種容器：  
    1. Application pod：執行主要的應用程序邏輯。  
    2. init containers：在 Pod 啟動過程中執行。  
    3. Ephemeral Containers：用排查問題。  
    

---

## Pod theory

The atomic unit of scheduling on Kubernetes is the Pod. This is just a fancy way of saying apps deployed to  
Kubernetes always run inside Pods.  
Some quick examples… If you deploy an app, you deploy it in a Pod. If you terminate an app, you terminate its  
Pod. If you scale an app up or down, you add or remove Pods.  
  
在 K8s 中，排程的最小單位是 Pod。意指所有部署到 K8s 中的 Application，都必須在 Pod 中運行。  
  
1. 當需要部署一個 Application 時，實際上是將其部署到 Pod 中。  
2. 當終止一個 Application 時，實際上也是中指他所在的 Pod。  
3. 當你調整應用程式的規模時，無論是擴展還是縮減，都是透過新增或移除 Pod 來實現。  

---

## Pod lifecycle

### Pod phase

首先 Pod 會有自己的生命週期，Pod 會遵循這些狀態創建、刪除、執行等等...  
  

- 相關重點
    - Binding：如果一個 Pod 被分配到特定的節點，我們稱之為綁定 (binding)。
    - Scheduling：選擇使用哪個節點的過程稱為調度 (scheduling)。
    - 調度限制：Pod 的調度 (scheduling) 只會進行一次，並且該 Pod 將持續運行於此節點，直到被停止或終止 (terminated)。
- Pod 的主要狀態
    - Peding : Pod 已經創但是尚未啟動，此街段包括 Pod 被 scheduling 或是下載 Image。
    - Running : Pod 正在運行，至少有一個 container 在執行中，或著處於啟動或是重新啟動狀態。
    - Succeded : 所有的 containers 都成功 terminated，且不會再次重新啟動。
    - Failed : Pod 中的所有 containers 都已經 terminated，並且至少有一個 container 失敗 * terminated，且為設定自動重新啟動。
    - Unknown : 無法獲取 Pod 的狀態。
- 常見狀態說明
    - CrazhLoopBackOff：如果 Pod 反覆重啟失敗，kubectl 的 Status 欄位會顯示此狀態，表示容器正在嘗試啟動但持續失敗。
    - Terminating：當刪除 Pod 時，Status 欄位可能顯示 Terminating，表示 Pod 正在關閉中。
- Pod 終止時間
    - 當 Pod 被終止時，會有一個默認為 30 秒的優雅關閉時間，用於清理資源並完成關閉過程。
    - 如果需要強制終止 Pod，可以使用 --force 標誌來繞過優雅關閉時間。

### Pod status

- Waiting：container 正在完成他啟動時需要的動作，例如：從某個 registry 取得 image，或者 container 正在取得 Secret 資料等等
- Running：這個狀態表示 container 執行中並沒有發生什麼問題。
- Terminated：這個狀態表示 container 在執行後，因為某些原因導致失敗所產生的狀態，或是正常關閉的狀態。

---

## **從程式碼到 Pod**

1. 寫程式碼
2. 打包你的 container image
3. 打吧你的 container image 到 Pod
4. 把 Pod 跑在 K8s 上

- 那為什麼不直接跑 container 在 K8s 上，以下有幾個比較短的理由
    - Pods 增強了 containers
        - 可以在 Pods 上加上 Labels 和 annotations
        - Pods 重啟的一些政策
        - Probes(探針)，startup probes、readiness prbes、liveness probes，和更多。
        - Affinity 和 anti-affinity （親和性和反親和信）
    - Pods 可以被調度 (scheduling)
    - Pods 可以開啟資源共享

---

## Pod 的資源管理和限制資源

### ResourceQuota 與 LimitRange：總量和個別範圍

| 機制 | 作用範圍 | 用途 |
|---|---|---|
| ResourceQuota | 整個 Namespace，亦可設定 scope 篩選適用物件 | 限制資源配額總量，例如 requests／limits 加總、PVC 儲存請求或物件數量 |
| LimitRange | Namespace 內的個別 Container、Pod、PVC，依資源類型支援的欄位而定 | 限制最小值、最大值或 limit/request 比例；可為 Container 提供預設 requests／limits |

ResourceQuota 的 CPU／Memory 配額計算的是宣告的 requests／limits，不是即時使用量；配額也不代表叢集已替這個 Namespace 預留實體容量。LimitRange 的 `default` 是預設 limit，`defaultRequest` 是預設 request。

兩者主要在 API admission 時檢查或補值：新增／更新超出配額或範圍的資源會被拒絕。修改 LimitRange 不會回頭重寫已執行 Pod 的設定。

來源：[Resource Quotas](https://kubernetes.io/docs/concepts/policy/resource-quotas/)、[Limit Ranges](https://kubernetes.io/docs/concepts/policy/limit-range/)。

### Requests 與 Limits：排程需求和執行上限

| 設定 | 意義 | 超過數值時 |
|---|---|---|
| `requests` | 資源需求與排程容量判斷依據；CPU request 也影響競爭時的分配權重 | 資源可用且未受 limit 等限制時，可以使用更多；超過 request 本身不會直接觸發 OOM kill |
| `limits.cpu` | CPU 使用上限 | 受 CPU throttling 限制，不會單純因為碰到 CPU limit 而被終止 |
| `limits.memory` | Memory 使用上限 | 可能觸發 OOM kill，容器狀態可能顯示 `OOMKilled` |

`requests.memory` 不是「啟動時必須實際占用的最小記憶體」，也不是使用上限；Scheduler 主要依 requests 與 Node 可分配容量判斷能否排入，而不是看當下空閒量。排程其他限制見 [[K8s 排程：Scheduler、PriorityClass、Affinity、Taint 與 Toleration]]。

若只設定某項資源的 limit，沒有 request，且 admission 沒有提供該項預設 request，Kubernetes 會把 limit 複製為 request。因此要看建立後的實際物件，而不能只看手寫的 YAML。

來源：[Resource Management for Pods and Containers](https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/)。

### QoS 三種等級

以下先以一般在 Container 層設定 CPU／Memory 的 Pod 為例，依 admission 補值後的實際設定判斷：

| QoS | Requests／Limits 條件 | 記憶體壓力下的驅逐傾向 |
|---|---|---|
| Guaranteed | 每個 Container 的 CPU、Memory request 與 limit 都有設定、皆大於 0，且同一資源 request = limit | 通常較不容易被驅逐，但不是絕對不會 |
| Burstable | 不符合 Guaranteed，且至少一個 Container 有 CPU 或 Memory request／limit | 取決於是否超過 request、Pod Priority 與相對使用量 |
| BestEffort | 所有 Container 都沒有 CPU、Memory requests／limits | 通常較容易被驅逐 |

若使用支援的 Pod-level resources（`spec.resources`），Pod 層 CPU／Memory 設定也參與 QoS 判斷：Pod 層兩項資源的 request 與 limit 相等可符合 Guaranteed；完全沒設定 CPU／Memory 才可能是 BestEffort。以上表格的逐 Container 規則不能直接套用到所有 Pod-level 設定情況。

> [!important] QoS 是判斷傾向，不是固定排序
> Node-pressure eviction 並非單純照 BestEffort → Burstable → Guaranteed 排序。kubelet 會考慮相關資源使用是否超過 requests、Pod Priority、使用量相對 requests 的程度。QoS 對記憶體壓力的傾向較有參考性，不能直接套用到 DiskPressure 等所有情境。

`OOMKilled` 和 `Evicted` 也要分開：前者是容器內的程序遭 OOM kill，可能依 restart policy 重啟容器；後者是 kubelet 因節點壓力等原因終止整個 Pod，若由 Deployment 管理，控制器會另外補建 Pod。

來源：[Pod Quality of Service Classes](https://kubernetes.io/docs/concepts/workloads/pods/pod-qos/)、[Node-pressure Eviction](https://kubernetes.io/docs/concepts/scheduling-eviction/node-pressure-eviction/)。

### 實作：產生模板，再設定 requests／limits

```bash
alias k=kubectl
k run nginx-pod --image=nginx:latest --dry-run=client -o yaml > nginx-pod.yaml
```

產生的模板會有 `resources: {}`。編輯為以下設定；這裡省略不必要的空白欄位：

```yaml
apiVersion: v1
kind: Pod
metadata:
  labels:
    run: nginx-pod
  name: nginx-pod
spec:
  containers:
    - image: nginx:latest
      name: nginx-pod
      resources:
        requests:
          memory: "64Mi"
          cpu: "250m"
        limits:
          memory: "256Mi"
          cpu: "1"
  dnsPolicy: ClusterFirst
  restartPolicy: Always
```

`64Mi` 是 64 Mebibytes 的記憶體 request；`250m` 是 0.25 CPU core 的 request，CPU limit `1` 則是 1 core。這個 Pod 的 request 與 limit 不相等，因此是 Burstable。

```bash
k apply -f nginx-pod.yaml

# lab 換成實際 namespace；quota、limits 分別是 ResourceQuota、LimitRange 的資源短名
k get quota,limits -n lab
k describe quota -n lab
k describe limits -n lab

# 查看補值後的 resources、QoS Class，以及 OOMKilled／Evicted 線索
k get po nginx-pod -o yaml
k describe po nginx-pod

# 忘記欄位時當場查
k explain po.spec.containers.resources
k explain quota.spec.hard
k explain limits.spec.limits
```

下方保留先前在 OrbStack 執行的歷史輸出，供對照；當下結果以實際查詢為準。更多常用操作見 [[K8s kubectl 常用指令]]。

- 查看該 Pod 的狀態
    
    ```Shell
    kubectl describe pod nginx-pod
    ```
    
    ```Plain
    Name:             nginx-pod
    Namespace:        default
    Priority:         0
    Service Account:  default
    Node:             orbstack/198.19.249.2
    Start Time:       Fri, 29 Nov 2024 13:57:14 +0800
    Labels:           run=nginx-pod
    Annotations:      <none>
    Status:           Running
    IP:               192.168.194.9
    IPs:
    IP:  192.168.194.9
    IP:  fd07:b51a:cc66:a::9
    Containers:
    nginx-pod:
        Container ID:   docker://53e5a5cb01fa37f074d782ca3299e2410f6e736133f03bce44a9e9d2dbd4ea72
        Image:          nginx:latest
        Image ID:       docker-pullable://nginx@sha256:0c86dddac19f2ce4fd716ac58c0fd87bf69bfd4edabfd6971fb885bafd12a00b
        Port:           <none>
        Host Port:      <none>
        State:          Running
        Started:      Fri, 29 Nov 2024 13:57:26 +0800
        Ready:          True
        Restart Count:  0
        Limits:
        cpu:     1
        memory:  256Mi
        Requests:
        cpu:        250m
        memory:     64Mi
        Environment:  <none>
        Mounts:
        /var/run/secrets/kubernetes.io/serviceaccount from kube-api-access-cjgr8 (ro)
    Conditions:
    Type                        Status
    PodReadyToStartContainers   True
    Initialized                 True
    Ready                       True
    ContainersReady             True
    PodScheduled                True
    Volumes:
    kube-api-access-cjgr8:
        Type:                    Projected (a volume that contains injected data from multiple sources)
        TokenExpirationSeconds:  3607
        ConfigMapName:           kube-root-ca.crt
        ConfigMapOptional:       <nil>
        DownwardAPI:             true
    QoS Class:                   Burstable
    Node-Selectors:              <none>
    Tolerations:                 node.kubernetes.io/not-ready:NoExecute op=Exists for 300s
                                node.kubernetes.io/unreachable:NoExecute op=Exists for 300s
    Events:
    Type    Reason     Age   From               Message
    ----    ------     ----  ----               -------
    Normal  Scheduled  18m   default-scheduler  Successfully assigned default/nginx-pod to orbstack
    Normal  Pulling    18m   kubelet            Pulling image "nginx:latest"
    Normal  Pulled     18m   kubelet            Successfully pulled image "nginx:latest" in 11.338s (11.338s including waiting)
    Normal  Created    18m   kubelet            Created container nginx-pod
    Normal  Started    18m   kubelet            Started container nginx-pod
    ```
    

---

## **Pod 相關指令**

- 需要查看 Pod 的相關 logs
    
    ```Shell
    kubectl logs <pod-name>
    ```
    
- 這個指令可以查尋 Pod 的相關屬性
    
    ```Shell
    kubectl explain pods --recursive | less
    ```
    
    ```Plain
    KIND:       Pod
    VERSION:    v1
    
    DESCRIPTION:
        Pod is a collection of containers that can run on a host. This resource is
        created by clients and scheduled onto hosts.
    
    FIELDS:
    apiVersion    <string>
    kind  <string>
    metadata      <ObjectMeta>
        annotations <map[string]string>
        creationTimestamp   <string>
        deletionGracePeriodSeconds  <integer>
        deletionTimestamp   <string>
        finalizers  <[]string>
        generateName        <string>
        generation  <integer>
        labels      <map[string]string>
        managedFields       <[]ManagedFieldsEntry>
    ...
    ```
    
- 預設會從 default namespace 取得 pods 相關資訊
    
    ```Shell
    kubectl get pods
    
    NAME    READY   STATUS    RESTARTS   AGE
    nginx   1/1     Running   0          8m7s
    ```
    
    - 如果要找到更多相關資訊
        
        ```Shell
        kubectl get pods -o wide
        NAME    READY   STATUS    RESTARTS   AGE   IP              NODE       NOMINATED NODE   READINESS GATES
        nginx   1/1     Running   0          12m   192.168.194.8   orbstack   <none>           <none>
        ```
        
    - 以下指令可以取得所有 namespace 底下的 pods
        
        ```Shell
        kubectl get pods --all-namespaces
        
        NAMESPACE     NAME                                      READY   STATUS    RESTARTS   AGE
        kube-system   local-path-provisioner-6c86858495-m5x7g   1/1     Running   0          148d
        kube-system   coredns-66cc6945cb-hswrk                  1/1     Running   0          148d
        default       nginx
        ```
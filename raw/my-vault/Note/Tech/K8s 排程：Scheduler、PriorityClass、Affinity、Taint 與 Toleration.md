---
blog: true
blog_title: 'K8s 排程：Scheduler、PriorityClass、Affinity、Taint 與 Toleration'
blog_date: '2026-10-29'
blog_url: https://blog.walle4561.com/articles/posts/k8s-scheduler-priority-affinity-taints/
blog_tag: OS
blog_topic: Kubernetes
---

# K8s 排程：Scheduler、PriorityClass、Affinity、Taint 與 Toleration

> [!tip] 排程題先判斷四件事
> **誰來排 → 哪些 Node 能放 → 合格 Node 中選哪台 → 放不下時是否能搶占。** PriorityClass 不會讓 Pod 跳過 Taint、Affinity 或資源限制；Toleration 也不會指定目的地。

| 問題 | 看哪個設定／機制 |
|---|---|
| 由誰處理這個 Pod？ | `schedulerName`、Scheduler profile |
| 哪個 Pod 優先被處理？ | PriorityClass、排程佇列 |
| 哪些 Node 能放？ | requests、nodeSelector、Required Affinity、Taint／Toleration 等 |
| 合格 Node 中偏好哪台？ | Scheduler Score、Preferred Affinity 等 |
| 移除低優先權 Pod 能否騰出合法位置？ | Preemption；不能排除節點本身的硬限制 |

閱讀順序：[Scheduler 流程](#scheduler選-pod篩-node評分與綁定) → [節點選擇與 Taint／Toleration](#先分清楚pod-想去哪裡node-允許誰進來) → [PriorityClass 與搶占、操作及排錯](#priorityclasspod-有多重要) → [多排程器](#multi-scheduler多個程序不一定是不同二進位檔)。

以下指令使用 `k` 代表 `kubectl`，先設定 `alias k=kubectl`。Pod 建立、Admission 與 Node 執行的全貌見 [[K8s kubelet 與 Pod 執行流程]]。

## Scheduler：選 Pod、篩 Node、評分與綁定

Scheduler 的核心是 **先選 Pod → 篩 Node → 評分選 Node → 綁定**。Plugin 是實作具體判斷的模組，掛在 Framework 提供的擴充點；同一個 Plugin 可以參與多個階段，也不必實作所有擴充點。

| 擴充點 | 在做什麼 | 例子／重點 |
|---|---|---|
| QueueSort | 決定等待中的 Pod 處理順序 | PrioritySort 依 Priority 等條件排序 |
| PreFilter | 整理篩選需要的共用資料，或先做整體檢查 | 計算 Pod 的資源需求 |
| Filter | 逐台檢查 Node 能不能放 | NodeResourcesFit 檢查 requests；NodeAffinity、TaintToleration 檢查節點條件 |
| PostFilter | 找不到可行 Node 時嘗試補救 | DefaultPreemption 評估搶占；不是每次都會搶占成功 |
| PreScore | 整理評分需要的共用資料 | 為候選 Node 的評分準備資訊 |
| Score | 替合格 Node 評分 | NodeResourcesFit 依資源策略評分；ImageLocality 偏好已有需要映像的 Node |
| Reserve | 在 Scheduler／Plugin 內暫記資源安排 | 尚未正式綁定，不等於在 Node 啟動容器 |
| Permit | 允許、拒絕或等待 | 可等待相關工作準備完成後再繼續 |
| PreBind | 做綁定前必要的準備 | 例如協調所需儲存資源 |
| Bind | 透過 API Server 完成 Pod 與 Node 的綁定 | DefaultBinder |
| PostBind | 綁定成功後通知與收尾 | 不再重新決定 Node |

Reserve 之後的流程失敗時，Unreserve 負責撤銷先前暫記的安排。Filter 和 Score 是「能不能放」與「放哪台比較好」的差別；PostFilter 則是沒有可行 Node 時的分支，不是正常篩選成功後一定接著執行的步驟。完整擴充點見 [Scheduling Framework](https://kubernetes.io/docs/concepts/scheduling-eviction/scheduling-framework/)，內建 Plugin 見 [Scheduler Configuration](https://kubernetes.io/docs/reference/scheduling/config/#scheduling-plugins)。

requests／limits 與 QoS 見 [[K8s 中的 Pod 是什麼#Pod 的資源管理和限制資源]]。

## 先分清楚：Pod 想去哪裡、Node 允許誰進來

| 機制 | 設定位置 | 用途 |
|---|---|---|
| Node Label | Node 的 `metadata.labels` | 節點的特徵，例如 `disk=ssd` |
| `nodeSelector` | Pod 的 `spec.nodeSelector` | 選擇具有指定 labels 的 Node，多個鍵值全部要符合 |
| Node Affinity | Pod 的 `spec.affinity.nodeAffinity` | 用更有彈性的條件選 Node，區分必要條件與偏好 |
| Taint | Node 的 `spec.taints` | 排斥不符合容忍條件的 Pod |
| Toleration | Pod 的 `spec.tolerations` | 容忍匹配的 Taint，解除該 Taint 帶來的限制 |

> [!important] Toleration 是通行資格，不是目的地
> 容忍某個 Node 的 Taint，不代表 Pod 一定會排到那台 Node，也不會略過 nodeSelector、Required Node Affinity、資源容量等其他限制。Node Label 和 Taint 是不同欄位；設定其中一項不會自動產生另一項。

Deployment 的 `spec.selector.matchLabels` 用來選 Pod；這裡的 `nodeSelector` 與 `nodeAffinity` 用來選 Node。若寫在 Deployment 中，Pod 的排程設定放在 `spec.template.spec` 下。

## 第一層：外層排程限制通常是 AND

對一般由 Kubernetes Scheduler 排程的新 Pod，可以把候選 Node 理解成：

$$
\begin{aligned}
\text{可排程候選 Node} ={}& \text{nodeSelector 符合} \\
&\land \text{Required Node Affinity 符合} \\
&\land \text{無未容忍的阻擋性 Taint} \\
&\land \text{其他排程限制符合}
\end{aligned}
$$

未設定的限制不會額外篩掉 Node。「其他限制」包含可用資源、Pod affinity／anti-affinity、儲存與拓撲等要求。

Required Node Affinity 是硬條件；Preferred Node Affinity 是節點評分的一部分，不保證選中偏好的節點。`PreferNoSchedule` 也是軟性偏好，不屬於上面的阻擋性 Taint。

## 第二層：Required Node Affinity 內部，Terms 是 OR

- 多個 `nodeSelectorTerms`：任一 Term 符合即可，彼此是 **OR**。
- 同一 Term 內多個 `matchExpressions`：全部要符合，彼此是 **AND**。
- 單一 `In` 條件的 `values`：命中其中一個值即可。

例如：

$$
(\text{disk=ssd} \land \text{zone=east})
\lor
(\text{disk=nvme} \land \text{zone=west})
$$

對應 Pod 的 `spec` 片段：

```yaml
affinity:
  nodeAffinity:
    requiredDuringSchedulingIgnoredDuringExecution:
      nodeSelectorTerms:
        - matchExpressions:
            - key: disk
              operator: In
              values: [ssd]
            - key: zone
              operator: In
              values: [east]
        - matchExpressions:
            - key: disk
              operator: In
              values: [nvme]
            - key: zone
              operator: In
              values: [west]
```

若同時設定 `nodeSelector: {env: prod}`，完整條件是：

```text
env=prod AND ((disk=ssd AND zone=east) OR (disk=nvme AND zone=west))
```

這只是 label／affinity 條件；還要通過 Taint 與其他排程限制。

## 第三層：Taint 與 Toleration 獨立檢查

一個 Node 可以有多個 Taints。先排除 Pod 已容忍的項目，再看剩下的 effect；只容忍其中一個，不代表通過其他 Taints。

| 未被容忍的 Taint effect | 新 Pod | 已經在 Node 上的 Pod |
|---|---|---|
| `NoSchedule` | 不排到這個 Node | 不單純因此被驅逐 |
| `PreferNoSchedule` | 盡量避免，但仍可能排入 | 不單純因此被驅逐 |
| `NoExecute` | 不排到這個 Node | 觸發驅逐 |

常見匹配方式：`Equal` 比對 key、value 與 effect；`Exists` 比對 key 與 effect，不填 value。effect 留空可以匹配該 key 的所有 effects，因此不要把「同 key」直接當作「一定匹配」。

例如 Node 上有 `maintenance=true:NoExecute`，以下是 Pod 的 `spec` 片段：

```yaml
tolerations:
  - key: maintenance
    operator: Equal
    value: "true"
    effect: NoExecute
    tolerationSeconds: 120
```

若這個 Taint 在 Pod 已執行後加入，Pod 可容忍 120 秒；Taint 持續存在就會在期限到後被驅逐，期限前移除則不會因這個 Taint 被驅逐。不設定 `tolerationSeconds` 表示無期限容忍這個匹配的 `NoExecute` Taint；它不保證 Pod 不會因其他原因停止。

## IgnoredDuringExecution 和 NoExecute 為什麼不衝突？

`requiredDuringSchedulingIgnoredDuringExecution` 的意思是：**排程時必須符合；排程完成後，Node Label 改變不會單純因為這條 Affinity 不再符合而驅逐 Pod。**

這個「忽略」只針對該 Affinity 條件在執行期間的變化，不會停用獨立的 Taint 驅逐機制。

假設 Pod 已在 node01 執行，以下各列獨立判斷，並假設沒有其他未容忍的 Taint：

| 事件 | 結果 |
|---|---|
| Node Label 改變，不再符合 Required Node Affinity | 不會單純因此被驅逐 |
| Node 新增 `NoSchedule` Taint | 既有 Pod 不因此被驅逐 |
| Node 新增 `NoExecute`，Pod 沒有匹配 Toleration | Pod 會被驅逐 |
| Node 新增 `NoExecute`，Pod 有匹配 Toleration，沒有時間限制 | Pod 不會因這個 Taint 被驅逐 |
| Node 新增 `NoExecute`，Pod 有 120 秒 Toleration | 若 Taint 持續存在，期限到後會被驅逐 |

「會被驅逐」描述控制平面的處置，不代表容器必定在同一瞬間停止。判斷時也要看實際 Pod 上的 Tolerations，不能只看手寫 YAML；例如 Kubernetes 通常會自動加入 not-ready／unreachable 的 300 秒 Tolerations。

## 一個完整例子：通行資格加上目的地

假設 Node 同時具備 `dedicated=gpu` label 與 `dedicated=gpu:NoSchedule` Taint。以下是 Pod 的 `spec` 片段：

```yaml
nodeSelector:
  dedicated: gpu
tolerations:
  - key: dedicated
    operator: Equal
    value: gpu
    effect: NoSchedule
```

`nodeSelector` 限定候選節點必須有該 label；Toleration 容許 Pod 通過匹配的 Taint。兩個條件都通過後，仍要檢查資源等其他要求。

只有 Toleration 時，Pod 也可能排到其他沒有這個 Taint 的 Node；只有 nodeSelector 時，匹配 label 的 Node 仍可能被未容忍的 Taint 擋住。

## 當場查與操作

以下是操作範例，node01 要換成實際節點名稱；新增／移除 Taint 會影響排程，`NoExecute` 還可能觸發驅逐。這篇筆記未對叢集執行這些操作。

```bash
alias k=kubectl

# 看 labels、Taints 與 Pod 實際落點
k get nodes --show-labels
k describe node node01
k get po -o wide
k describe po nginx

# 查 YAML 欄位
k explain po.spec.nodeSelector
k explain po.spec.affinity.nodeAffinity
k explain po.spec.tolerations

# 設定 label，以及新增／移除同一個 Taint
k label node node01 dedicated=gpu
k taint node node01 dedicated=gpu:NoSchedule
k taint node node01 dedicated=gpu:NoSchedule-
```

> [!tip] 記住三個判斷層級
> **外層限制一起滿足；Required Node Affinity 的 Terms 才是 OR；Toleration 獨立處理 Taint。**
> 先判斷「新 Pod 能不能排進去」，再判斷「已執行 Pod 會不會被驅逐」，就不會把 IgnoredDuringExecution 和 NoExecute 混在一起。

## PriorityClass：Pod 有多重要

[PriorityClass](https://kubernetes.io/docs/concepts/scheduling-eviction/pod-priority-preemption/#priorityclass) 是叢集層級的資源，縮寫 `pc`，不屬於某個 Namespace。它把類別名稱對應到整數優先權：**value 越大，優先權越高**；數字不是 CPU、Memory、時間或速度倍率。

例如 `low-priority = 1000`、`high-priority = 100000`，只代表後者優先；不代表執行速度快 100 倍。

| 欄位 | 意義 |
|---|---|
| PriorityClass 的 `value` | 使用該類別的 Pod 所取得的優先權數值 |
| Pod 的 `spec.priorityClassName` | 指定要用哪個類別 |
| Pod 的 `spec.priority` | 由 Priority admission controller 依類別解析後填入，建立時不手動填寫 |
| `globalDefault: true` | 讓未指定類別的新 Pod 使用這個類別 |
| `globalDefault: false` | 不當全域預設，需要明確指定類別才會使用 |
| `preemptionPolicy: PreemptLowerPriority` | 允許為這個 Pod 評估搶占較低優先權 Pod，為預設值 |
| `preemptionPolicy: Never` | 不主動搶占其他 Pod；仍有排程優先權，也仍可能被更高優先權 Pod 搶占 |

沒指定類別、叢集也沒設定 global default 時，Pod priority 為 0。新增 global default 不會回頭修改既有 Pod 的 priority。

```text
Pod：priorityClassName: high-priority
    → Priority admission 查到該 PriorityClass 的 value: 100000
    → Pod 自動取得 priority: 100000
    → Scheduler 依 PrioritySort 等機制處理排程
```

### 內建類別不等於一般 Pod 的預設類別

| 內建 PriorityClass | value | 用途 |
|---|---:|---|
| system-cluster-critical | 2000000000 | 叢集關鍵元件 |
| system-node-critical | 2000001000 | 節點關鍵元件，優先權更高 |

system-node-critical 的 description「must not be moved」是在說明用途，文字本身不會鎖定 Node，也不是永遠不被驅逐的保證。一般應用依重要程度建立自己的類別。

### 搶占：移除較低優先權 Pod 後，能不能合法排入？

[Preemption](https://kubernetes.io/docs/concepts/scheduling-eviction/pod-priority-preemption/#preemption) 是高優先權的待排程 Pod 找不到位置時，評估是否能終止某些較低優先權 Pod 來騰出空間。不是一看到低優先權 Pod 就驅逐；能直接排入就不需要搶占，也不保證搶占一定成功。

```text
新 Pod 等待排程
    → 先找能直接放入的 Node
    → 找不到，且新 Pod 允許 PreemptLowerPriority
    → 假設移除某台 Node 上部分較低優先權 Pod
    → 若能滿足排程條件，才可能選它們作為搶占對象
```

| 卡住的原因 | 搶占是否可能解決？ |
|---|---|
| Node 的 CPU／Memory 排程容量被其他 Pod requests 占滿 | 可能，移除 Pod 可釋放這些容量 |
| Node 有未容忍的阻擋性 Taint | 不能靠移除 Pod 解決，Taint 還在 |
| nodeSelector／Required Node Affinity 不符合 | 不能靠移除 Pod 解決，Node labels 沒改 |
| 新 Pod 單獨的需求就超過 Node 總可分配容量 | 不能，清空也放不下 |

例如唯一合格的 Node 有 4 CPU 可分配，低優先權 Pod A 的 request 占了 4 CPU；新 Pod B 的 request 是 2 CPU、priority 更高。若沒有其他條件阻擋，Scheduler 可能終止 A，等資源釋放後安排 B。A 不是被直接搬到另一台；若由 Deployment 管理，控制器會另外補建 Pod。

CPU／Memory 是否放得下主要看 **requests 與 Node allocatable**，不是直接拿 limits 或當下 CPU 使用率比較。Priority 也不會略過 Taint、Affinity 或其他硬限制；節點條件必須同時滿足前面的 Node Selector、Affinity 與 Taint／Toleration 規則。

Priority 和 QoS 是不同維度：Priority 表示重要程度；QoS 由 CPU／Memory requests／limits 等設定決定。搶占也不等同 kubelet 的節點壓力驅逐。

### 建立與檢查：用 pc

```bash
k create pc high-priority --value=100000
k create pc low-priority --value=1000
k get pc
k describe pc high-priority
k describe pc system-node-critical
```

`--preemption-policy` 預設為 `PreemptLowerPriority`，`--global-default` 預設為 false，所以題目要求允許搶占、不要設成 global default 時，上面兩條建立指令就足夠。不允許主動搶占時才加 `--preemption-policy=Never`。

### Pod 指定 PriorityClass：先產生 YAML

`k run -h` 查指令參數；run 沒有直接設定 PriorityClass 的參數，所以先產生 YAML，再補欄位，不必記一長串 JSON overrides。

```bash
k run low-prio-pod --image=nginx -n default --dry-run=client -o yaml > pod.yaml
vi pod.yaml
```

在 spec 下加入 priorityClassName，與 containers 同層：

```yaml
spec:
  priorityClassName: low-priority
  containers:
    - name: low-prio-pod
      image: nginx
```

```bash
k apply -f pod.yaml
k describe po low-prio-pod -n default

# 忘記欄位名稱時查 explain，不是猜 --priority-...
k explain po.spec | grep -i priority
```

### replace --force 的 priority: 0 錯誤

把 critical-app 改成 high-priority 時，從既有物件匯出的 YAML 可能保留舊的 priority：

```yaml
spec:
  priority: 0
  priorityClassName: high-priority
```

執行 `k replace --force -f pod.yaml` 後出現：

```text
pod "critical-app" deleted from default namespace
Error from server (Forbidden): pods "critical-app" is forbidden: the integer value of priority (0) must not be provided in pod spec; priority admission controller computed 100000 from the given PriorityClass name
```

**不是 replace 不支援，而是舊的 `priority: 0` 和 high-priority 算出的 100000 衝突。** Priority admission controller 會依類別填入 priority；建立時只指定 priorityClassName，刪掉 priority 整行，不要把它手改成 100000。

```yaml
spec:
  priorityClassName: high-priority
```

保留原本 image、resources、volumes 等所需設定。重建匯出的 Pod 時，也移除 status、metadata 中的 uid／resourceVersion／creationTimestamp／managedFields 等伺服器欄位；若希望重新排程，移除舊的 spec.nodeName，不要帶著先前的綁定直接重建。

**若錯誤已顯示舊 Pod 刪除成功，修完檔案後直接 create。** 以下以 kubectl 保留的暫存檔為例，使用時換成實際檔名：

```bash
vi /tmp/kubectl-edit-3323068860.yaml
k create -f /tmp/kubectl-edit-3323068860.yaml -n default
k get po critical-app -n default -w
```

看到 Running 後按 Ctrl-C，再查 `k describe po critical-app -n default`，確認 Priority 為 100000、Priority Class 為 high-priority。若未 Running，先看 Events，分辨 FailedScheduling、image pull 或其他啟動問題。

`replace --force` 是「刪掉舊物件，再建立新物件」，不會略過 Admission，也不是原子操作：建立失敗不會自動還原舊 Pod。若舊 Pod 還存在，先清理重建用的 YAML，再執行題目要求的重建：

```bash
k replace --force -f pod.yaml -n default
```

以上重建步驟適用本題直接管理的 Pod；Deployment 管理的 Pod 應修改其 spec.template，避免只改掉單一副本。

## Multi-scheduler：多個程序，不一定是不同二進位檔

**kube-scheduler 本身是可獨立執行的二進位程式；multi-scheduler 是部署方式，不是另一個叫 multi-scheduler 的執行檔。**

可以使用相同 kube-scheduler 程式，啟動兩個獨立程序，分別讀取不同設定；也能把它們放在不同的 Pod／容器中。第二個 Scheduler 不一定要重新寫或編譯，若需要不同自訂 plugins 或自行實作，才可能改用另一個自訂 build／執行檔。

```text
同一套 kube-scheduler 程式
    ├─ 程序 A：設定 default-scheduler → 接手對應名稱的 Pod
    └─ 程序 B：設定 my-scheduler      → 接手對應名稱的 Pod
```

第二個程序的啟動概念是：

```bash
kube-scheduler --config=/etc/kubernetes/my-scheduler/config.yaml
```

這是 Scheduler 程序的啟動方式，還需設定 API 連線、ServiceAccount／RBAC 與必要部署項目；不是執行這一行就完成所有安裝。官方的 [Multiple Schedulers 範例](https://kubernetes.io/docs/tasks/extend-kubernetes/configure-multiple-schedulers/) 是將設定存入 ConfigMap，掛載給另一個 Scheduler Pod，再由容器中的 kube-scheduler 讀取。

單副本實驗設定可以寫成：

```yaml
apiVersion: kubescheduler.config.k8s.io/v1
kind: KubeSchedulerConfiguration
profiles:
  - schedulerName: my-scheduler
leaderElection:
  leaderElect: false
```

Pod 的 `spec.schedulerName: my-scheduler` 要匹配設定中的 profile 名稱；它不是拿來對應 Scheduler Pod 的 metadata.name，也不是二進位檔名。沒填時預設為 default-scheduler；指定的 Scheduler 沒運作，Pod 會繼續等，不會自動改交給預設 Scheduler。不同 Scheduler 仍共用叢集的 Node 資源，不會因名稱不同就自動切出獨立資源池。

| 情境 | 程序與用途 |
|---|---|
| Multi-scheduler | 不同排程器程序處理不同 schedulerName 的 Pod，可使用不同策略 |
| Multi-profile | 同一個 kube-scheduler 程序載入多個 profiles，每個 profile 有自己的 schedulerName 與 plugin 設定 |
| Scheduler 高可用副本 | 同一個邏輯 Scheduler 的多個副本，使用同一組 leader election lock 選出 leader |

Multi-profile 的設定方式見 [Scheduler Profiles](https://kubernetes.io/docs/reference/scheduling/config/#multiple-profiles)。不同邏輯 Scheduler 各自做高可用時，要使用不同的 leader election lock，不能誤用同一個鎖，否則可能只剩其中一組在工作。

一般應用先用預設 Scheduler 的 nodeSelector、Affinity、Taint／Toleration 與 requests 解決需求；需要不同排程決策或特殊工作協調時，再評估多 Scheduler 或 profiles。

## Pending 排程題：最短檢查順序

```bash
k get po -o wide
k describe po <Pod名稱>
k get nodes --show-labels
k describe node <候選Node名稱>
k get pc
```

1. 先看 Events 是否為 `FailedScheduling`；若是映像或啟動錯誤，改查 Node 上的執行階段。
2. 確認 `schedulerName` 有對應且運作中的 Scheduler。
3. 逐一核對 selector／required affinity、未容忍 Taint、requests 與 Node allocatable。
4. 再看 Priority／Preemption；提高優先權不能解決 label 不符、未容忍 Taint 或單一 Pod 需求超過整台 Node 容量。

## 參考


- [Assigning Pods to Nodes：nodeSelector、Node Affinity 與 AND／OR 邏輯](https://kubernetes.io/docs/concepts/scheduling-eviction/assign-pod-node/)
- [Taints and Tolerations：effect、匹配與 tolerationSeconds](https://kubernetes.io/docs/concepts/scheduling-eviction/taint-and-toleration/)
- [Pod Priority and Preemption](https://kubernetes.io/docs/concepts/scheduling-eviction/pod-priority-preemption/)
- [Scheduling Framework](https://kubernetes.io/docs/concepts/scheduling-eviction/scheduling-framework/)
- [Scheduler Configuration 與 Profiles](https://kubernetes.io/docs/reference/scheduling/config/)
- [Multiple Schedulers](https://kubernetes.io/docs/tasks/extend-kubernetes/configure-multiple-schedulers/)

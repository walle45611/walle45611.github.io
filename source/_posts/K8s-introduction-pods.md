---
title: K8s 中的 Pod 是什麼
date: 2024-11-28 10:09:29
tags: k8s
categories: [IT 技術,k8s,container]
---

![圖 1. : Blog Image](https://imgur.com/tulZWKO.png)

<!--more-->

## Pod 是什麼

Pod 是可以在 K8s 創建和管理的最小單位。

一個 Pod 是一個群組，裡面可以有一個或是多個 containers, 共享 storage 和 resource 和怎麼運行這個 Pod 的相關 specification (規格，設定)，Pod 內的容器在同一個節點被調度和運行，，形成一個應用程序的「邏輯主機」，類似於在非雲端環境下同一實體機或虛擬機中的應用。

* Pod 可以包含以下幾種容器：
  1. Application pod：執行主要的應用程序邏輯。
  2. init containers：在 Pod 啟動過程中執行。
  3. Ephemeral Containers：用排查問題。

## Pod theory

The atomic unit of scheduling on Kubernetes is the Pod. This is just a fancy way of saying apps deployed to
Kubernetes always run inside Pods.
Some quick examples… If you deploy an app, you deploy it in a Pod. If you terminate an app, you terminate its
Pod. If you scale an app up or down, you add or remove Pods.

在 K8s 中，排程的最小單位是 Pod。意指所有部署到 K8s 中的 Application，都必須在 Pod 中運行。

1. 當需要部署一個 Application 時，實際上是將其部署到 Pod 中。
2. 當終止一個 Application 時，實際上也是中指他所在的 Pod。
3. 當你調整應用程式的規模時，無論是擴展還是縮減，都是透過新增或移除 Pod 來實現。

## Pod lifecycle

### Pod phase

首先 Pod 會有自己的生命週期，Pod 會遵循這些狀態創建、刪除、執行等等...

* 相關重點
  * Binding：如果一個 Pod 被分配到特定的節點，我們稱之為綁定 (binding)。
  * Scheduling：選擇使用哪個節點的過程稱為調度 (scheduling)。
  * 調度限制：Pod 的調度 (scheduling) 只會進行一次，並且該 Pod 將持續運行於此節點，直到被停止或終止 (terminated)。

* Pod 的主要狀態
  * Peding : Pod 已經創但是尚未啟動，此街段包括 Pod 被 scheduling 或是下載 Image。
  * Running : Pod 正在運行，至少有一個 container 在執行中，或著處於啟動或是重新啟動狀態。
  * Succeded : 所有的 containers 都成功 terminated，且不會再次重新啟動。
  * Failed : Pod 中的所有 containers 都已經 terminated，並且至少有一個 container 失敗 terminated，且為設定自動重新啟動。
  * Unknown : 無法獲取 Pod 的狀態。

* 常見狀態說明
  * CrazhLoopBackOff：如果 Pod 反覆重啟失敗，kubectl 的 Status 欄位會顯示此狀態，表示容器正在嘗試啟動但持續失敗。
  * Terminating：當刪除 Pod 時，Status 欄位可能顯示 Terminating，表示 Pod 正在關閉中。

* Pod 終止時間
  * 當 Pod 被終止時，會有一個默認為 30 秒的優雅關閉時間，用於清理資源並完成關閉過程。
  * 如果需要強制終止 Pod，可以使用 --force 標誌來繞過優雅關閉時間。

### Pod status

* Waiting：container 正在完成他啟動時需要的動作，例如：從某個 registry 取得 image，或者 container 正在取得 Secret 資料等等
* Running：這個狀態表示 container 執行中並沒有發生什麼問題。
* Terminated：這個狀態表示 container 在執行後，因為某些原因導致失敗所產生的狀態，或是正常關閉的狀態。

## 從程式碼到 Pod

1. 寫程式碼
2. 打包你的 container image
3. 打吧你的 container image 到 Pod
4. 把 Pod 跑在 K8s 上

* 那為什麼不直接跑 container 在 K8s 上，以下有幾個比較短的理由
  * Pods 增強了 containers
    * 可以在 Pods 上加上 Labels 和 annotations
    * Pods 重啟的一些政策
    * Probes(探針)，startup probes、readiness prbes、liveness probes，和更多。
    * Affinity 和 anti-affinity （親和性和反親和信）
  * Pods 可以被調度 (scheduling)
  * Pods 可以開啟資源共享

## Pod 的資源管理和限制資源

* 以下指令可以先建立一個模板

    ```bash
    kubectl run nginx-pod --image=nginx:latest --dry-run=client -o yaml > nginx-pod.yaml
    ```

    ```yaml
    apiVersion: v1
    kind: Pod
    metadata:
    creationTimestamp: null
    labels:
        run: nginx-pod
    name: nginx-pod
    spec:
    containers:
    - image: nginx:latest
        name: nginx-pod
        resources: {}
    dnsPolicy: ClusterFirst
    restartPolicy: Always
    status: {}
    ```

* 修改為以下的 yaml

    ```yaml
    apiVersion: v1
    kind: Pod
    metadata:
    creationTimestamp: null
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
    status: {}
    ```

  * requests 定義這個 container 需要的最小資源量。K8s 會選擇一個可用的 node，選擇一個能滿足這些請求的 node 來直行這個 container：
    * `memory : "128Mi"`：表示了這個 container 需要至少 128 Mebibytes 才能啟動。
    * `CPU : "500m"`：表示該 container 需要 0.5 個 CPU core。
  * limits 定義了容器能使用的資源上限。如果容器嘗試使用超過這個值的資源：
    * memory 超過限制：container 會被 kill （Out of memory, OOM）。
    * CPU 過過限制：container 的 CPU 會被限制不會被 kill，但可能性能會被降低。
  * 套用以上設定

    ```bash
    kubectl apply -f nginx-pod.yaml
    ```

* 查看該 Pod 的狀態

    ```bash
    kubectl describe pod nginx-pod
    ```

    ```text
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

## Pod 相關指令

* 需要查看 Pod 的相關 logs

  ```bash
  kubectl logs <pod-name>
  ```

* 這個指令可以查尋 Pod 的相關屬性

    ```bash
    kubectl explain pods --recursive | less
    ```

    ```text
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

* 預設會從 default namespace 取得 pods 相關資訊

    ```bash
    kubectl get pods

    NAME    READY   STATUS    RESTARTS   AGE
    nginx   1/1     Running   0          8m7s
    ```

  * 如果要找到更多相關資訊

    ```bash
    kubectl get pods -o wide
    NAME    READY   STATUS    RESTARTS   AGE   IP              NODE       NOMINATED NODE   READINESS GATES
    nginx   1/1     Running   0          12m   192.168.194.8   orbstack   <none>           <none>
    ```

  * 以下指令可以取得所有 namespace 底下的 pods

    ```bash
    kubectl get pods --all-namespaces

    NAMESPACE     NAME                                      READY   STATUS    RESTARTS   AGE
    kube-system   local-path-provisioner-6c86858495-m5x7g   1/1     Running   0          148d
    kube-system   coredns-66cc6945cb-hswrk                  1/1     Running   0          148d
    default       nginx                                     1/1     Running   0          10m
    ```

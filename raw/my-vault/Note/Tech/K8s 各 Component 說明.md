## Kubernetes 設計原則

來源：KubeCon + CloudNativeCon North America 2018 投影片〈Kubernetes Principles Introduced〉。

1. **宣告式優先於命令式（Kube API declarative over imperative）**：描述系統應達到的期望狀態，由 Kubernetes 持續協調實際狀態與期望狀態之間的差異。
2. **沒有隱藏的內部 API（No hidden internal APIs）**：系統內部使用的 API 也應對外開放，讓外部工具能透過相同介面整合。
3. **配合使用者既有的環境與需求（Meet the user where they are）**：設計應考量使用者現有的工具、工作流程與使用情境。
4. **工作負載可攜性（Workload portability）**：讓工作負載能在不同執行環境之間移轉，降低對特定基礎設施的依賴。

---

## API Server

- **功能**：
    - Kubernetes 元件透過 API Server 存取叢集資源與狀態；CRI、CSI 等元件間介面則有各自的通訊路徑。
    - 提供 RESTful API，允許通過 HTTPS 傳送 YAML 配置。
    - 負責身份驗證（Authentication）和授權（Authorization）。

---

## Cluster Store

- **功能**：
    - 負責儲存 Kubernetes 的狀態數據。
    - 沒有 Cluster Store 就沒有 Cluster。
- **基礎**：
    - 基於 etcd，一種流行的分佈式數據庫。
    - 建議為高可用性（HA）運行 3-5 個 etcd 副本。

---

## Controller Manager 和 Controllers

- **功能**：
    - Controller Manager 實現所有的後台控制器，監控集群元件並響應事件。
    - 是 "控制器的控制器"，會生成所有獨立的控制器並監控它們。
- **設計模式**：
    - Kubernetes 採用聲明式設計模式。
    - 操作流程：
        1. 獲取期望狀態。
        2. 觀察當前狀態。
        3. 判定差異。
        4. 調和差異。

---

## Scheduler（調度器）

- **功能**：
    - 監控 API Server 中的新工作任務，將其指派給適合的健康節點。
    - 背後實現了複雜邏輯以篩選不適合的節點，並對符合條件的節點進行排序，最終選擇得分最高的節點執行任務。
- **檢查項目**：
    - 進行資源檢查（predicate checks），確認節點是否有足夠資源執行工作。

---

## Kubernetes Node

### kubelet

- Kubernetes 的主要 Agent，運行在每個節點上。
- Node 是節點，kubelet 是節點上的代理程式，兩者不是同義詞。詳見 [[K8s kubelet 與 Pod 執行流程]]。

### Container Runtime

- kubelet 需要 Container Runtime 來執行容器相關任務，如拉取映像、啟動和停止容器。
- Kubernetes v1.24 移除內建 dockershim；Docker Engine 可透過 cri-dockerd 銜接，Docker 建立的相容映像仍可使用。詳見 [[K8s Container Runtime 與 dockershim 遷移]]。
- **containerd** 是 Docker Engine 的容器監督邏輯，被捐贈給 CNCF 並得到廣泛支持。

### kube-proxy

- **功能**：
    - 實作 Kubernetes Service 的部分網路轉送功能，不負責分配 Node IP。
    - 依使用模式設定本機封包轉送規則，將 Service 流量導向後端 Pod。
- **流量處理**：
    - Pod 網路連通由 CNI 等網路元件提供；具備替代能力的實作可接手 kube-proxy 的 Service 功能，見 [[K8s CNI 比較：Cilium、Calico 與 Flannel]]。

---

## Packaging Apps for Kubernetes

### Containerized

- **定義**：
    - 使用選擇的程式語言開發應用微服務，將其打包為容器映像並存儲於映像庫（Registry）。

### Pod

- **定義**：
    - Pod 是容器在 Kubernetes 集群中運行的封裝單位。
- [了解更多](https://blog.walle4561.com/2024/11/28/K8s-introduction-pods/)

### Deployment

- **定義**：
    - 雖然可以直接運行靜態 Pod，但推薦通過更高級的控制器部署所有 Pod。
    - 最常見的控制器是 Deployment，提供以下功能：
        - 可擴展性（Scalability）。
        - 自愈能力（Self-healing）。
        - 滾動更新（Rolling updates）。
    - 使用 YAML 配置文件定義 Deployment，例如指定副本數量和更新方式。

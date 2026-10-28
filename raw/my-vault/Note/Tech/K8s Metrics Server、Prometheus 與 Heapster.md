---
blog: true
blog_title: 'K8s Metrics Server、Prometheus 與 Heapster'
blog_date: '2026-10-29'
blog_url: https://blog.walle4561.com/articles/posts/k8s-metrics-server-prometheus-heapster/
blog_tag: Kubernetes
blog_topic: Kubernetes
---

# K8s Metrics Server、Prometheus 與 Heapster

> [!tip] 題目辨識
> 要 `kubectl top` 或 CPU／Memory 自動擴縮的資源指標 → Metrics Server。要歷史曲線、查詢與告警 → Prometheus。Heapster 是已終止維護的舊方案。

| 元件 | 核心用途 | 考前要記 |
|---|---|---|
| Heapster | 舊版叢集資源監控管線 | 已 EOL，不作為新部署選項 |
| Metrics Server | 收集 kubelet 的近期 CPU／Memory 資源用量，提供 Metrics API | 支援 top、HPA 的資源指標及 VPA 的用量來源；不做長期歷史儲存 |
| Prometheus | 抓取、儲存時間序列，支援 PromQL 與告警規則 | 不會因為裝了它，就自動提供 Metrics Server 的 API |

```text
kubelet → Metrics Server → metrics.k8s.io → kubectl top／HPA 等使用者
應用與元件的 metrics endpoint → Prometheus → 歷史查詢／圖表／告警
```

「Deploy Metrics Server to enable monitoring」指的是把資源指標服務裝進叢集，不是部署完整監控看板。Prometheus 若要提供 HPA 的 custom／external metrics，通常還需要對應的 adapter。

## 必要操作

使用題目提供、且與叢集版本相容的 manifest；以下假設已取得 `components.yaml`：

```bash
kubectl apply -f components.yaml
kubectl top nodes
kubectl top pods -A
```

部署後需要等收集到資料。若顯示 Metrics API unavailable，先查：

```bash
kubectl logs -n kube-system deploy/metrics-server
```

依錯誤分辨 kubelet 連線、憑證或 API aggregation 問題。`--kubelet-insecure-tls` 是略過 kubelet 憑證驗證的實驗環境權宜設定，不是預設安裝必要參數。

> [!warning] 用量不等於配置
> `top` 顯示近期使用量；Pod 的 requests／limits 是資源需求與限制。不能用 top 的數字直接判定排程器應該把 Pod 排在哪。

相關：[[K8s 中的 Pod 是什麼#Pod 的資源管理和限制資源]]。

## 參考

- [Metrics Server 官方說明與版本相容性](https://github.com/kubernetes-sigs/metrics-server)
- [Heapster：已 EOL](https://github.com/kubernetes-retired/heapster)
- [Kubernetes 資源指標管線](https://kubernetes.io/docs/tasks/debug/debug-cluster/resource-metrics-pipeline/)
- [HPA 與自訂指標](https://kubernetes.io/docs/tasks/run-application/horizontal-pod-autoscale/)

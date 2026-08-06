# K8s 中的 Pod 是什麼

- source: `raw/k8s-introduction-pods.md`
- original title: K8s 中的 Pod 是什麼
- author: Walle
- published: 2024-11-28
- source_created: 2024-11-28
- ingested_at: 2026-08-03
- type: migrated blog article summary

## Summary

這篇文章介紹 Kubernetes 的 Pod，涵蓋 Pod 的定義、容器組成、生命週期、狀態、資源限制與常用 kubectl 操作，並附上 YAML 與實作步驟。

## Key Points

- Pod 是 Kubernetes 中可以被建立與管理的最小單位，也是容器被調度與執行的邏輯主機。
- 一個 Pod 可以包含 application、init 與 ephemeral containers，並共享儲存與執行規格。
- 文章區分 Pod phase、status、生命週期與資源 requests/limits。
- 原文的 YAML 範例與 kubectl 指令保留在 Blog source，版本相依操作仍需對照目前 Kubernetes 文件。

## Related Concepts

- [blog-knowledge-migration](../concepts/blog-knowledge-migration.md)
- [microk8s-production-readiness](../concepts/microk8s-production-readiness.md)

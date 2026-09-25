# MicroK8s 高可用安裝與設定

- source: `raw/my-vault/Note/Tech/MicroK8s 高可用安裝與設定.md`
- source_sha256: `8a58453460156944ea8d4395df7dcbcf364b8ecfa46db5bb1331cf75fb03133b`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 MicroK8s 能否用於 Production？、結論與建議、系統需求、安裝與基本設定、安裝 MicroK8s、啟用必要插件。

## Source Notes

- 在最近專題開發的過程中，我反覆安裝與測試了多次 MicroK8s，期間踩了不少坑，也累積了許多實戰經驗。因此，我決定整理這篇 Blog，讓自己以後部署時可以有一份可直接複製使用的指令手冊，也幫助正在學習 Kubernetes 的朋友少走一些彎路。
- 本文是基於 MicroK8s 官方文件、Helm、Argo CD Image Updater 等官方說明整合的安裝流程，並修正了常見名詞與設定錯誤，方便直接在 Ubuntu 環境中快速部署高可用 Kubernetes 集群。
- 在 Canonical 官方文章中，MicroK8s 被明確描述為 「強大、輕量、可靠的 production‑ready Kubernetes 發行版」，並稱其為「企業級 Kubernetes 發行版」，內建多種生產等級插件（如 Istio、Knative、Grafana、Cilium 等）。官方寫道

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)

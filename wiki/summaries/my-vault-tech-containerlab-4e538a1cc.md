# Containerlab 安裝與設定指南

- source: `raw/my-vault/Note/Tech/Containerlab 安裝與設定指南.md`
- source_sha256: `2b7240120fa72f40dd6d218e2e6d791a5549ee86e18307505c2d010599aa2f2d`
- source_reviewed: 2026-10-09
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 環境架構、建立並進入 Debian machine、安裝 Docker 與 Containerlab、建立最小測試 Lab、建置 Cisco IOL 映像、部署 Cisco Router 與 Switch。

## Source Notes

- 以 Apple Silicon Mac 搭配 OrbStack 的 Debian ARM64 Linux machine 建立網路實驗環境，先驗證 Containerlab，再加入 Cisco IOL Router 與 Switch。
- 來源：Mac安裝網路模擬器；整理日期：2026-09-20。
- Containerlab 透過 YAML 定義節點與連線，再建立容器及實驗網路。它需要 Linux 的 network namespace、veth、netlink 等功能，因此本流程在 Debian 裡執行 Containerlab 與 Docker。這是 Containerlab macOS 官方指南列出的方式。

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)

## 2026-10-09 來源複核

來源現含 UTM Debian ARM64、Rosetta／IOL 與 Alpine 登入／IP 操作補充；br-fgt 接線留在 [[fortigate-cisco-iol-utm-ospf-lab]]。不同 VM 平台、架構與節點 kind 的前提不可混用。

目前來源章節：Containerlab 安裝與設定指南、環境架構、1. 建立並進入 Debian machine、2. 安裝 Docker 與 Containerlab、3. 建立最小測試 Lab、4. 建置 Cisco IOL 映像、5. 部署 Cisco Router 與 Switch、6. 查看拓撲與管理 Lab、常見問題、UTM Debian ARM64：安裝與排錯、與 OrbStack 安裝的差別、Debian 安裝 Docker、Containerlab 與 IOL 映像、外部 FortiGate VM 的 bridge 接線、IOL redeploy 後仍無法 SSH、排查順序、Alpine 節點操作補充、登入 Alpine 節點、設定 Alpine 實驗介面。

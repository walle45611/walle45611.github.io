# 摘要：如何免費同步 Obsidian

- source: https://www.youtube.com/watch?v=t3cy132eeUU
- original title: How to sync Obsidian 4 FREE · Syncthing vs Autosync
- speaker: DJ Lensing
- published: 2022-09-22
- type: YouTube transcript summary

## 核心主張

Obsidian 本身免費，但官方 Sync 是付費服務。若預算有限，可透過第三方工具實現免費同步。

## 重要概念與工具

- **Autosync 類 (DriveSync/Dropsync/OneSync)**：
  - 適用於 Google Drive, Dropbox, OneDrive。
  - 優點：容易設定。
  - 缺點：同步過程可能受 Android App 沙盒限制影響，需適應工作流。
- **Syncthing**：
  - 點對點 (P2P) 同步工具，開源且加密。
  - 優點：不依賴第三方雲端服務，被視為更穩健的免費方案。
- **重要提醒**：
  - **同步 $\neq$ 備份**。設定同步前務必進行完整備份。
  - **不要混用多種同步服務**，容易造成衝突與檔案遺失。

## 對知識庫的啟示

- 這份來源確立了 Obsidian 同步的基礎認知：同步方案的選擇取決於對便利性、隱私性與技術複雜度的權衡。
- 未來可視需求建立 [[sync-obsidian]] 概念頁，整理各類同步方案的實務邊界與衝突處理指南。

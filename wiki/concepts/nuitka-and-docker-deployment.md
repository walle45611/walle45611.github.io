---
title: "Python 編譯與容器打包"
tags:
  - concept/cloud
---

# Python 編譯與容器打包

已收錄文章討論 Nuitka 與 Docker multi-stage build 的搭配：在 builder 階段編譯，再把需要的產物放進 runtime image，讓建置環境與交付環境分開。

## 編譯與打包的責任

Nuitka 處理 Python 編譯，Docker 管理映像與執行環境；最後仍需核對依賴、共享函式庫、目標架構、entrypoint 與運作結果。使用 minimal／scratch base 必須符合產物實際需求，不能僅複製一個 binary 就假設完整可用。

## 來源數據與限制

舊概念頁把約 15 MB、快 4–6 秒等來源描述寫成通用結論，現改限於該文章的案例敘述。映像大小、啟動速度、memory、CI 時間與安全效果不能從工具名稱推定，需目標應用比較；編譯也不等於程式與資料已受完整保護。依賴保留與 standalone 模式條件仍需版本文件驗證。

## 相鄰概念

- [[kubernetes-design-principles]]：映像交付之後的叢集運維與 GitOps。
- [[local-llm-deployment]]：架構與 runtime 相容性也須實測。

## 來源

- [[nuitka-and-docker-for-high-performance-and-secure-python-deployment]]

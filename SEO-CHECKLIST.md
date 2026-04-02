# SEO 檢查清單

本清單提供新文章發布前的 SEO 檢查項目和定期維護任務。

---

## 📝 新文章發布前檢查清單

### Front Matter 必填項目

```yaml
---
title: 文章標題（50-60 字元最佳）
date: 2026-04-02 12:00:00
tags:
  - 標籤1
  - 標籤2
  - 標籤3
description: 簡短描述文章內容，120-155 字元，吸引讀者點擊
categories: 分類名稱
---
```

#### ✅ 檢查項目：

- [ ] **標題 (title)**
  - [ ] 長度在 50-60 字元之間
  - [ ] 包含主要關鍵字
  - [ ] 清晰描述文章內容
  - [ ] 避免過度使用符號

- [ ] **描述 (description)**
  - [ ] 已填寫（不能空白）
  - [ ] 長度在 120-155 字元之間
  - [ ] 包含 1-2 個關鍵字
  - [ ] 吸引人且具體
  - [ ] 準確描述文章價值

- [ ] **標籤 (tags)**
  - [ ] 至少 3 個標籤
  - [ ] 不超過 7 個標籤
  - [ ] 使用相關且具體的關鍵字
  - [ ] 避免太廣泛的標籤（如「技術」）
  - [ ] 標籤名稱一致（檢查是否已存在相同標籤）

- [ ] **分類 (categories)**
  - [ ] 已設定分類
  - [ ] 分類與文章主題相符
  - [ ] 遵循既有分類結構

- [ ] **日期 (date)**
  - [ ] 日期正確
  - [ ] 時間格式正確

---

### 內容結構檢查

- [ ] **文章長度**
  - [ ] 至少 300 字（建議 500+ 字）
  - [ ] 長文章通常 SEO 表現更好

- [ ] **標題層級**
  - [ ] 使用正確的 H2, H3, H4 層級
  - [ ] 不跳級（不從 H2 直接到 H4）
  - [ ] H2 標題包含相關關鍵字
  - [ ] 標題清晰描述內容

- [ ] **文章摘要**
  - [ ] 使用 `<!--more-->` 設定摘要位置
  - [ ] 摘要長度 150-250 字
  - [ ] 摘要吸引讀者繼續閱讀

- [ ] **內部連結**
  - [ ] 至少 2-3 個內部連結到相關文章
  - [ ] 連結文字描述明確（不用「點這裡」）
  - [ ] 連結有效且相關

- [ ] **外部連結**
  - [ ] 外部連結來源可靠
  - [ ] 考慮使用 `rel="noopener noreferrer"`

---

### 圖片最佳化檢查

- [ ] **Alt Text**
  - [ ] 所有圖片都有 alt 屬性
  - [ ] Alt text 描述性且簡潔（125 字元以內）
  - [ ] 自然地包含關鍵字
  - [ ] 避免「這是一張...的圖片」等贅字

- [ ] **圖片優化**
  - [ ] 圖片已壓縮（使用 TinyPNG 等工具）
  - [ ] 圖片尺寸適當（不要上傳 4K 圖片）
  - [ ] 考慮使用 WebP 格式
  - [ ] 使用 CDN 載入圖片（如 blog-images.walle4561.com）

- [ ] **圖片檔名**
  - [ ] 檔名描述性（如 `kubernetes-architecture.png`）
  - [ ] 避免 `IMG_1234.png` 等無意義檔名
  - [ ] 使用英文檔名（避免中文）

---

### 程式碼區塊檢查

- [ ] **語法高亮**
  - [ ] 所有程式碼區塊都指定語言
  - [ ] 語言標記正確（javascript, python, bash 等）

- [ ] **程式碼品質**
  - [ ] 程式碼簡潔易懂
  - [ ] 關鍵邏輯有註解
  - [ ] 必要時分段說明

---

### 技術檢查

- [ ] **本地測試**
  - [ ] 執行 `hexo clean && hexo generate` 無錯誤
  - [ ] 執行 `hexo server` 本地預覽正常
  - [ ] 檢查文章排版和顯示

- [ ] **Front Matter 格式**
  - [ ] YAML 格式正確
  - [ ] 沒有語法錯誤
  - [ ] 縮排正確

---

## 📅 定期 SEO 維護清單

### 每週檢查

- [ ] **新文章品質**
  - [ ] 檢查本週發布的文章是否遵循 SEO 最佳實踐
  - [ ] 確認所有圖片都有 alt text
  - [ ] 驗證內部連結有效

- [ ] **效能監控**
  - [ ] 檢查網站載入速度
  - [ ] 確認 CSS/JS 壓縮啟用

### 每月檢查

- [ ] **Google Analytics**
  - [ ] 查看流量趨勢
  - [ ] 識別熱門文章
  - [ ] 分析用戶行為（停留時間、跳出率）

- [ ] **Google Search Console**（如已設定）
  - [ ] 檢查搜尋表現
  - [ ] 查看點擊率和曝光
  - [ ] 識別排名提升的關鍵字
  - [ ] 檢查索引狀態

- [ ] **連結檢查**
  - [ ] 檢查是否有失效的內部連結
  - [ ] 檢查外部連結是否仍然有效
  - [ ] 修復 404 錯誤

- [ ] **內容更新**
  - [ ] 識別過時的內容
  - [ ] 更新技術文章的版本資訊
  - [ ] 修正錯誤或遺漏

### 每季檢查

- [ ] **SEO 健康檢查**
  - [ ] 使用 PageSpeed Insights 測試效能
  - [ ] 使用 Google Rich Results Test 驗證結構化資料
  - [ ] 檢查 sitemap.xml 和 atom.xml 正常生成

- [ ] **關鍵字分析**
  - [ ] 分析哪些關鍵字帶來流量
  - [ ] 調整內容策略
  - [ ] 識別新的關鍵字機會

- [ ] **競爭對手分析**
  - [ ] 查看類似主題的熱門文章
  - [ ] 學習他們的 SEO 策略
  - [ ] 識別內容缺口

### 每年檢查

- [ ] **全面 SEO 審查**
  - [ ] 檢閱所有 SEO 設定是否仍然最佳
  - [ ] 更新 SEO 策略
  - [ ] 考慮新的 SEO 技術和工具

- [ ] **內容盤點**
  - [ ] 統計文章數量和分類分佈
  - [ ] 識別需要更新或刪除的內容
  - [ ] 規劃下一年的內容方向

---

## 🚀 部署前最終檢查

### 每次部署前

- [ ] **配置檢查**
  - [ ] `_config.yml` 無錯誤
  - [ ] `_config.next.yml` 配置正確
  - [ ] 所有變更已提交到 Git

- [ ] **建構測試**
  - [ ] `hexo clean` 清理舊檔案
  - [ ] `hexo generate` 成功生成
  - [ ] 檢查生成的 `public/` 目錄

- [ ] **SEO 檔案驗證**
  - [ ] `public/sitemap.xml` 存在
  - [ ] `public/atom.xml` 存在
  - [ ] `public/robots.txt` 存在且正確

- [ ] **推送和部署**
  - [ ] 提交變更到 Git
  - [ ] 推送到 GitHub
  - [ ] 等待 Cloudflare Pages 部署完成

---

## 🧪 部署後驗證清單

### 線上測試（每次部署後）

- [ ] **基本功能**
  - [ ] 訪問首頁 https://blog.walle4561.com
  - [ ] 訪問最新文章
  - [ ] 測試搜尋功能

- [ ] **SEO 檔案**
  - [ ] 訪問 https://blog.walle4561.com/sitemap.xml
  - [ ] 訪問 https://blog.walle4561.com/atom.xml
  - [ ] 訪問 https://blog.walle4561.com/robots.txt

- [ ] **社群分享測試**
  - [ ] 使用 [Twitter Card Validator](https://cards-dev.twitter.com/validator) 測試
  - [ ] 使用 [Facebook Sharing Debugger](https://developers.facebook.com/tools/debug/) 測試
  - [ ] 檢查分享預覽圖和描述

- [ ] **結構化資料**
  - [ ] 使用 [Google Rich Results Test](https://search.google.com/test/rich-results) 測試
  - [ ] 確認無錯誤或警告

- [ ] **效能測試**
  - [ ] 使用 [PageSpeed Insights](https://pagespeed.web.dev/) 測試
  - [ ] 確認分數在綠色範圍（90+）
  - [ ] 檢查 Core Web Vitals

---

## 📋 快速參考

### 理想的文章 Front Matter 範例

```yaml
---
title: Kubernetes MicroK8s 完整部署指南
date: 2026-04-02 12:00:00
tags:
  - Kubernetes
  - MicroK8s
  - DevOps
  - Container
  - Ubuntu
description: 深入探討 Kubernetes MicroK8s 的完整部署流程，包含環境設定、叢集配置與實戰應用，適合初學者入門學習容器編排技術。
categories: DevOps
---
```

### 理想的圖片 Markdown 範例

```markdown
![Kubernetes MicroK8s 架構圖展示控制平面、工作節點和服務網格的關係](https://blog-images.walle4561.com/microk8s-architecture.png)
```

### 理想的內部連結範例

```markdown
關於 Kubernetes 的基礎概念，請參考我之前的文章 [Kubernetes 入門指南](/2025/05/15/kubernetes-basics/)。
```

---

## 🔗 相關資源

- [SEO-GUIDE.md](./SEO-GUIDE.md) - 完整的 SEO 最佳實踐指南
- [SEO-TEST-REPORT.md](./SEO-TEST-REPORT.md) - SEO 優化測試報告
- [Google 搜尋中心](https://developers.google.com/search/docs)
- [Hexo 官方文檔](https://hexo.io/docs/)

---

## 💡 提示

- **不要過度優化**: SEO 很重要，但內容品質更重要
- **保持一致性**: 使用此檢查清單維持文章品質一致
- **定期檢視**: 每月回顧並更新此清單
- **記錄改進**: 追蹤 SEO 表現並持續改進

---

**記住**: 好的 SEO 從好的內容開始。專注於為讀者創造價值，SEO 自然會跟隨！✨

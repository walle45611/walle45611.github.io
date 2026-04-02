# SEO 最佳實踐指南

本指南提供撰寫 SEO 友善部落格文章的最佳實踐，適用於 Walle Blog。

## 📝 Front Matter 欄位規範

### 必填欄位

```yaml
---
title: 你的文章標題
date: 2026-04-02 11:42:00
tags:
  - Tag1
  - Tag2
description: 簡短描述文章內容，120-155 字元最佳，吸引讀者點擊
categories: 分類名稱
---
```

### Description 撰寫指南

**✅ 好的範例**:
```yaml
description: "深入探討 Kubernetes MicroK8s 的完整部署流程，包含環境設定、叢集配置與實戰應用，適合初學者入門。"
```
- ✅ 120-155 字元長度（搜尋結果預覽的理想長度）
- ✅ 包含關鍵字（Kubernetes、MicroK8s、部署）
- ✅ 說明文章價值和目標讀者
- ✅ 吸引人且具體

**❌ 不好的範例**:
```yaml
description: "技術文章"  # 太短，無意義
description: "本文詳細介紹了 Kubernetes MicroK8s 的所有功能、特性、使用方法、部署流程、配置選項、網路設定、儲存管理、安全性考量以及疑難排解方案，並且提供了大量的實際範例和最佳實踐建議..."  # 太長，超過 155 字元
description: "這是一篇關於 K8s 的文章"  # 太模糊，無關鍵資訊
```

### Keywords 選擇建議

**選擇原則**:
1. **相關性**: 與文章內容直接相關
2. **具體性**: 使用具體的技術名詞而非泛泛之談
3. **搜尋意圖**: 考慮讀者會搜尋什麼關鍵字
4. **數量**: 3-7 個關鍵字最佳

**✅ 好的範例**:
```yaml
tags:
  - Kubernetes
  - MicroK8s
  - Container
  - DevOps
  - Ubuntu
```

**❌ 不好的範例**:
```yaml
tags:
  - 技術  # 太廣泛
  - 好用的工具  # 不專業
  - K8s  # 縮寫可以，但應加上完整名稱
```

## 🖼️ 圖片最佳實踐

### Alt Text 規範

**目的**:
- 讓螢幕閱讀器使用者了解圖片內容（無障礙）
- 圖片無法載入時顯示替代文字
- 幫助搜尋引擎索引圖片內容

**撰寫原則**:
1. **描述性**: 清楚描述圖片內容
2. **簡潔**: 通常 125 字元以內
3. **避免贅字**: 不需要寫「這是一張...的圖片」
4. **包含關鍵字**: 自然地融入相關關鍵字

**✅ 好的範例**:
```markdown
![MicroK8s 架構圖展示控制平面、工作節點和服務網格的關係](https://blog-images.walle4561.com/microk8s-architecture.png)

![終端機顯示 kubectl get pods 命令的輸出結果](https://blog-images.walle4561.com/kubectl-output.png)

![資料結構圖解：二元搜尋樹的插入操作流程](https://blog-images.walle4561.com/bst-insert.png)
```

**❌ 不好的範例**:
```markdown
![圖片](url)  # 完全無意義

![圖 1.](url)  # 太簡短，無描述

![這是一張展示 MicroK8s 架構的圖片](url)  # 有贅字「這是一張...的圖片」

![screenshot](url)  # 英文且無具體描述
```

### 圖片檔案命名

**✅ 好的範例**:
- `microk8s-architecture-diagram.png`
- `kubernetes-pod-lifecycle.jpg`
- `binary-search-tree-traversal.png`

**❌ 不好的範例**:
- `IMG_1234.png`
- `screenshot.png`
- `圖片1.jpg`（避免中文檔名）

## 📑 內容結構最佳實踐

### 標題層級

**使用規則**:
```markdown
# 文章標題（Front Matter 的 title，不要在內文重複）

## 主要章節（H2）

### 次要章節（H3）

#### 細節說明（H4）
```

**注意事項**:
- ✅ 按順序使用（不要跳級，如從 H2 直接到 H4）
- ✅ 每個 H2 標題應包含相關關鍵字
- ✅ 標題應清晰描述內容
- ❌ 不要全用大寫
- ❌ 不要在標題內使用太多符號

### 文章摘要

使用 `<!--more-->` 標記來設定文章摘要：

```markdown
---
title: 文章標題
description: SEO 描述
---

簡短的文章介紹，說明這篇文章的主要內容和讀者可以學到什麼...

<!--more-->

詳細的文章內容從這裡開始...
```

**摘要撰寫原則**:
- 2-3 段落（約 150-250 字）
- 說明文章主題和價值
- 吸引讀者繼續閱讀
- 可以包含關鍵字，但要自然

## 🔗 內部連結策略

**為什麼重要**:
- 幫助搜尋引擎了解網站結構
- 提升相關文章的權重
- 改善用戶體驗（引導讀者閱讀更多內容）

**實作方式**:
```markdown
如前文 [Kubernetes 基礎概念](/2025/06/23/kubernetes-basics/) 所述...

關於二元樹的詳細介紹，請參考 [資料結構：樹狀結構](/2025/05/15/tree-data-structure/)。
```

**最佳實踐**:
- ✅ 每篇文章至少 2-3 個內部連結
- ✅ 連結文字描述明確（避免「點這裡」、「這篇文章」）
- ✅ 連結相關主題的文章
- ✅ 檢查連結有效性

## 🚀 效能最佳化

### 圖片優化

**建議**:
- 使用 WebP 格式（更小的檔案大小）
- 壓縮圖片（使用 TinyPNG、ImageOptim 等工具）
- 適當的圖片尺寸（不要上傳 4K 圖片用於部落格）
- 考慮使用 CDN（如 Cloudflare Images）

### 程式碼區塊

**最佳實踐**:
```markdown
​```javascript
// 指定語言以啟用語法高亮
function example() {
  return "Hello World";
}
​```
```

- ✅ 總是指定程式碼語言
- ✅ 保持程式碼簡潔（必要時分段）
- ✅ 添加註解說明關鍵邏輯

## 📊 SEO 檢查清單

發布文章前，請確認：

### 基本檢查
- [ ] 標題有意義且包含關鍵字（50-60 字元最佳）
- [ ] Description 欄位已填寫（120-155 字元）
- [ ] 至少 3 個相關的 tags
- [ ] Categories 已設定
- [ ] 文章長度至少 300 字（長文章通常 SEO 表現更好）

### 圖片檢查
- [ ] 所有圖片都有描述性的 alt text
- [ ] 圖片檔案已優化（檔案大小合理）
- [ ] 圖片來源合法（自製或有授權）

### 內容檢查
- [ ] 標題層級正確（H2 → H3 → H4）
- [ ] 使用 `<!--more-->` 設定摘要
- [ ] 至少 2-3 個內部連結
- [ ] 外部連結使用合適的 rel 屬性
- [ ] 程式碼區塊有指定語言

### 技術檢查
- [ ] 文章可正常生成（`hexo generate` 無錯誤）
- [ ] 本地預覽無問題（`hexo server`）
- [ ] Front matter 格式正確（YAML）
- [ ] 特殊字元正確轉義

## 🎯 關鍵字研究建議

### 使用工具
1. **Google Search Console**: 查看哪些關鍵字帶來流量
2. **Google Trends**: 了解關鍵字趨勢
3. **相關搜尋**: Google 搜尋結果底部的「相關搜尋」
4. **競爭對手分析**: 查看類似主題的熱門文章使用什麼關鍵字

### 長尾關鍵字策略

**範例**:
- ❌ 太廣泛: "Kubernetes"（競爭激烈）
- ✅ 更具體: "MicroK8s Ubuntu 22.04 安裝教學"（更容易排名）

長尾關鍵字通常：
- 3-5 個詞組成
- 更具體、更明確的搜尋意圖
- 競爭較小
- 轉換率更高（讀者更可能找到他們真正需要的內容）

## 📈 效果追蹤

### 建議追蹤指標
1. **Google Analytics**: 頁面瀏覽量、停留時間、跳出率
2. **Google Search Console**: 搜尋曝光、點擊率、平均排名
3. **社群分享**: Twitter、Facebook 分享次數
4. **RSS 訂閱**: 訂閱者數量

### 定期檢查
- 每月檢查 Top 10 流量文章
- 識別表現良好的關鍵字
- 更新過時的內容
- 修復失效的連結

## 🔄 內容更新策略

### 何時更新舊文章
1. 技術或資訊過時
2. 有新的見解或經驗
3. 發現錯誤或遺漏
4. SEO 表現不佳但主題仍相關

### 更新最佳實踐
- 更新 `updated` 日期
- 在文章開頭註明更新內容
- 保持 URL 不變（使用 abbrlink）
- 重新提交 sitemap 給搜尋引擎

---

## 📚 延伸閱讀

- [Google 搜尋中心 - SEO 入門指南](https://developers.google.com/search/docs/fundamentals/seo-starter-guide)
- [Hexo 官方文檔](https://hexo.io/docs/)
- [Schema.org 結構化資料](https://schema.org/)
- [Web Content Accessibility Guidelines (WCAG)](https://www.w3.org/WAI/WCAG21/quickref/)

---

**記住**: SEO 是長期累積的過程，持續產出優質內容比任何技巧都重要！

# 快速使用 Hexo 搭配 Google Analytics

- source: `raw/setting-hexo-ga.md`
- original title: 快速使用 Hexo 搭配 Google Analytics (GA)
- author: Walle
- published: 2024-08-14
- source_created: 2024-08-14
- ingested_at: 2026-08-03
- type: migrated blog article summary

## Summary

這篇文章記錄如何為 Hexo/NexT Blog 加上 Google Analytics，從 GA 帳號與 tracking ID，到 NexT 設定、`google-analytics.njk`、GitHub Actions build 與資料回傳確認。

## Key Points

- GA 可用來觀察訪客來源、頁面瀏覽與互動行為。
- 原文以 NexT 的 `google_analytics` 設定與自訂 gtag 模板完成追蹤。
- GitHub Actions 必須在安裝相依套件後、build 前完成必要檔案與設定。
- 本頁只保存歷史 Hexo 設定脈絡；目前 Site 的 GA4 設定應以目前部署程式碼為準。

## Related Concepts

- [blog-knowledge-migration](../concepts/blog-knowledge-migration.md)

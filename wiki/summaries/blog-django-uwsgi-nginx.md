# 使用 Docker 部署 Django：uWSGI 與 Nginx

- source: `raw/django-uwsgi-nginx.md`
- original title: 使用 Docker 部署 Django：結合 uWSGI 和 Nginx 提升效能
- author: Walle
- published: 2024-08-18
- source_created: 2024-08-18
- ingested_at: 2026-08-03
- type: migrated blog article summary

## Summary

這篇文章說明為何不直接使用 Django development server，而以 Nginx 處理反向代理與靜態檔案，再以 uWSGI 作為 Nginx 與 Django 之間的橋樑，並示範放入 Docker container 的部署方式。

## Key Points

- Nginx 負責反向代理、靜態檔案與請求分流，避免讓 Django 開發伺服器承擔正式流量。
- uWSGI 負責連接 Django 與 Nginx，讓 Python 應用以較適合部署的方式運行。
- 原文以 Dockerfile、entrypoint.sh 與 nginx.conf 組成啟動流程，包含 migrate、collectstatic 與服務啟動。

## Related Concepts

- [blog-knowledge-migration](../concepts/blog-knowledge-migration.md)

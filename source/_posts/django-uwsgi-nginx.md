---
title: 使用 Docker 部署 Django：結合 uWSGI 和 Nginx 提升效能
date: 2024-08-18 16:37:49
tags: [Django , container]
---

## 前情提要

在部署 Django 專案時，雖然可以使用 `python manage.py runserver` 指令來啟動伺服器，但這樣的做法在處理靜態檔案時效能不佳，尤其是在面對大量訪問時，效能瓶頸會更加明顯。實務上，我們常使用反向代理（reverse proxy）來提升效能，並避免直接使用 Django 的開發伺服器。Nginx 作為反向代理，不僅可以處理靜態檔案，還能分流請求，提升應用的安全性與效能。

此外，與 .NET Core 可以透過編譯並打包成單一可部署檔案的方式不同，在 Python 環境下，我們需要借助像 uWSGI 這類的工具，作為 Nginx 與 Django 之間的橋樑，以實現高效的應用程式部署。

<!--more-->

## 開始實踐

由於使用 container，我們的目標是將 Nginx 和 Django 專案放到同一個 container 中。

### 1. 撰寫 Dockerfile

首先，撰寫 `Dockerfile`。在這裡，我們會準備 `nginx.conf` 和一個 shell 腳本，用於產生靜態檔案、資料庫遷移（migrate），並啟動 uWSGI 和 Nginx。

```dockerfile
FROM python:3.12-slim

ENV PYTHONPATH=/usr/local/lib/python3.12/site-packages

WORKDIR /app

COPY requirements.txt /app/
RUN pip install --no-cache-dir -r requirements.txt

COPY . /app/

RUN apt-get update && \
    apt-get install -y nginx && \
    apt-get install -y uwsgi-plugin-python3 && \
    rm -rf /var/lib/apt/lists/*

COPY nginx.conf /etc/nginx/sites-enabled/nginx

COPY uwsgi.ini /app/

EXPOSE 80

COPY entrypoint.sh /app/
RUN chmod +x /app/entrypoint.sh
CMD ["/app/entrypoint.sh"]
```

### 2. 撰寫 entrypoint.sh

entrypoint.sh 用來執行資料庫遷移和靜態檔案產出，然後啟動 uWSGI 和 Nginx：

```shell
#!/bin/sh

python core/manage.py collectstatic --noinput
python core/manage.py migrate

uwsgi --ini uwsgi.ini &

nginx -g 'daemon off;'
```

### 3. 撰寫 nginx.conf

```text
server {
    listen 80;
    server_name 127.0.0.1;
    charset utf-8;

    client_max_body_size 75M;  

    location / {
        include uwsgi_params;
        uwsgi_pass 127.0.0.1:8000;
    }

    location /static/ {
        alias /app/core/staticfiles/;
    }
}
```

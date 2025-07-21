---
title: 快速使用 Hexo 搭配 Google Analytics (GA)
tags:
  - Hexo
  - Web Development
  - Analytics
  - Static Site
categories: Web Development
abbrlink: e14c
date: 2024-08-14 00:27:38
---

## Google Analytics 簡介

Google Analytics (GA) 網路分析工具，主要用於追蹤和報告網站流量。透過詳細的資料和洞察力，GA 幫助網站擁有者優化網站性能和使用者體驗。以下是其主要功能：

1. 訪客追蹤：追蹤訪客來源、在站點的停留時間及瀏覽頁面。
2. 流量來源分析：報告顯示訪客來自何處，包括搜尋引擎、社交媒體平台或其他外部連結。
3. 行為洞察：分析訪客的互動行為，如點擊率、離開率和轉化率。

先前文章有先簡介過 Hexo 那們我們加上 GA 的功能可以使我們的 GitHub Pages 上有可以分析訪客是從哪邊來的，讓我們可以更輕易的去調整我們的網頁，BTW 如果沒有看過先前文章可以先看[參考連結](https://walle45611.github.io/2024/08/12/Setting-Hoex/#more)。

<!--more-->

## 設定 Hexo

1. 設定 Google Analytics (GA) 帳號，此部步驟可以[參考網址](https://support.google.com/analytics/answer/1009692?hl=zh-Hant)

2. 以下圖可以確認自己的 `tracking_id` 在 GA 面板中。
    ![tracking_id](https://i.imgur.com/Qthw6fp.png)

3. 設定 `_config.next.yml` 設定自己的 `tracking_id` (評估 ID) 可以在文件中搜尋關鍵字 `google_analytics`，因為 NexT 使用舊版的 GA 格式所以還是使用 UA 開頭 [參考網址](https://theme-next.js.org/docs/third-party-services/statistics-and-analytics.html?highlight=google+an)。

   ```yml
   tracking_id: UA-XXXXXXXXXX
   ```

4. 複製此段 `GA` 連接程式到自己的專案根目錄底下 `google-analytics.njk`

    ```njk
    <!-- Google tag (gtag.js) -->
    <script async src="https://www.googletagmanager.com/gtag/js?id=G-G0PYR1QYT5"></script>
    <script>
    window.dataLayer = window.dataLayer || [];
    function gtag(){dataLayer.push(arguments);}
    gtag('js', new Date());

    gtag('config', 'G-G0PYR1QYT5');
    </script>
    ```

5. 在 `GitHub Action` 上加入這段指令，記得是在 `npm install` 後 build 專案之前加上

    ```yml
    - name: Replace google-analytics.njk
        run: cp ${{ github.workspace }}/google-analytics.njk ${{ github.workspace }}/node_modules/hexo-theme-next/layout/_third-party/analytics/google-analytics.njk
    ```

6. 確認自己的 GA 是否又成功到自己的 [GA](https://analytics.google.com/) 面板上看

    ![GA_Check](https://i.imgur.com/fVXv3Ae.png)

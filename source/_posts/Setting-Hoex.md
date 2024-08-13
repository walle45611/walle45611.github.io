---
title: 快入使用 Hexo 建置自己的 Blog
date: 2024-08-12 18:52:26
tags: hexo
categories: 
    - "hexo 設定"
---

## 關於 Hoex 的思考

我長時間以來一直在尋找適合我的文章編輯軟體，並嘗試了多種工具，包括：

- **Notion**：功能強大且多樣，但我感覺它更適合管理和協作，而非專注於純粹的寫作。
- **HackMD**：針對 Markdown 編輯的工具，非常適合協作，但在個人寫作方面稍顯不足。
- **Jekyll**：雖然強大且靈活，但需要一定的技術基礎，對於想要快速進行寫作的人來說，門檻較高。

近期，我發現了 **Hoex** 這款工具，它似乎更適合我目前的需求：

- **Hoex** 提供了一個簡潔且高效的寫作環境，非常適合專注於內容創作。相比之下，過去使用的 Jekyll 需要更多的配置和維護，而 Hoex 更加輕量且易於使用。
- **NPM 生態系統**：Hoex 的使用方式與 NPM 密切相關，這使得整個工具鏈非常現代且方便，尤其對於熟悉前端開發的人來說，能快速上手並整合其他工具。

總結來說，**Hoex** 為我提供了簡單而專注的寫作體驗，加上它與 NPM 的兼容性，使得整個寫作流程更加順暢和現代化。這讓我覺得比起以往的工具，Hoex 更加適合我現在的寫作需求。以下這篇文章將講解如何使用 Hexo 來建立自己的 Blog。

<!-- more -->

## 安裝

- 安裝 `hexo-cli` 可以節省很多的時間，使我們能夠更專注於寫作和思考文章的內容。[參考網址](https://hexo.io/docs/)

    ```bash
    npm install -g hexo-cli
    ```

## 開啟專案

- [指令參考網址](https://hexo.io/docs/commands)

- 初始化專案
  
    ```bash
    hexo init <folder>
    cd <folder>
    npm i 
    ```

- 初始化專案後會有以下專案預設的資料夾結構
  
    ```text
    .
    ├── _config.yml
    ├── package.json
    ├── scaffolds
    ├── source
    |   ├── _drafts
    |   └── _posts
    └── themes
    ```

  - 比較重要的路徑檔案有 `_config.yml`、`source/_posts`：
    - `_source/_posts` 是儲存已發佈文章的地方。
    - `_config.yml` [詳細官網參數設定](https://hexo.io/zh-cn/docs/configuration)：
      - 語言設定：在預設文件的第 11 行有 `language` 的選項，

        ```yml
        language:
            - zh-tw  
        ```

      - SEO 優化：在第 6-8 行左右設定 `description`、`subtitle`、`title`，

        ```yml
        title: Walle Blog
        subtitle: "Welcome to my blog!"
        description: "Walle's blog"
        ```

      - 時區設定：在第 13 行左右加入 `timezone` 選項來選擇時區
  
        ```yml
        timezone: Asia/Taipei
        ```

      - 網址設定：由於我將專案放置於 `GitHub Pages` 上，因此需要在第 17 行設置 `url`，

        ```yml
        url: https://walle45611.github.io
        ```

## 使用 Next Theme

- 使用 `Next` 這個主題可以讓我們的網站更具有現代感和自訂性。首先，我們需要將 `Next` 的 `_config.yml` 檔案複製到專案的根目錄，

    ```bash
    cp node_modules/hexo-theme-next/_config.yml _config.next.yml
    ```

- 複製完成後，我們可以開始設定有關於 `Next` 主題的相關屬性：
  - `scheme: Gemini` : 一開始我想說為什麼別人的文章與文章的分隔都很好看，我才發現 scheme 可以更改...
  - `excerpt_description: true`：如果文章太長，可以在文章的適當位置加上 `<!--more-->`，這樣就會從該處截斷文章顯示在首頁，避免因為文章過長而導致首頁只有一篇文章的情況。

## 架上 robots.txt 防止爬蟲

- 以下是 robots.txt 檔案

  ```text
    User-agent: *
    Allow: /
    Allow: /archives/
    Allow: /categories/
    Allow: /tags/ 
    Disallow: /vendors/
    Disallow: /fonts/
    Disallow: /fancybox/
    Sitemap: https://walle45611.github.io/sitemap.xml
  ```

## 開發

- 開啟新文章使用預設的命令：

    ```bash
    hexo new <layout> <your post name>
    ```

- 開發過程中可以使用以下指令開啟暫時的預覽頁面（但 Hexo 沒有 hot reload 的功能）：

    ```bash
    hexo server
    ```

## 部署專案

- 創建 `github pages`，需要在 github 上創建一個 repo 名稱為 `<username>.github.io`。

- 部署可以分成兩種部署方式，使用 `hexo deploy` 部署的方式或使用 `github action`，我更傾向使用 `github action`，因為不知道 `github pages` 是使用什麼平台部署的，所以為了不要造成部署的有平台不同的問題，選擇在 `github action` 上 build 專案。

- 使用 `hexo deploy` 工具部署專案。第一次會先詢問 GitHub 的使用者名稱和帳號，如果是 Mac 使用者，請記得先在 GitHub 申請 OAuth Token：

    ```bash
    hexo clean && hexo deploy
    ```

  - 安裝 `hexo-deployer-git`：

    ```bash
    npm install hexo-deployer-git --save
    ```

    - 設定 `_config.yml`：

    ```yml
    deploy:
        type: git
        repo: https://github.com/walle45611/walle45611.github.io
        branch: main
    ```

- 使用 `GitHub Action` 請先將 repo pages 的設定調整 `source` 選項到 `Github Action` 如下圖：

  ![GithubAction](https://i.imgur.com/2gYt4Pv.png)

- 使用 `github action` 工具部署，記得是要把整個專案上傳到 repo 中，再使用 `github action`，可以使用 `github` 的頁面設定，這樣就可以使用 `github pages`，以下是 `github action` 的設定 yml： [參考連結](https://hexo.io/docs/github-pages#Useful-links)
  
    ```yml
    name: Pages

    on:
        push:
            branches:
            - main # default branch

    jobs:
    build:
        runs-on: ubuntu-latest
        steps:
        - uses: actions/checkout@v4
            with:
            token: ${{ secrets.GITHUB_TOKEN }}
            # If your repository depends on submodule, please see: https://github.com/actions/checkout
            submodules: recursive
        - name: Use Node.js 20
            uses: actions/setup-node@v4
            with:
            # Examples: 20, 18.19, >=16.20.2, lts/Iron, lts/Hydrogen, *, latest, current, node
            # Ref: https://github.com/actions/setup-node#supported-version-syntax
            node-version: "20"
        - name: Cache NPM dependencies
            uses: actions/cache@v4
            with:
            path: node_modules
            key: ${{ runner.OS }}-npm-cache
            restore-keys: |
                ${{ runner.OS }}-npm-cache
        - name: Install Dependencies
            run: npm install
        - name: Build
            run: npm run build
        - name: Upload Pages artifact
            uses: actions/upload-pages-artifact@v3
            with:
            path: ./public
    deploy:
        needs: build
        permissions:
        pages: write
        id-token: write
        environment:
        name: github-pages
        url: ${{ steps.deployment.outputs.page_url }}
        runs-on: ubuntu-latest
        steps:
        - name: Deploy to GitHub Pages
            id: deployment
            uses: actions/deploy-pages@v4
    ```

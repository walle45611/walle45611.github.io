---
title: LazyVim 設定過程筆記
abbrlink: 50b
date: 2025-01-26 15:38:15
tags: [Editor, Vim, Development Tools]
categories: Development Tools
---

## LazyVim 和 Oh My Tmux 設定過程

我嘗試了一些方式安裝 Neovim，大部分找到的都偏複雜，
後來我覺得直接用 LazyVim 安裝最快了，然後搭配 tmux 感覺就是工作效率翻倍，
但是我覺得還是要熟悉 Vim 才會有這樣的感覺，然後搭配著 Tmux 的功能讓頁面可以快速切換，
總體來說我覺得這樣的搭配也算好用。

<!--more-->

![圖 1. Blog Image](https://i.ibb.co/qWgZZCD/blog.webp)

---

## **1. 安裝 LazyVim**

LazyVim 是基於 Neovim 的一個優化配置框架，能夠簡化插件管理與配置。以下是安裝步驟：

### **安裝 Neovim 與基礎工具**

1. **安裝 Neovim**

    - macOS：

        ```bash
        brew install neovim
        ```

    - Ubuntu/Debian：

        ```bash
        sudo apt install neovim
        ```

2. **確認 Neovim 版本**（建議 0.8 以上）：

    ```bash
    nvim --version
    ```

3. **安裝必要工具**：

    - **Node.js**（用於補全與語法檢查）：

        - macOS：

            ```bash
            brew install node
            ```

        - Ubuntu/Debian：

            ```bash
            sudo apt install nodejs npm
            ```

    - **Ripgrep**：支援 Telescope 的模糊搜尋。

        ```bash
        brew install ripgrep
        ```

### **Clone LazyVim 配置**

1. 刪除舊有的 Neovim 配置：

    ```bash
    rm -rf ~/.config/nvim
    ```

2. Clone LazyVim：

    ```bash
    git clone https://github.com/LazyVim/starter ~/.config/nvim
    ```

3. 啟動 Neovim：

    ```bash
    nvim
    ```

    第一次啟動會自動安裝所有插件。

---

## **2. 安裝 Oh My Tmux**

[Oh My Tmux](https://github.com/gpakosz/.tmux) 是一款強大的 tmux 配置框架，能提升 tmux 使用體驗。

### **安裝步驟**

1. **Clone Oh My Tmux 倉庫**：

    ```bash
    git clone https://github.com/gpakosz/.tmux.git ~/.tmux
    ln -s -f ~/.tmux/.tmux.conf ~/
    cp ~/.tmux/.tmux.conf.local ~/
    ```

2. **自定義配置**（可選）：
   編輯 `~/.tmux.conf.local`，根據個人需求調整 tmux 配置。例如：

    ```bash
     tmux_conf_theme_left_separator_main='\uE0B0'
     tmux_conf_theme_left_separator_sub='\uE0B1'
     tmux_conf_theme_right_separator_main='\uE0B2'
     tmux_conf_theme_right_separator_sub='\uE0B3'
    ```

3. **啟動 tmux**：

    ```bash
    tmux
    ```

    若配置成功，你將看到 Oh My Tmux 的自定義界面。

---

## **3. 禁用與自定義插件**

在 `~/.config/nvim/lua/plugins/init.lua` 中，進行插件的禁用與管理。

### **禁用插件**

以下為禁用某些內建插件的範例：

```lua
return {
  -- 禁用文件樹插件
  { "nvim-neo-tree/neo-tree.nvim", enabled = false },
  { "nvim-treesitter-textobjects", enabled = false },
  { "nvim-ts-autotag", enabled = false },

  -- 禁用動畫效果插件
  { "folke/noice.nvim", enabled = false },
  { "folke/flash.nvim", enabled = false },

  -- 禁用 Bufferline
  { "akinsho/bufferline.nvim", enabled = false },
  { "echasnovski/mini.animate", enabled = false },
}
```

---

## **4. 自定義選項**

在 `~/.config/nvim/lua/config/options.lua` 中，可添加額外的選項，這邊加上這行的原因是因為關閉動畫：

```lua
-- Options are automatically loaded before lazy.nvim startup
-- Default options that are always set: https://github.com/LazyVim/LazyVim/blob/main/lua/lazyvim/config/options.lua
-- Add any additional options here
vim.g.snacks_animate = false
```

---

## **5. 測試與驗證**

![圖 2. 設定完成的圖片:](https://imgur.com/GdwfzgR.png)

1. 啟動 Neovim，執行：

    ```vim
    :Lazy sync
    ```

2. 測試功能：

    - 確認禁用插件是否生效，例如 `neo-tree` 不應再加載。
    - 測試語法高亮與模糊搜尋是否正常運行。

3. 啟動 tmux，確認 Oh My Tmux 配置是否生效。

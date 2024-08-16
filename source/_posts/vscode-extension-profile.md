---
title: 利用 vscode profile 分開不同開發環境的 vscode extension 
date: 2024-08-16 13:50:59
tags:
---

## 前情提要

* 因為有個困擾就是，我常常因為要切不同框架撰寫前後端，所以有時候用的 vscode extension 都會不同，這個問提困擾我很舊，因為每次打開 vscode 就很久，直到有天我發現可以使用 vscode profile 去切割不同的開發環境所需要的 extension，從此我的 vscode 開啟速比以前為管理前快多了...。

* [官方文件](https://code.visualstudio.com/docs/editor/profiles)

* 我的習慣是我 `Default` profile 會用來作為其他 profile 的 base 所以我會裝一些常用的工具在 `Default` profile 這樣就可以複製到新的 profile 上然後繼續裝上自己需要使用的 extension。

<!--more-->

## profile 設定

* 以下以 mac 為例
* 設定位置
  ![vscode-extension-1](https://i.imgur.com/ZZE6ntN.png)
  * 以下會有幾個選項，可以在這邊切換自己的 profile，也可以創建自己的 profile，或是刪除
  * `New Profile`，這邊我會選擇 `copy from` 選 `default`，或是如果想要創自己的 base 都可以自己做選擇
    ![vscode-extension-2](https://imgur.com/xhaB7ll.png)
  * `Delete Profile`，可以選擇自己想要刪除的 profile
    ![vscode-extension-2](https://imgur.com/ndLKodT.png)

## 結論

大概就這樣而已了，這篇文章是為了要告訴不知道 profile 去管理自己的 vscode extension 的朋友們，因為有時候看到大家的 vscode 開啟都需要很久的時間，所以想說寫個文章讓大家知道這個小技巧
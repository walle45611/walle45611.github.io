---
title: Windows 10 L2TP VPN 連線錯誤
date: 2024-08-30 13:47:12
tags: [VPN, Windows]
categories: IT 技術
---

![圖 1. : Blog Image](https://imgur.com/TpAWGlJ.png)

## 前情提要

今天在補習班的時候，突然有同事說 VPN 連線失敗，我想說慘了是不是真的掛掉了 Radius 還是 Cisco Router 上的設定有問題，然後我試了一下我的帳號可以，那麼我就開始排除錯誤，還好問題不大。

<!--more-->

## WAN mini port definition

A WAN mini port is a software driver for Windows XP (and later versions) that allows devices to connect to wide area networks (WANs). Crucially, WAN mini ports allow connections to the internet without using a router as an intermediary — for example, by plugging the ethernet cable directly into your computer.

WAN mini ports are typically installed automatically when a computer is connected to a WAN, such as through a DSL or cable modem. WAN mini ports are also automatically installed when you connect to a virtual private network (VPN). Each WAN mini port can be configured separately using the Windows Device Manager.

* 簡單總結一下這個其實就是說你需要連接到 WAN windows 呢他會幫你安裝 WAN Miniport Driver，或是連接到 VPN 他也會自動安裝此 Driver。

## 開始除錯

![圖 2. : Windows VPN 連接錯誤](https://imgur.com/44XrYRw.png)

1. 我一定先確定對方的電腦可不可以 ping 到公司的對外連 ip。
2. 開始交叉測試帳號密碼是不是可以，為了測試 Radius 是不是有問題。
3. 我問了這個發生錯誤電腦的主人說這個錯誤訊息是為什麼發生的他說他**重置**了他的**網路介面卡**
4. 我開始查詢 Windows Events log 我想說走到這步了，我看看是什麼 Error Code 說不定會遇到以前遇過的錯誤訊息，然後就出現了 720

    ![圖 3. : Windows Events log](https://imgur.com/OTXieNt.png)

5. 然後我就突然想到某個夜晚的除錯，哈哈哈，簡單來說就是 Windows TCP/IP 出現故障所以呢需要把這個 WAN Miniport(IP) 解除安裝此 Driver 再重開機，重新讓 Windows 安裝。
   1. control -> 裝置和印表機 -> 裝置管理員

        ![圖 4. : Driver 示意圖](https://imgur.com/fq6lEsr.png)

## 參考

* [WAN mini port definition](https://nordvpn.com/zh-tw/cybersecurity/glossary/wan-mini-port/)

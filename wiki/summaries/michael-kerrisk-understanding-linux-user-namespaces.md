# Understanding Linux User Namespaces

- source: `raw/web-clipper/Michael Kerrisk  Understanding Linux user namespaces.md`
- source link: https://www.youtube.com/watch?v=XgThPoL9mPE
- original title: Michael Kerrisk :: Understanding Linux user namespaces
- author: Michael Kerrisk（講者）；CoreCppIL（影片頻道）
- published: 2023-09-04
- source_created: 2026-09-23
- ingested_at: 2026-09-25
- type: Core C++ 2023 演講逐字稿摘要

## Summary

演講用命令列實驗解釋 Linux user namespace 如何讓程序在 namespace 內擁有 UID 0 與 capabilities，同時在外部仍對應一般使用者。關鍵限制是權限檢查發生在**管理目標資源的 user namespace**：在新 user namespace 內是 root，並不等於能管理初始 namespace 的主機資源。

## Key Points

1. **一般 namespace（1:11–11:41）**：UTS、mount、network 等 namespace 隔離不同種類的系統資源。每個程序各屬於每種類型的一個 namespace；`/proc/<pid>/ns/` 的符號連結可協助判斷兩個程序是否位於相同 instance。演講以 `unshare` 建立、`nsenter` 進入 UTS namespace，觀察兩個 shell 的 hostname 與 namespace 識別值。
2. **Capabilities（12:21–16:06）**：相較於傳統 setuid-root 程式取得整套 root 權限，capabilities 將特權拆成多種權能。演講當時以 41 種 capabilities 說明；這是演講時間點的數量，不宜當成永久不變的規格。
3. **UID/GID 映射（16:21–24:58）**：user namespace 有父子階層。透過 `/proc/<pid>/uid_map` 與 `gid_map`，可將 namespace 內 UID/GID 0 映射到外部 UID/GID 1000。示範程序在內部顯示 0，在初始 user namespace 觀察仍是 1000；映射項包含內部起點、外部起點與範圍長度。
4. **權限邊界（25:25–36:16）**：新 user namespace 內的 capabilities 只對其權限範圍內的資源有效。單獨建立 user namespace，不能因此修改初始 UTS namespace 的 hostname；同時建立由新 user namespace 擁有的 UTS namespace 後，才可在其中修改 hostname。若程序仍在初始 network namespace，就不能靠新 user namespace 的 `CAP_NET_ADMIN` 修改那裡的介面。
5. **用途（40:04–43:30）**：講者以無特權容器、瀏覽器 renderer sandbox、Firejail，以及 Flatpak／Snap 等作為應用例；也指出 sandbox 通常結合其他隔離機制，user namespace 本身並非完整容器或安全政策。

## Evidence and Limits

- 素材含影片簡介與約 51 分鐘的自動逐字稿，命令與專有名詞有明顯轉寫錯誤。因此保留實驗的觀察與原理，不把逐字稿中的模糊命令整理成可直接複製執行的腳本。
- 講者明說 UID/GID map 的寫入規則涉及許多安全條件，演講未完整介紹；本摘要不補寫那些條件。實際可否建立無特權 user namespace，也需依目標 Linux 主機設定確認。
- 影片於 2023 年發布；capabilities 數量、應用程式採用方式與 Linux 發行版預設值均不據此推定為現在的狀態。

## Alignment and Related Concepts

補強既有容器網路筆記：network namespace 說明網路資源如何隔離，本演講則說明建立與管理 namespace 時的身分、映射與權限範圍。

- [linux-user-namespaces](../concepts/linux-user-namespaces.md)
- [container-networking](../concepts/container-networking.md)

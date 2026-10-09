# Containerlab：以程式碼管理網路實驗

- source: `raw/web-clipper/Containerlab - a Modern way to Deploy Networking Topologies for Labs, CI and Testing - Roman Dodin.md`
- source_sha256: `d7af134e916d7e7fe2a53ac21c852125d5f689355d0180a59bd721054641b79c`
- source link: https://www.youtube.com/watch?v=snQTlFahY1c
- published: 2021-11-12
- source_created: 2026-10-07
- ingested_at: 2026-10-09
- type: source summary

## 摘要與關鍵資訊

Roman Dodin 的 2021 年演講以 GoBGP、Arista cEOS 與 Nokia SR Linux 的三節點 BGP route reflection 示範，把實驗拓樸、映像與初始設定整理成可版本控制及 CI 重建的輸入。

- YAML 描述 lab name、nodes、kind、image 與 links 的 endpoints；Containerlab 負責建立節點與連線。
- 透過 deploy／inspect 管理實驗，使用 SSH 或 docker exec 進入節點；網路 OS 的程式化介面可供外部自動化工具使用。
- save 將支援節點的 running configuration 存為 startup configuration，再從 lab directory 取出並放回 repository；startup-config 或 binds 可把配置帶入下一次部署。
- 容器節點較輕量，映像可透過 registry 版本化；VM 型網路 OS 也有支援，但不能把所有節點都當成原生容器。
- 映像取得、授權與節點種類仍有差異；宣告拓樸不會自動消除這些前提。

## 來源邊界

逐字稿有名稱辨識錯誤。支援廠商數、作業系統數、UI 與安裝方式均為 2021 年演講當時描述，不視為現行產品規格。本來源補充 labs-as-code 的設計理由；Apple Silicon／UTM 操作見既有 [[my-vault-tech-containerlab-4e538a1cc]] 與 [[fortigate-cisco-iol-utm-ospf-lab]]。

## 相關概念

- [[container-networking]]
- [[network-protocol-layers]]

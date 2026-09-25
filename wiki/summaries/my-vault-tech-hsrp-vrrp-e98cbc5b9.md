# HSRP (Hot Standby Routing Protocol)

- source: `raw/my-vault/Note/Tech/HSRP 與 VRRP 熱備份路由協定指南.md`
- source_sha256: `0fc334d8a89e1eedcfd12c3717b8b704af5e8f2d5fdba2af273d2eab1f0afa99`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 HSRP概述、HSRP vlan 關係、HSRP 選舉、狀態設定priority、HSRP 狀態 和 timer、重新選舉。

## Source Notes

- HSRP是一種Cisco的專屬協定，他讓多台路由使用一個IP Gateway。RFC2281
- 基本上，為了使用相同的閘道IP，都會被分配到同一個HSPR Grouop，其中一台是active HSRP router，另一台做standby HSRP router，其他路由器處於聆聽的狀態，HSRP 會定期發送hello封包以便知道active路由的存在
- 可以分配0~255任何的群組編號給HSRP Group，==如果再幾個VLAN介面上，設定HSRP Group，可以將群組編號設定為VLAN相同。==大多數只支持16個群組編號，所以有了另一種做法，讓妹個VLAN介面的群組編號都是相同(即1)，==因為HSRP群組只有在本身介面上有意義，完全不用擔心編號用完，也就是說vlan 11的HSRP 1跟vlan 10 HSRP 1是不一樣的==

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)

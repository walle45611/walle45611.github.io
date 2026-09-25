# RADIUS (Remote Authentication Dial In User Service)

- source: `raw/my-vault/Note/Research/RADIUS (Remote Authentication Dial In User Service).md`
- source_sha256: `d986ca145e5cccce68f2d26adba7a2225c423c7c38c59c0551bf78cae0d7d00a`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 組成元件、可以使用驗證的Protocol。

## Source Notes

- RADIUS定義於IETF中的RFC2865，允許network Authtication server(NAS)對用戶進行驗證、授權、計費(AAA)。RADIUS server具體來說是透過LDAP進行驗證，RADIUS可以集中放置從儲存在LDAP 伺服器中，並由RADIUS server進行驗證用戶資訊，而減少管理的開銷，有可以使用過程安全更簡單。
- RADIUS是一種Client/Server的行定。RADIUS的client通常是，Router，Switch或是無線設備，如果NAS收到用戶的請求，就會傳送到指定的RADIUS，並將用戶的設定資訊傳給NAS，然後NAS接受或拒絕。
- AAA，在RADIUS中驗證和授權是結合在一起的。如果發現用戶名，且密碼正確，RADUIS會回傳一個Access-Accept的回應，其中包括一些參數，以暴政對該用戶的存取。這些參數是在RADIUD中的設定，包括access type、protocol type、用戶指定該戶的IP address及u6ek7ACL會要NAS上應用的static route，另外還有一些數值

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

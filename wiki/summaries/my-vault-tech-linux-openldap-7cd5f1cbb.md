# Linux OpenLDAP 設定指南

- source: `raw/my-vault/Note/Tech/Linux OpenLDAP 設定指南.md`
- source_sha256: `1fe3c85ce8b45b3ee790e1decc63b55932577a7de94d29b95385a1315ffe7430`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 設定流程、設定FQDN、安裝、LDAP基礎設定、更改密碼、顯示基本的訊息。

## Source Notes

- 如果確定要架設 LDAP 作為你的身份驗證來源，那麼你得要事先規劃好這部 LDAP 所提供的 baseDN、相關的管理者密碼、相關的帳號 UID 起始號碼、 相關的使用者家目錄 (最好不要跟系統預設的 /home 相同位置，否則容易造成本機帳號與網路帳號衝突的狀況)等等。至於一般的架設流程大概是這樣的
- 安裝LDAP，並提供登入LDAP功能的RootDN密碼
- 啟動 LDAP 服務，並觀察 LDAP 服務的埠口，以啟用防火牆放行的功能

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)

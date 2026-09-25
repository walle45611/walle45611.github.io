# LDAP (Light-weight Directory Access Protocol)

- source: `raw/my-vault/Note/Research/LDAP (Light-weight Directory Access Protocol).md`
- source_sha256: `535300947f612f1acab82e84f24bb48c29a3fa09aac1f3e23071f5fbfe5fe444`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 LDAP身世、LDAP優勢、LDAP 可以做甚麼、LDAP組件、資料結構、LDAP樹狀結構。

## Source Notes

- LDAP前輩是x.500 directory service，一個操作在應用層複雜的協定;因為會隨著載入的資料大小不同而導致封包大小不同。LDAP樣式多變，只有定義一個協定，所以輕量，也很乾淨
- LDAP是一個以訊息為基礎，採用主從式架構，定義於RFC2251協定，並是一個非同步的協定架構，意味著用戶端可以發乎多筆請求，且server不必要照順序的方式回答slave可以以警及程度回答slave
- 本身有主從架構(Master \Slave)，可以利用多個子系統來分散存取，主從模式下也可以備用

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

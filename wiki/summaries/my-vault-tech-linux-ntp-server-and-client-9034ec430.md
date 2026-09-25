# Linux NTP Server and Client 設定與原理說明

- source: `raw/my-vault/Note/Tech/Linux NTP Server and Client 設定與原理說明.md`
- source_sha256: `ba8ec98b2f9dd5c293241e417603b1896f4a90c1aaa29ba7e90ce21df2573903`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 🕰️ NTP Server（使用 chronyd）、🔧 安裝與啟用、🖋️ 設定 /etc/chrony/chrony.conf、✅ 驗證 Server 狀態、🧭 NTP Client 設定（使用 systemd-timesyncd）、編輯時間同步設定檔。

## Source Notes

- System clock synchronized 應為 yes
- 可透過 crontab 定期手動強制與 NTP Server 同步時間（非必要）

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)

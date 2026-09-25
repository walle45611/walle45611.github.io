---
Type:
  - windows Server DHCP
---
### DHCP server authorized

並不是就可以對dhcp client提供服務還需要經過授權，為經過授權的DHCP server無法將IP address出租給用戶端。

- ADDS domain services，DHCP才可以授權
- ADDS DHCP必須都要被授權
- 使用Enterprise Admins才可以有授權動作
- DHCP 啟動若透過ADDS查詢到IP位址已經在授權清單內的話，DHCP就可以正常啟用出租IP給用戶端
- 不是網域成員的DHCP獨立server無法被授權。此獨立的DHCP services會檢查是否有被授權的DHCP server，如果沒有就正常啟動，如果有就停止

### Powershell

```PowerShell
Install-WindowsFeature dhcp -IncludeManagementTools
Add-DhcpServerv4Scope -Name "internal" -StartRange 192.168.10.100 -EndRange 192.168.10.200 -SubnetMask 255.255.255.0
Set-DhcpServerv4OptionValue -Router 192.168.10.254 -DnsDomain "sayms.local" -DnsServer 192.168.10.1
```
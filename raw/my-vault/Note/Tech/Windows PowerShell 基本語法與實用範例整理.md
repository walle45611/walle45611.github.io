---
Type:
  - windows Server P2P
---
## ✅ 基本指令與常見功能

### 查看連接埠使用情況

```powershell
netstat -an | findstr "445"
```

### 建立檔案

```powershell
fsutil file createnew TestFileName 1000
```

### 列出目錄檔案與屬性

```powershell
ls | Format-Table Name,mode
```

### 查看檔案內容

```powershell
type .\test.txt
```

### 建立新檔案

```powershell
new-item -Name test.txt
```

### 操作環境變數

```powershell
$env:name="value"
ls env:name
del env:name
```

---

## 🔍 比較與邏輯運算子

```powershell
-eq -ne -lt -le -gt -ge
-not -and -or
-contains -notcontains
-like -notlike
-Ceq（區分大小寫）
```

```powershell
1 -eq 1         # True
1,2,3 -eq 1     # 1
"A" -Ceq "a"    # False
```

---

## 📘 指令別名速查

|簡寫|等效指令|
|---|---|
|`ls`|`Get-ChildItem`|
|`gp`|`Get-ItemProperty`|
|`gi`|`Get-Item`|
|`gm`|`Get-Member`|
|`gcm`|`Get-Command`|
|`gc`|`Get-Content`|
|`ni`|`New-Item`|
|`cd`|`Set-Location`|
|`foreach`|`ForEach-Object`|

---

## 🔎 檔案與服務操作

### 遞迴搜尋檔案

```powershell
ls -Path . -Recurse -Filter *python*
```

### 查詢服務

```powershell
Get-Service | select -Property ServiceName,Name
Get-Service | Where-Object {$_.name -like '*spoo*'}
```

### 啟動服務

```powershell
Get-Service | Where-Object {$_.name -like '*spoo*'} | Start-Service -PassThru
```

### 遠端執行命令

```powershell
Invoke-Command -ScriptBlock { Get-Service }
```

---

## 🔁 控制流程

### if-else 條件判斷

```powershell
$num = 100
if($num -gt 99) {1} else {0}
```

### switch 條件選擇

```powershell
$day = 3
switch ( $day ){
    3 { $result = 'Wednesday' }
}
```

### while 與 do while

```powershell
$num = 15
while($num -gt 10){ $num; $num-- }
```

```powershell
do { $num; $num-- } while($num -gt 10)
```

### for 迴圈

```powershell
$num = 0
for($i=1; $i -le 100; $i++){
    $num += $i
}
$num # 結果：5050
```

---

## 📚 陣列與函式

```powershell
$arr = 1, "hello"
$arr -is [array]  # True
```

### 自訂函式

```powershell
function myping { ping www.google.com }
myping
```

```powershell
function myping($site1) { ping $site1 }
myping www.google.com
```

---

## 🧪 環境與服務資訊查詢

```powershell
Get-PSProvider
Get-PSDrive
```

### 指令說明視窗

```powershell
help Get-Service -ShowWindow
```

### 查詢模組中的指令

```powershell
gcm -Module DnsServer
```

---

## 💡 應用實例

### 從 Windows 鎖屏背景複製圖片

```powershell
ls C:\Users\<user>\AppData\Local\Packages\Microsoft.Windows.ContentDeliveryManager_cw5n1h2txyewy\LocalState\Assets | foreach {
  $i=1
  $daytime=get-date -Format FiledateTime -replace "T","_"
  $today=Get-Date -Format "yyyy.MM.dd"
  $char="-"
  cp $_.FullName "C:\Users\$env:Username\Desktop\back\$daytime$char$i.jpg"
  $i++
} {get-date;"complete"}
```

### Hash 校驗

```powershell
Get-FileHash C:\Users\user1\Downloads\Contoso8_1_ENT.iso -Algorithm SHA384 | Format-List
```

### 清單查詢環境變數、處理程序

```powershell
ps | Where-Object {$_ -like "*vm*"}
```

```powershell
ps | foreach { if($_ -like "*vm*") { $_.name } }
```

---

## 🧹 系統管理

### 同步網路時間

```powershell
net time \\dc1 /set
```

### Appx Package 操作

```powershell
Get-appxpackage -alluser
Remove-appxpackage -allusers -package ""
```

### 使用者輸入

```powershell
$input = Read-Host "請輸入姓名"
"您好，您的姓名是:$input"
```

---

這份筆記涵蓋了 PowerShell 常用語法、服務控制、條件邏輯、遠端執行與日常管理工具，適合入門與日常維運使用。
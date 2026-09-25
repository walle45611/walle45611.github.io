## 1. 下載與安裝 EVE-NG & Windows 客戶端

### 🔗 EVE-NG 社群版 VM 下載

> [!info] Download  
> [https://www.eve-ng.net/index.php/download/#DL-COMM](https://www.eve-ng.net/index.php/download/#DL-COMM)

### 🔗 EVE-NG Windows Client 工具包

> [!info] Download  
> [https://www.eve-ng.net/index.php/download/#DL-WIN](https://www.eve-ng.net/index.php/download/#DL-WIN)

---

## 2. 匯入 IOS 映像並授權

### 放入 IOS

```bash
/opt/unetlab/addons/iol/bin/IOS.bin
```

### 修正權限

```bash
/opt/unetlab/wrappers/unl_wrapper -a fixpermissions
```

### IOU License 設定

```bash
cat /opt/unetlab/addons/iol/bin/iourc
[license]
unl01 = 972f30267ef51616;
```

---

## 3. 開啟瀏覽器即可使用

透過 IP 進入 Web GUI 操作平台。

---

## 4. 匯入 Windows 10 或 Windows Server 作業系統

### 🔗 官方教學

> [!info] Windows Workstation:  
> [https://www.eve-ng.net/index.php/documentation/howtos/howto-create-own-windows-host-on-the-eve/](https://www.eve-ng.net/index.php/documentation/howtos/howto-create-own-windows-host-on-the-eve/)

> [!info] Windows Server:  
> [https://www.eve-ng.net/index.php/documentation/howtos/howto-create-own-windows-server-on-the-eve/](https://www.eve-ng.net/index.php/documentation/howtos/howto-create-own-windows-server-on-the-eve/)

### 建立 Windows Server 映像流程

```bash
mkdir /opt/unetlab/addons/qemu/winserver-2012R2/
cd /opt/unetlab/addons/qemu/winserver-2012R2/
scp windows.iso root@192.168.0.106:/opt/unetlab/addons/qemu/winserver-2012R2/
mv windows.iso cdrom.iso
qemu-img create -f qcow2 virtioa.qcow2 100G
```

- 建立節點 → 安裝 Windows → 無磁碟 → Load driver → 依序安裝
    

![[Assets/Note/Tech/EVE-NG 安裝與設定指南/01-建立 Windows Server 映像流程.png]]  
![[Assets/Note/Tech/EVE-NG 安裝與設定指南/02-建立 Windows Server 映像流程.png]]  
![[Assets/Note/Tech/EVE-NG 安裝與設定指南/03-建立 Windows Server 映像流程.png]]  
![[Assets/Note/Tech/EVE-NG 安裝與設定指南/04-建立 Windows Server 映像流程.png]]

### 匯出 sysprep 完成的映像

![[Assets/Note/Tech/EVE-NG 安裝與設定指南/05-匯出 sysprep 完成的映像.png]]  
![[Assets/Note/Tech/EVE-NG 安裝與設定指南/06-匯出 sysprep 完成的映像.png]]  
![[Assets/Note/Tech/EVE-NG 安裝與設定指南/07-匯出 sysprep 完成的映像.png]]

### 替換完成安裝的映像

```bash
cd /opt/unetlab/tmp/[user id]/[lab id]/[node id]
qemu-img commit virtioa.qcow2
```

### 刪除 ISO

```bash
cd /opt/unetlab/addons/qemu/winserver-2012R2/
rm -f cdrom.iso
```

---

## 5. 設定 MobaXterm 支援 telnet 開啟連線（選用）

將以下內容儲存成 `start_mobaxterm.bat`，放入 `C:\Program Files\EVE-NG`：

```bat
@echo off
SET input=%1
FOR /f "tokens=1,2,3 delims=:" %%a IN ("%input%") do SET host=%%b&SET port=%%c
SET host=%host:~2%
echo %host%
echo %port%
cd C:\Program Files (x86)\Mobatek\MobaXterm
MobaXterm.exe -newtab "telnet -r %host% %port%"
```

### 登錄檔（註冊 telnet protocol 給 mobaxterm）

儲存為 `.reg`，雙擊匯入：

```reg
Windows Registry Editor Version 5.00

[HKEY_CURRENT_USER\SOFTWARE\Classes\MobaXterm.telnet]
@="telnet"

[HKEY_CURRENT_USER\SOFTWARE\Classes\MobaXterm.telnet\DefaultIcon]
@="C:\\Program Files (x86)\\Mobatek\\MobaXterm\\MobaXterm.exe, 0"

[HKEY_CURRENT_USER\SOFTWARE\Classes\MobaXterm.telnet\shell\open\command]
@="\"C:\\Program Files\\EVE-NG\\start_mobaxterm.bat\" %1"

[HKEY_CURRENT_USER\SOFTWARE\MobaXterm\Capabilities\URLAssociations]
"telnet"="MobaXterm.telnet"

[HKEY_CURRENT_USER\SOFTWARE\RegisteredApplications]
"MobaXterm"="Software\\MobaXterm\\Capabilities"

[HKEY_CURRENT_USER\SOFTWARE\Classes\telnet\shell\open\command]
@="\"C:\\Program Files\\EVE-NG\\start_mobaxterm.bat\" %1"

[HKEY_CLASSES_ROOT\telnet\shell\open\command]
@="\"C:\\Program Files\\EVE-NG\\start_mobaxterm.bat\" %1"
```

---

## 6. 常見問題排除

### Wireshark 抓包錯誤：`end of file on pipe magic during open`

→ 解法：透過 Putty 登入 EVE-NG 機器。

### SSH host key 錯誤

```bash
cd "C:\Program Files\EVE-NG"
echo y | .plink.exe -ssh -l username -pw password ipaddr
```

---

## 7. 實用資源

### IOS 映像來源

> [!info] dl.nextadmin.net - IOL bin 映像  
> [http://dl.nextadmin.net/dl/EVE-NG-image/iol/bin/](http://dl.nextadmin.net/dl/EVE-NG-image/iol/bin/)

### 參考連結

- [Cisco vIOS from VIRL 官方文件](https://www.eve-ng.net/index.php/documentation/howtos/howto-add-cisco-vios-from-virl/)
    
- [YouTube 教學影片 1](https://www.youtube.com/watch?v=uy-lXKVD1yY&ab_channel=DynamicCoder)
    
- [YouTube 教學影片 2](https://www.youtube.com/watch?v=Id6HseiylU0&ab_channel=TechnicalEkjata)
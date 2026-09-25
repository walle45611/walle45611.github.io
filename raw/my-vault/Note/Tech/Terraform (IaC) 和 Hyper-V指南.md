---
Created: 2023-06-02T16:03
tags:
  - DevOps
---
# Hyper-V 環境設定

## 安裝與設定

### 在 Hyper-V Server 上啟用 WinRM

```PowerShell
Enable-PSRemoting -SkipNetworkProfileCheck -Force

Set-WSManInstance WinRM/Config/WinRS -ValueSet @{MaxMemoryPerShellMB = 1024}
Set-WSManInstance WinRM/Config -ValueSet @{MaxTimeoutms=1800000}
Set-WSManInstance WinRM/Config/Client -ValueSet @{TrustedHosts="*"}
Set-WSManInstance WinRM/Config/Service/Auth -ValueSet @{Negotiate = $true}
```

### WinRM HTTPS 設定

```PowerShell
# Create CA certificate
$rootCaName = "DevRootCA"
$rootCaPassword = ConvertTo-SecureString "P@ssw0rd" -asplaintext -force 
$rootCaCertificate = Get-ChildItem cert:\LocalMachine\Root |?{$_.subject -eq "CN=$rootCaName"}
if (!$rootCaCertificate){
  Get-ChildItem cert:\LocalMachine\My |?{$_.subject -eq "CN=$rootCaName"} | remove-item -force
  if (Test-Path .\$rootCaName.cer) {
    remove-item .\$rootCaName.cer -force
  }
  if (Test-Path .\$rootCaName.pfx) {
    remove-item .\$rootCaName.pfx -force
  }
  $params = @{
    Type = 'Custom'
    DnsName = $rootCaName
    Subject = "CN=$rootCaName"
    KeyExportPolicy = 'Exportable'
    CertStoreLocation = 'Cert:\LocalMachine\My'
    KeyUsageProperty = 'All'
    KeyUsage = 'None'
    Provider = 'Microsoft Strong Cryptographic Provider'
    KeySpec = 'KeyExchange'
    KeyLength = 4096
    HashAlgorithm = 'SHA256'
    KeyAlgorithm = 'RSA'
    NotAfter = (Get-Date).AddYears(5)
  }
  $rootCaCertificate = New-SelfSignedCertificate @params

  Export-Certificate -Cert $rootCaCertificate -FilePath .\$rootCaName.cer -Verbose
  Export-PfxCertificate -Cert $rootCaCertificate -FilePath .\$rootCaName.pfx -Password $rootCaPassword -Verbose
  Get-ChildItem cert:\LocalMachine\My |?{$_.subject -eq "CN=$rootCaName"} | remove-item -force
  Import-PfxCertificate -FilePath .\$rootCaName.pfx -CertStoreLocation Cert:\LocalMachine\Root -password $rootCaPassword -Exportable -Verbose
  Import-PfxCertificate -FilePath .\$rootCaName.pfx -CertStoreLocation Cert:\LocalMachine\My -password $rootCaPassword -Exportable -Verbose
  $rootCaCertificate = Get-ChildItem cert:\LocalMachine\My |?{$_.subject -eq "CN=$rootCaName"}
}

# Create host certificate using CA
$hostName = [System.Net.Dns]::GetHostName()
$hostPassword = ConvertTo-SecureString "P@ssw0rd" -asplaintext -force
$hostCertificate = Get-ChildItem cert:\LocalMachine\My |?{$_.subject -eq "CN=$hostName"}
if (!$hostCertificate){
  if (Test-Path .\$hostName.cer) {
    remove-item .\$hostName.cer -force
  }
  if (Test-Path .\$hostName.pfx) {
    remove-item .\$hostName.pfx -force
  }
  $dnsNames = @($hostName, "localhost", "127.0.0.1") + [System.Net.Dns]::GetHostByName($env:computerName).AddressList.IpAddressToString
  
  $params = @{
    Type = 'Custom'
    DnsName = $dnsNames
    Subject = "CN=$hostName"
    KeyExportPolicy = 'Exportable'
    CertStoreLocation = 'Cert:\LocalMachine\My'
    KeyUsageProperty = 'All'
    KeyUsage = @('KeyEncipherment','DigitalSignature','NonRepudiation')
    TextExtension = @("2.5.29.37={text}1.3.6.1.5.5.7.3.1,1.3.6.1.5.5.7.3.2")
    Signer = $rootCaCertificate
    Provider = 'Microsoft Strong Cryptographic Provider'
    KeySpec = 'KeyExchange'
    KeyLength = 2048
    HashAlgorithm = 'SHA256'
    KeyAlgorithm = 'RSA'
    NotAfter = (Get-date).AddYears(2)
  }
  $hostCertificate = New-SelfSignedCertificate @params
  Export-Certificate -Cert $hostCertificate -FilePath .\$hostName.cer -Verbose
  Export-PfxCertificate -Cert $hostCertificate -FilePath .\$hostName.pfx -Password $hostPassword -Verbose
  Get-ChildItem cert:\LocalMachine\My |?{$_.subject -eq "CN=$hostName"} | remove-item -force
  Import-PfxCertificate -FilePath .\$hostName.pfx -CertStoreLocation Cert:\LocalMachine\My -password $hostPassword -Exportable -Verbose
  $hostCertificate = Get-ChildItem cert:\LocalMachine\My |?{$_.subject -eq "CN=$hostName"}
}

Get-ChildItem wsman:\localhost\Listener\ | Where-Object -Property Keys -eq 'Transport=HTTPS' | Remove-Item -Recurse
New-Item -Path WSMan:\localhost\Listener -Transport HTTPS -Address * -CertificateThumbPrint $($hostCertificate.Thumbprint) -Force -Verbose

Restart-Service WinRM -Verbose

New-NetFirewallRule -DisplayName "Windows Remote Management (HTTPS-In)" -Name "WinRMHTTPSIn" -Profile Any -LocalPort 5986 -Protocol TCP -Verbose
```

### WinRM HTTP 設定

```PowerShell
# Get the public networks
$PubNets = Get-NetConnectionProfile -NetworkCategory Public -ErrorAction SilentlyContinue 

# Set the profile to private
foreach ($PubNet in $PubNets) {
    Set-NetConnectionProfile -InterfaceIndex $PubNet.InterfaceIndex -NetworkCategory Private
}

# Configure winrm
Set-WSManInstance WinRM/Config/Service -ValueSet @{AllowUnencrypted = $true}

# Restore network categories
foreach ($PubNet in $PubNets) {
    Set-NetConnectionProfile -InterfaceIndex $PubNet.InterfaceIndex -NetworkCategory Public
}

Get-ChildItem wsman:\localhost\Listener\ | Where-Object -Property Keys -eq 'Transport=HTTP' | Remove-Item -Recurse
New-Item -Path WSMan:\localhost\Listener -Transport HTTP -Address * -Force -Verbose

Restart-Service WinRM -Verbose

New-NetFirewallRule -DisplayName "Windows Remote Management (HTTP-In)" -Name "WinRMHTTPIn" -Profile Any -LocalPort 5985 -Protocol TCP -Verbose
```

### Terraform 安裝

#### Windows

- 下載地址：https://releases.hashicorp.com/terraform/1.4.6/terraform_1.4.6_windows_amd64.zip

> [!info] Terraform 官方安裝指南 探索 Terraform 產品文檔、教程和範例
> 
> 參考連結：https://developer.hashicorp.com/terraform/downloads

#### macOS

```bash
brew tap hashicorp/tap
brew install hashicorp/tap/terraform
```

## 檔案結構

![[Assets/Note/Tech/Terraform (IaC) 和 Hyper-V指南/01-檔案結構.png]]

## 參數說明

### Provider 設定

以下內容都在 `main.tf` 中設定：

```hcl
terraform {
  required_providers {
    hyperv = {
      source = "taliesins/hyperv"
      version = "1.0.4"
    }
  }
}
```

### 連線基本設定

```hcl
provider "hyperv" {
  user            = "Walle"
  password        = "password"
  host            = "127.0.0.1"
  port            = 5986          # WinRM port
  https           = true
  insecure        = false
  use_ntlm        = true
  tls_server_name = ""
  cacert_path     = ""
  cert_path       = ""
  key_path        = ""
  script_path     = "C:/Temp/terraform_%RAND%.cmd"
  timeout         = "30s"
}
```

### 虛擬硬碟建立

```hcl
resource "hyperv_vhd" "web_server_g2_vhd" {
  path = "c:\\web_server\\web_server_g2.vhdx"  # 需要是絕對路徑
  size = 10737418240                           # 10GB (單位為 bytes)
}
```

### 虛擬機器設定

#### 基本配置

```hcl
resource "hyperv_machine_instance" "default" {
  name                                    = "Debian 11"
  generation                              = 2            # Hyper-V 版本：1 或 2
  memory_startup_bytes                    = 536870912    # Memory 大小
  notes                                   = ""           # 註解
  processor_count                         = 1            # 虛擬處理器數量
  static_memory                           = true         # 是否使用靜態記憶體
  state                                   = "Running"    # VM 狀態：Running 或 Off
  
  # 其他設定...
  automatic_critical_error_action         = "Pause"
  automatic_critical_error_action_timeout = 30
  automatic_start_action                  = "StartIfRunning"
  automatic_start_delay                   = 0
  automatic_stop_action                   = "Save"
  checkpoint_type                         = "Production"
  guest_controlled_cache_types            = false
  high_memory_mapped_io_space             = 536870912
  lock_on_disconnect                      = "Off"
  low_memory_mapped_io_space              = 134217728
  memory_maximum_bytes                    = 1099511627776
  memory_minimum_bytes                    = 536870912
  smart_paging_file_path                  = "C:\\ProgramData\\Microsoft\\Windows\\Hyper-V"
  snapshot_file_location                  = "C:\\ProgramData\\Microsoft\\Windows\\Hyper-V"
}
```

#### 韌體設定

```hcl
vm_firmware {
  enable_secure_boot = "Off"
  preferred_network_boot_protocol = "IPv4"
  console_mode                    = "None"
  pause_after_boot_failure        = "Off"

  boot_order {
    boot_type           = "DvdDrive"
    controller_number   = "0"
    controller_location = "1"
  }

  boot_order {
    boot_type           = "HardDiskDrive"
    controller_number   = "0"
    controller_location = "0"
  }
}
```

![[Assets/Note/Tech/Terraform (IaC) 和 Hyper-V指南/02-韌體設定.png]]

開機順序會照著程式碼的順序，影響開機的優先順序。

![[Assets/Note/Tech/Terraform (IaC) 和 Hyper-V指南/03-韌體設定.png]]

支援的 `boot_type`：

- `DvdDrive`
- `HardDiskDrive`
- `NetworkAdapter`

#### 網路介面卡設定

```hcl
network_adaptors {
  name        = "wan"                    # 網卡名稱
  switch_name = "default Switch"         # 連接的 vSwitch
  
  management_os                              = false
  is_legacy                                  = false
  dynamic_mac_address                        = true
  static_mac_address                         = ""
  mac_address_spoofing                       = "Off"
  dhcp_guard                                 = "Off"
  router_guard                               = "Off"
  port_mirroring                             = "None"
  ieee_priority_tag                          = "Off"
  vmq_weight                                 = 100
  iov_queue_pairs_requested                  = 1
  iov_interrupt_moderation                   = "Off"
  iov_weight                                 = 100
  ipsec_offload_maximum_security_association = 512
  maximum_bandwidth                          = 0
  minimum_bandwidth_absolute                 = 0
  minimum_bandwidth_weight                   = 0
  mandatory_feature_id                       = []
  resource_pool_name                         = ""
  test_replica_pool_name                     = ""
  test_replica_switch_name                   = ""
  virtual_subnet_id                          = 0
  allow_teaming                              = "On"
  not_monitored_in_cluster                   = false
  storm_limit                                = 0
  dynamic_ip_address_limit                   = 0
  device_naming                              = "Off"
  fix_speed_10g                              = "Off"
  packet_direct_num_procs                    = 0
  packet_direct_moderation_count             = 0
  packet_direct_moderation_interval          = 0
  vrss_enabled                               = true
  vmmq_enabled                               = false
  vmmq_queue_pairs                           = 16
}
```

#### DVD 光碟機設定

```hcl
dvd_drives {
  controller_number   = "0"
  controller_location = "1"
  path                = "c:/Users/Walle/Downloads/debian-11.7.0-amd64-netinst.iso"  # ISO 檔案位置
  resource_pool_name = ""
}
```

#### 硬碟設定

```hcl
hard_disk_drives {
  controller_type                 = "Scsi"
  controller_number               = "0"
  controller_location             = "0"
  path                            = hyperv_vhd.web_server_g2_vhd.path  # 引用建立的虛擬硬碟路徑
  disk_number                     = 4294967295
  resource_pool_name              = "Primordial"
  support_persistent_reservations = false
  maximum_iops                    = 0
  minimum_iops                    = 0
  qos_policy_id                   = "00000000-0000-0000-0000-000000000000"
  override_cache_attributes       = "Default"
}
```

#### 處理器設定

```hcl
vm_processor {
  compatibility_for_migration_enabled               = false
  compatibility_for_older_operating_systems_enabled = false
  hw_thread_count_per_core                          = 0
  maximum                                           = 100
  reserve                                           = 0
  relative_weight                                   = 100
  maximum_count_per_numa_node                       = 0
  maximum_count_per_numa_socket                     = 0
  enable_host_resource_protection                   = false
  expose_virtualization_extensions                  = false
}
```

![[Assets/Note/Tech/Terraform (IaC) 和 Hyper-V指南/04-處理器設定.png]]

#### 整合服務設定

```hcl
integration_services = {
  "Guest Service Interface" = false
  "Heartbeat"               = true
  "Key-Value Pair Exchange" = true
  "Shutdown"                = true
  "Time Synchronization"    = true
  "VSS"                     = true
}
```

![[Assets/Note/Tech/Terraform (IaC) 和 Hyper-V指南/05-整合服務設定.png]]

## Terraform CLI 操作步驟

### 初始化

```bash
terraform init
```

![[Assets/Note/Tech/Terraform (IaC) 和 Hyper-V指南/06-初始化.png]]

### 規劃

```bash
terraform plan
```

### 套用

```bash
terraform apply
```

![[Assets/Note/Tech/Terraform (IaC) 和 Hyper-V指南/07-套用.png]]

> [!important] 重要事項 使用前請使用 Administrator 權限的使用者執行

## Hyper-V Differencing Disk 應用

這樣一來就可以自動化將 Server 架設起來，並且可以使用版控控制此台 Server 的 Hyper-V 設定。

> [!important] Windows 裝置注意事項 如果使用 Windows 設備記得使用 Sysprep 工具 Generalize
> 
> 路徑：`C:\Windows\System32\Sysprep\sysprep.exe`

![[Assets/Note/Tech/Terraform (IaC) 和 Hyper-V指南/08-Hyper-V Differencing Disk 應用.png]]

### 準備 Template VM

將此 VM 的 Virtual Disk 匯出成 Differencing Disk

### 匯出 Differencing Disk 步驟

1. **新增硬碟** ![[Assets/Note/Tech/Terraform (IaC) 和 Hyper-V指南/09-匯出 Differencing Disk 步驟 - 新增硬碟.png]]
    
2. **選擇 Hard Disk** ![[Assets/Note/Tech/Terraform (IaC) 和 Hyper-V指南/10-匯出 Differencing Disk 步驟 - 選擇 Hard Di.png]]
    
3. **選擇 Disk Format** ![[Assets/Note/Tech/Terraform (IaC) 和 Hyper-V指南/11-匯出 Differencing Disk 步驟 - 選擇 Disk Fo.png]]
    
4. **選擇 Differencing** ![[Assets/Note/Tech/Terraform (IaC) 和 Hyper-V指南/12-匯出 Differencing Disk 步驟 - 選擇 Differe.png]]
    
5. **設定匯出名稱** ![[Assets/Note/Tech/Terraform (IaC) 和 Hyper-V指南/13-匯出 Differencing Disk 步驟 - 設定匯出名稱.png]]
    
6. **選擇父級虛擬硬碟** 選擇需要使用的虛擬硬碟，例如 `windows server 2022 Parent.vhdx` ![[Assets/Note/Tech/Terraform (IaC) 和 Hyper-V指南/14-匯出 Differencing Disk 步驟.png]]
    

### Terraform 程式碼修改

主要修改的地方是將 `path` 改為 Differencing Disk 的位置：

```hcl
hard_disk_drives {
  controller_type                 = "Scsi"
  controller_number               = "0"
  controller_location             = "0"
  path                            = "c:/Users/Public/Documents/Hyper-V/Virtual hard disks/winSrv2022Diff.vhdx"
  disk_number                     = 4294967295
  resource_pool_name              = "Primordial"
  support_persistent_reservations = false
  maximum_iops                    = 0
  minimum_iops                    = 0
  qos_policy_id                   = "00000000-0000-0000-0000-000000000000"
  override_cache_attributes       = "Default"
}
```

### 完成畫面

![[Assets/Note/Tech/Terraform (IaC) 和 Hyper-V指南/15-完成畫面.png]]

---

# Azure 雲端環境

## 安裝需求

### Windows

> [!info] Terraform 官方安裝指南 探索 Terraform 產品文檔、教程和範例
> 
> 參考連結：https://developer.hashicorp.com/terraform/downloads

### macOS

```bash
brew tap hashicorp/tap
brew install hashicorp/tap/terraform
```

> [!important] 先決條件 要先安裝 Azure CLI

#### Windows Azure CLI 安裝

> [!info] Azure CLI 安裝指南 Azure CLI 可在 Windows、macOS 和 Linux 環境中安裝。也可以在 Docker 容器和 Azure Cloud Shell 中執行
> 
> 參考連結：https://learn.microsoft.com/zh-tw/cli/azure/install-azure-cli

#### macOS Azure CLI 安裝

```bash
brew update && brew install azure-cli
```

## 建立服務

### providers.tf

下載所需要的 providers：

```hcl
terraform {
  required_version = ">=0.12"

  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~>2.0"
    }
    random = {
      source  = "hashicorp/random"
      version = "~>3.0"
    }
    tls = {
      source = "hashicorp/tls"
      version = "~>4.0"
    }
  }
}

provider "azurerm" {
  features {}
}
```

### variables.tf

建立變數檔案：

```hcl
variable "resource_group_location" {
  type        = string
  default     = "eastus"
  description = "Location of the resource group."
}

variable "resource_group_name_prefix" {
  type        = string
  default     = "rg"
  description = "Prefix of the resource group name that's combined with a random ID so name is unique in your Azure subscription."
}
```

### 建立虛擬網路（VPC）

```hcl
resource "azurerm_virtual_network" "my_terraform_network" {
  name                = "myVnet"
  address_space       = ["10.0.0.0/16"]
  location            = azurerm_resource_group.rg.location
  resource_group_name = azurerm_resource_group.rg.name
}
```

### 建立子網路

```hcl
resource "azurerm_subnet" "my_terraform_subnet" {
  name                 = "mySubnet"
  resource_group_name  = azurerm_resource_group.rg.name
  virtual_network_name = azurerm_virtual_network.my_terraform_network.name
  address_prefixes     = ["10.0.1.0/24"]
}
```

### 建立公用 IP

```hcl
resource "azurerm_public_ip" "my_terraform_public_ip" {
  name                = "myPublicIP"
  location            = azurerm_resource_group.rg.location
  resource_group_name = azurerm_resource_group.rg.name
  allocation_method   = "Dynamic"
}
```

### 建立安全性群組和規則

```hcl
resource "azurerm_network_security_group" "my_terraform_nsg" {
  name                = "myNetworkSecurityGroup"
  location            = azurerm_resource_group.rg.location
  resource_group_name = azurerm_resource_group.rg.name

  security_rule {
    name                       = "SSH"
    priority                   = 1001              # 規則優先權
    direction                  = "Inbound"         # 規則方向
    access                     = "Allow"           # Allow 或 Deny
    protocol                   = "Tcp"             # TCP 或 UDP
    source_port_range          = "*"               # 來源連接埠
    destination_port_range     = "22"              # 目標連接埠
    source_address_prefix      = "*"
    destination_address_prefix = "*"
  }
}
```

#### 安全性規則參數說明

- **name**：規則的名稱
- **priority**：規則的優先權（套用規則的優先順序）
- **direction**：對此規則的描述
- **access**：`Allow` 或 `Deny`
- **protocol**：`Tcp` 或 `Udp`
- **source_port_range**：連接的連接埠
- **destination_port_range**：被連接的連接埠（此 VM 伺服器的連接埠號碼）

### 建立網路介面

```hcl
resource "azurerm_network_interface" "my_terraform_nic" {
  name                = "myNIC"
  location            = azurerm_resource_group.rg.location
  resource_group_name = azurerm_resource_group.rg.name

  ip_configuration {
    name                          = "my_nic_configuration"
    subnet_id                     = azurerm_subnet.my_terraform_subnet.id
    private_ip_address_allocation = "Dynamic"
    public_ip_address_id          = azurerm_public_ip.my_terraform_public_ip.id
  }
}
```

### 完整的虛擬機器建立範例

```hcl
# Connect the security group to the network interface
resource "azurerm_network_interface_security_group_association" "example" {
  network_interface_id      = azurerm_network_interface.my_terraform_nic.id
  network_security_group_id = azurerm_network_security_group.my_terraform_nsg.id
}

# Generate random text for a unique storage account name
resource "random_id" "random_id" {
  keepers = {
    resource_group = azurerm_resource_group.rg.name
  }
  byte_length = 8
}

# Create storage account for boot diagnostics
resource "azurerm_storage_account" "my_storage_account" {
  name                     = "diag${random_id.random_id.hex}"
  location                 = azurerm_resource_group.rg.location
  resource_group_name      = azurerm_resource_group.rg.name
  account_tier             = "Standard"
  account_replication_type = "LRS"
}

# Create and display an SSH key
resource "tls_private_key" "example_ssh" {
  algorithm = "RSA"
  rsa_bits  = 4096
}

# Create virtual machine
resource "azurerm_linux_virtual_machine" "my_terraform_vm" {
  name                  = "myVM"
  location              = azurerm_resource_group.rg.location
  resource_group_name   = azurerm_resource_group.rg.name
  network_interface_ids = [azurerm_network_interface.my_terraform_nic.id]
  size                  = "Standard_DS1_v2"

  os_disk {
    name                 = "myOsDisk"
    caching              = "ReadWrite"
    storage_account_type = "Premium_LRS"
  }

  source_image_reference {
    publisher = "Canonical"
    offer     = "0001-com-ubuntu-server-jammy"
    sku       = "22_04-lts-gen2"
    version   = "latest"
  }

  computer_name                   = "myvm"
  admin_username                  = "azureuser"
  disable_password_authentication = true

  admin_ssh_key {
    username   = "azureuser"
    public_key = tls_private_key.example_ssh.public_key_openssh
  }

  boot_diagnostics {
    storage_account_uri = azurerm_storage_account.my_storage_account.primary_blob_endpoint
  }
}
```

---

# 參考資源

## 官方文檔

> [!info] Terraform 官方文檔 Terraform 的文檔，包括 Terraform CLI、Terraform Cloud 和 Terraform Enterprise
> 
> 參考連結：https://developer.hashicorp.com/terraform/docs

## Terraform Hyper-V

> [!info] Terraform Registry - Hyper-V Provider
> 
> 參考連結：https://registry.terraform.io/providers/taliesins/hyperv/1.0.3/docs

## Terraform Azure

> [!info] 向 Azure 驗證 Terraform 了解使用 Microsoft 帳戶向 Azure 驗證的各種選項
> 
> 參考連結：https://learn.microsoft.com/zh-tw/azure/developer/terraform/authenticate-to-azure?tabs=bash

## Terraform Hyper-V 參數文檔

> [!info] Terraform Registry - Hyper-V Machine Instance
> 
> 參考連結：https://registry.terraform.io/providers/taliesins/hyperv/latest/docs/resources/machine_instance#enable_secure_boot

## Providers

GitHub 專案：https://github.com/taliesins/terraform-provider-hyperv
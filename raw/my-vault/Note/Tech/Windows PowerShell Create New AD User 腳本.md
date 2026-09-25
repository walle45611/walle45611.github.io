---
Type:
  - powershell script
---
```PowerShell
1..9 | foreach {New-ADUser -name "User0$_" -AccountPassword (ConvertTo-SecureString "Skills39" -AsPlainText -Force ) -Enabled $true}
Get-adUser -Filter {name -like "User*"}
```

  

  

- 以下是使用 PowerShell 創建 100 個 AD 用戶、Users 群組和 OU 的程式碼，其中 50 個用戶是教師 (teacher)，另外 50 個用戶是學生 (student)：
    
    ```PowerShell
    # 設定變數
    $DomainName = "yourdomain.com"
    $UserOUName = "Users"
    $TeacherOUName = "Teachers"
    $StudentOUName = "Students"
    $TeacherGroupName = "Teacher Group"
    $StudentGroupName = "Student Group"
    
    # 建立 OU
    New-ADOrganizationalUnit -Name $UserOUName -Path "DC=$DomainName"
    New-ADOrganizationalUnit -Name $TeacherOUName -Path "OU=$UserOUName,DC=$DomainName"
    New-ADOrganizationalUnit -Name $StudentOUName -Path "OU=$UserOUName,DC=$DomainName"
    
    # 建立 Users 群組
    New-ADGroup -Name $TeacherGroupName -GroupCategory Security -GroupScope Global -Path "OU=$TeacherOUName,OU=$UserOUName,DC=$DomainName"
    New-ADGroup -Name $StudentGroupName -GroupCategory Security -GroupScope Global -Path "OU=$StudentOUName,OU=$UserOUName,DC=$DomainName"
    
    # 建立 100 個用戶
    for ($i = 1; $i -le 100; $i++) {
        if ($i -le 50) {
            $UserType = "teacher"
            $OUName = $TeacherOUName
            $GroupName = $TeacherGroupName
        } else {
            $UserType = "student"
            $OUName = $StudentOUName
            $GroupName = $StudentGroupName
        }
        $Username = $UserType + $i
        $Password = ConvertTo-SecureString "P@ssw0rd" -AsPlainText -Force
        $UserParams = @{
            Name = $Username
            GivenName = $UserType
            Surname = $i
            DisplayName = "$UserType $i"
            SamAccountName = $Username
            UserPrincipalName = "$Username@$DomainName"
            AccountPassword = $Password
            Enabled = $true
            Path = "OU=$OUName,OU=$UserOUName,DC=$DomainName"
        }
        New-ADUser @UserParams
        Add-ADGroupMember -Identity $GroupName -Members $Username
    }
    ```
    

  

```PowerShell
$UserOUName = "Users"
$AppleOUName = "Apple"
$LemonOUName = "Lemon" 
$AppleGroupName = "Apple Users" 
$LemonGroupName = "Lemon Users"

New-ADOrganizationalUnit -Name $LemonOUName -Path "DC=LEMON,DC=COM"
New-ADOrganizationalUnit -Name $AppleOUName -Path "DC=LEMON,DC=COM"

New-ADGroup -name "$LemonGroupName" -GroupCategory Security -GroupScope Global -path "OU=$LemonOUName,DC=LEMON,DC=COM"
New-ADGroup -name "$AppleGroupName" -GroupCategory Security -GroupScope Global -path "OU=$AppleOUName,DC=LEMON,DC=COM"

$Password = ConvertTo-SecureString "Skills39" -AsPlainText -Force

for($i=1;$i -le 90;$i++){
    $Username="user" + "{0:d2}" -f $i
    $UserParams = @{
        Name = $Username
        AccountPassword = $Password
        Enabled = $true
    }
    New-ADUser @UserParams
    Add-ADGroupMember -Identity "Lemon Users" -Members $Username
}

for($i=1;$i -le 10;$i++){
    $Username="apple" + "{0:d2}" -f $i
    $UserParams = @{
        Name = $Username
        AccountPassword = $Password
        Enabled = $true
    }
    New-ADUser @UserParams
    Add-ADGroupMember -Identity "Apple Users" -Members $Username
}
```
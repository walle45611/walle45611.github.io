# Windows PowerShell：批次建立 AD 使用者

## 建立 9 個使用者並查詢

建立 `User01` 至 `User09`，啟用帳號後，查詢名稱以 `User` 開頭的 AD 使用者。

```powershell
1..9 | foreach {
    New-ADUser -name "User0$_" -AccountPassword (ConvertTo-SecureString "Skills39" -AsPlainText -Force) -Enabled $true
}
Get-adUser -Filter {name -like "User*"}
```

## 建立教師與學生的 OU、群組及使用者

此範例建立 `Users` OU，並在其下建立 `Teachers`、`Students` OU 與對應的安全性群組，再批次建立 100 個使用者：

- 教師：`teacher1` 至 `teacher50`。
- 學生：`student51` 至 `student100`。

每個使用者會加入對應群組。網域與 OU、群組名稱集中在腳本開頭設定。

```powershell
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

## 建立 Lemon 與 Apple 群組及使用者

此範例在 `LEMON.COM` 網域建立 `Lemon`、`Apple` OU 與對應的安全性群組，並分兩批建立使用者：

- `user01` 至 `user90`：加入 `Lemon Users` 群組。
- `apple01` 至 `apple10`：加入 `Apple Users` 群組。

```powershell
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

for ($i = 1; $i -le 90; $i++) {
    $Username = "user" + "{0:d2}" -f $i
    $UserParams = @{
        Name = $Username
        AccountPassword = $Password
        Enabled = $true
    }
    New-ADUser @UserParams
    Add-ADGroupMember -Identity "Lemon Users" -Members $Username
}

for ($i = 1; $i -le 10; $i++) {
    $Username = "apple" + "{0:d2}" -f $i
    $UserParams = @{
        Name = $Username
        AccountPassword = $Password
        Enabled = $true
    }
    New-ADUser @UserParams
    Add-ADGroupMember -Identity "Apple Users" -Members $Username
}
```

CloudDrive = Cloud Shell 的持久化檔案儲存區，用來跨 session 保存腳本與 `$HOME` 內容。


- ARM Template 是 **JSON 格式的宣告式 IaC**，用來描述要部署的 Azure 資源，而不是一步一步手動建立。官方連結：[Explore template structure](https://learn.microsoft.com/zh-tw/training/modules/create-azure-resource-manager-template-vs-code/2-explore-template-structure?tabs=azure-cli)
    
- ARM Template 透過 **Azure Resource Manager** 進行部署，因此可以用同一份範本重複建立一致的環境。官方連結：[Explore template structure](https://learn.microsoft.com/zh-tw/training/modules/create-azure-resource-manager-template-vs-code/2-explore-template-structure?tabs=azure-cli)
    
- 主要優勢可整理為：**一致性、等冪性、自動化、可驗證**。同一份範本重複部署，理論上會得到一致結果。官方連結：[Explore template structure](https://learn.microsoft.com/zh-tw/training/modules/create-azure-resource-manager-template-vs-code/2-explore-template-structure?tabs=azure-cli)
    
- ==ARM Template 常見的重要區段有：`$schema`、`contentVersion`、`parameters`、`variables`、`resources`、`outputs`。==官方連結：[Explore template structure](https://learn.microsoft.com/zh-tw/training/modules/create-azure-resource-manager-template-vs-code/2-explore-template-structure?tabs=azure-cli)
    
- `resources` 是真正定義 Azure 資源的核心。官方連結：[Explore template structure](https://learn.microsoft.com/zh-tw/training/modules/create-azure-resource-manager-template-vs-code/2-explore-template-structure?tabs=azure-cli)
    
- `parameters` 用來在部署時傳入值，`variables` 用來簡化重複內容，`outputs` 用來在部署完成後回傳結果。官方連結：[Explore template structure](https://learn.microsoft.com/zh-tw/training/modules/create-azure-resource-manager-template-vs-code/2-explore-template-structure?tabs=azure-cli)
    
- Resource type 的寫法是 **`資源提供者/資源類型`**，例如：`Microsoft.Storage/storageAccounts`。官方連結：[Resource providers and types](https://learn.microsoft.com/zh-tw/azure/azure-resource-manager/management/resource-providers-and-types)
    
- 常見的 Resource type 範例：`Microsoft.Storage/storageAccounts`、`Microsoft.Network/virtualNetworks`、`Microsoft.Network/networkSecurityGroups`、`Microsoft.Compute/virtualMachines`。官方連結：[Resource providers and types](https://learn.microsoft.com/zh-tw/azure/azure-resource-manager/management/resource-providers-and-types)
    
- 部署到 **Resource Group** 範圍時，Azure CLI 常用指令是 `az deployment group create`。官方連結：[az deployment group](https://learn.microsoft.com/zh-tw/cli/azure/deployment/group?view=azure-cli-latest)
    
- 預覽部署後會改到哪些資源時，常用指令是 `az deployment group what-if`。官方連結：[ARM 範本部署假設狀況作業](https://learn.microsoft.com/zh-tw/azure/azure-resource-manager/templates/deploy-what-if)
    
- `--confirm-with-what-if` 可在部署前先顯示變更內容，再決定是否真的執行。官方連結：[ARM 範本部署假設狀況作業](https://learn.microsoft.com/zh-tw/azure/azure-resource-manager/templates/deploy-what-if)
    
- 若是 **Subscription** 層級部署，常用指令會改成 `az deployment sub create` 或 `az deployment sub what-if`。官方連結：[將資源部署至訂用帳戶](https://learn.microsoft.com/zh-tw/azure/azure-resource-manager/templates/deploy-to-subscription)




- `parameters`：部署時傳入值，讓 ARM Template 更彈性、可重複使用。
	
- `outputs`：部署完成後回傳值，方便取得資源名稱、ID、端點等資訊。
	
- `variables`：範本內部使用的值，用來簡化重複內容，不是外部輸入。
	
- `resources`：真正定義 Azure 資源的核心區塊。
	
- ARM Template 的邏輯可以記成：**輸入參數 → 部署資源 → 輸出結果**。
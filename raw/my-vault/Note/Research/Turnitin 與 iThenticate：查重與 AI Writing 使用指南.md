# Turnitin 與 iThenticate：查重與 AI Writing 使用指南

## 查重與 AI Writing 的差別

| 功能 | 用途 |
| --- | --- |
| Similarity（相似度／查重） | 找出與其他來源相似的文字，檢查引用與改寫是否恰當。 |
| AI Writing | 偵測文字是否可能由 AI 生成，供進一步檢查。 |

**兩個數值是分開的。** 相似度高不一定是抄襲，AI Writing 也可能誤判；要回到標示段落與來源確認，不能只看百分比。[Turnitin：相似度說明](https://guides.turnitin.com/hc/en-us/articles/23713493434253-Understanding-the-similarity-score-for-students)、[AI Writing 說明](https://guides.turnitin.com/hc/en-us/articles/22774058814093-Using-the-AI-Writing-Report)

## 上傳與查看報告

1. 依學校提供的方式登入，進入 **My Files**，點右上角 **Upload**。

![[Assets/Note/Research/Turnitin 使用指南/01-Upload.png]]

2. 選擇 PDF，確認 **Title (Required)**，再按 **Submit**。截圖中的作者姓名欄位為選填。

![[Assets/Note/Research/Turnitin 使用指南/02-Submit.png]]

3. 處理完成後，從檔案列表開啟 **Similarity** 報告，查看相似段落與右側 **Sources**。需要調整比對條件時，點 **Filters**。

![[Assets/Note/Research/Turnitin 使用指南/03-Similarity.png]]

4. 依學校或老師要求設定排除條件，再按 **Apply Filters**。截圖勾選的是 **Exclude bibliography**（排除參考文獻）；其他常見選項包括 **Exclude quoted text**（排除引文）與 **Exclude small matches**（排除短小相符內容）。比較不同版本時，應使用相同設定。

![[Assets/Note/Research/Turnitin 使用指南/04-Filters.png]]

5. 切換上方 **AI Writing** 查看 AI 偵測結果；需要下載報告時，可由右上角 **Download** 選擇可用項目。

![[Assets/Note/Research/Turnitin 使用指南/05-AI-Writing.png]]

畫面中的 **`*%` 不代表 0%**，而是低於 20% 的偵測結果未顯示具體數字，以降低誤判造成的誤解。能否產生 AI Writing 報告，也取決於文件字數、語言及帳號功能；能上傳 PDF 不代表一定有 AI 報告。[官方說明](https://guides.turnitin.com/hc/en-us/articles/28457596598925-How-to-access-the-AI-Writing-Report)

實務上，**AI Writing 偏高時，通常希望修改到 20% 以下**，可作為這份指南的參考目標，實際仍依學校或老師要求。偵測有時會誤判、造成分數虛高，因此應閱讀被標示的段落，斟酌修改不自然或不精確的表達；不必為了壓低數字而改壞原本正確的內容。**低於 20% 也不代表一定合格。**

## 額度不夠時：先用 ZeroGPT 輔助檢查

依本次提供的學校使用資訊，**每天最多上傳 3 份 PDF**；這是學校使用限制，實際額度以校方與帳號畫面為準。

若仍在反覆小修 AI Writing 的部分，可先用 [ZeroGPT](https://www.zerogpt.com/) 做初步檢查，把學校額度留給較完整的版本：

1. 先檢查需要修改的文字，把偵測結果當作重新閱讀的提示。
2. 依自己的研究內容修正空泛、不精確或不自然的表達，確認論述與引用正確。
3. 整理成完整 PDF 後，再用學校提供的工具確認並保留報告。

![[Assets/Note/Research/Turnitin 使用指南/06-ZeroGPT.png]]

也可嘗試 [ZeroGPT AI Humanizer](https://www.zerogpt.com/ai-humanizer) 輔助潤飾，減少生硬、制式的「AI 味道」。修改後仍要自行確認原意、專業用語與引用是否正確；語句更自然不代表 AI 偵測分數一定會降低。

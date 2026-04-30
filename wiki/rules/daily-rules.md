# Daily Rules

## 任務目標

建立或更新 `wiki/assets/daily/YYYY-MM-DD.md`，把當日已歸檔的 summary 整理成可快速瀏覽的日報。

## 選材流程

寫入 daily 前，先做這三步：

1. 讀 `wiki/index.md` 的 `## Summaries`，找出日期對應到當日的 summary。
2. 打開這些 `wiki/summaries/...` 頁面，只根據已寫好的 summary 內容整理。
3. 若同一天有多篇 summary，以 summary 頁為主，不回頭重掃 `raw/`。

## 輸出路徑

固定寫入：

`wiki/assets/daily/YYYY-MM-DD.md`

## Daily 模板

輸出必須是合法 Markdown，並盡量維持這個結構：

```md
# Daily Digest <日期>

- date: <YYYY-MM-DD>
- timezone: Asia/Taipei
- generated_at: <YYYY-MM-DD>
- basis: [[wiki/summaries/...]], [[wiki/summaries/...]]

## Summary

📅 這是您在 <日期> 的知識庫攝取紀錄：

📚 共整理了 <N> 篇內容。

- <主題名稱>
  - source: `raw/...`  # 若來源來自 raw
  - source link: <外部原始來源網址>
  - 1 句話核心概念：<一句話摘要>
  - 3 到 5 個重點：
    - <重點 1>
    - <重點 2>
    - <重點 3>

## 知識串聯

<可選，寫一段當日內容之間的共同觀察>
```

## 寫作準則

1. `source link` 優先放外部原始來源網址。
2. 若來源來自 `raw/`，要另外保留 `source: raw/...`。
3. `basis` 優先用 `[[wiki/summaries/...]]`，必要時可加 `[[wiki/index.md]]`。
4. 僅寫入已實際讀取與整理過的內容，不補寫、不猜測。
5. 同一天的 daily 只更新同一份檔案，不新增重複檔。

## 寫入與回覆

1. 寫檔用 `upsert_file`。
2. 成功寫入後，同步更新 `wiki/log.md`，並記錄 basis、updated path、summary 數量或連結情況。
3. 對使用者的最終回覆要能直接對應 daily 內容，不能只說「已完成」。

## 失敗處理

若目標日期不明、來源不足或寫入失敗，直接說明原因，不要捏造內容，也不要先假裝 log 已完成。

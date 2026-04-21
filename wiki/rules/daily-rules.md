# Daily Rules

## 任務目標

任務目標是建立或更新 `wiki/assets/daily/YYYY-MM-DD.md`
## 輸入資料

在寫入 daily 檔案前，必須先取得以下資訊：

1. 從 `wiki/index.md` 的 `Summaries` 區塊確認當日有哪些 summary 條目。
2. 依照 `## Summaries` 條目中的日期與路徑，打開對應的 `wiki/summaries/...` 頁面。
3. 這類情況應以 `wiki/summaries/...` 為主，因為需要較細的逐篇資訊。

## 輸出路徑

輸出檔案路徑必須固定為：

`wiki/assets/daily/YYYY-MM-DD.md`

例如：

- `wiki/assets/daily/2026-04-17.md`
- `wiki/assets/daily/2026-04-21.md`

## 檔案格式

輸出內容必須是合法的 Markdown。

建議格式如下：

```md
# Daily Digest 2026-04-21

- date: 2026/04/21 星期二
- timezone: Asia/Taipei
- generated_at: 2026-04-21T10:00:00.000Z
- query: Date.now=1776700000000，請整理 2026/04/21 我讀了什麼？

## Summary

這裡放當日整理完成的摘要內容。
```

## 寫入規則

1. 最終檔案必須使用 `upsert_file` 寫入。
2. 寫入路徑必須位於 `wiki/assets/daily/` 之下。
3. 檔案內容可以使用一般 Markdown 語法。
4. 同一天的 daily 應更新同一份檔案，不應建立多份重複檔案。
5. 若資料不足，不得自行補寫或猜測缺漏內容。
6. 只能寫入已實際讀取與整理過的內容。

## 對使用者的回覆規則

在檔案成功寫入後：

- 不要把完整 Markdown 檔案內容直接回傳給使用者
- 只回覆精簡的純文字確認訊息
- 回覆中應明確指出已建立或更新的檔案路徑

例如：

`已更新 wiki/assets/daily/2026-04-21.md`

## 失敗處理

若出現以下情況：

- 目標日期不明確
- 必要來源資料不足
- 檔案寫入失敗

則必須：

1. 直接說明問題
2. 不得捏造內容
3. 若 `upsert_file` 未成功，禁止聲稱檔案已建立或更新

# Output Rules

本檔只定義輸出格式規則，不處理資料讀取、判斷流程、寫檔邏輯或 wiki 內容結構。

## 核心原則

同一次任務可以同時包含兩種輸出：

1. 寫入檔案的內容
2. 最後回覆給使用者的訊息

這兩種輸出應分開處理，不能混用格式。

## 1. 寫入檔案時

若本次任務需要修改或建立 `wiki/` 檔案，則：

1. 寫入檔案的內容應使用適合長期維護的 Markdown。
2. 可依任務需求使用標題、清單、區塊與其他 Markdown 結構。
3. 檔案內容應重視可讀性、可追溯性與後續維護性。

## 2. 最後回覆給使用者時

若本次任務需要對使用者輸出最終訊息，則：

1. 最終回覆預設應使用 `zh-TW`。
2. 若任務規則提供資訊順序或段落結構，應保留該資訊架構，不可因為排版調整就省略必要資訊。
3. 多個資訊點預設以段落、條列或分段呈現，不要把回覆退化成只有標題、連結清單或篇名羅列。
4. 若 task rule 明確要求逐篇摘要、整合結論或比較結果，則最終回覆必須達到該資訊密度；禁止只輸出簡短總覽句來提前結束。
5. 禁止結尾語，如 `以上資訊已經在知識庫中整理，若需更深入的安裝腳本或實際操作範例，請參考對應摘要頁或直接查閱原始 raw/ 檔案` 等等結尾之語氣。
6. 如有特別要求使用 Telegram MarkdownV2 回覆給 Telegram 使用者訊息時參考各種語法範例如下，請記住 Markdown not eq Telegram MarkdownV2，並且只能使用以下的合法格式，其他 Markdown 語法接不允許。

```text
bold \*text*
_italic \*text_
__underline__
~strikethrough~
||spoiler||
bold _italic bold ~italic bold strikethrough ||italic bold strikethrough spoiler||~ __underline italic bold___ bold*
[inline URL](http://www.example.com/)
[inline mention of a user](tg://user?id=123456789)
![👍](tg://emoji?id=5368324170671202286)
![22:45 tomorrow](tg://time?unix=1647531900&format=wDT)
![22:45 tomorrow](tg://time?unix=1647531900&format=t)
![22:45 tomorrow](tg://time?unix=1647531900&format=r)
![22:45 tomorrow](tg://time?unix=1647531900)
`inline fixed-width code`
`​`​`
pre-formatted fixed-width code block
`​`​`
`​`​`python
pre-formatted fixed-width code block written in the Python programming language
`​`​`
>Block quotation started
>Block quotation continued
>Block quotation continued
>Block quotation continued
>The last line of the block quotation
>It is separated from the previous block quotation by an empty bold entity
>Expandable block quotation continued
>Hidden by default part of the expandable block quotation started
>Expandable block quotation continued
>The last line of the expandable block quotation with the expandability mark||
```
- Please note:
	- Any character with code between 1 and 126 inclusively can be escaped anywhere with a preceding '' character, in which case it is treated as an ordinary character and not a part of the markup. This implies that '' character usually must be escaped with a preceding '' character.
	- Inside `pre` and `code` entities, all ‘`’ and '' characters must be escaped with a preceding '' character.
	- Inside the `(...)` part of the inline link and custom emoji definition, all ‘)’ and '' must be escaped with a preceding '' character.
	- In all other places characters ‘_’, ‘*’, ‘[’, ‘]’, ‘(’, ‘)’, ‘~’, ‘`’, ‘>’, ‘#’, ‘+’, ‘-’, ‘=’, ‘|’, ‘{’, ‘}’, ‘.’, ‘!’ must be escaped with the preceding character ''.
	- In case of ambiguity between `italic` and `underline` entities `__` is always greedily treated from left to right as beginning or end of an `underline` entity, so instead of `___italic underline___` use `___italic underline_**__`, adding an empty bold entity as a separator.
	- A valid emoji must be provided as an alternative value for the custom emoji. The emoji will be shown instead of the custom emoji in places where a custom emoji cannot be displayed (e.g., system notifications) or if the message is forwarded by a non-premium user. It is recommended to use the emoji from the emoji field of the custom emoji sticker.
	- Custom emoji entities can only be used by bots that purchased additional usernames on Fragment.
	- See date-time entity formatting for more details about supported date-time formats.
	- Character '#' is reserved and must be escaped with the preceding '\\'

## 3. 同時有寫檔與回覆時

若同一次任務同時包含「修改檔案」與「回覆使用者」，則：

1. 寫入檔案的內容使用 Markdown。
2. 不可把要寫入檔案的 Markdown 內容整份原封不動直接回給使用者。

## 4. 套用方式

1. 本檔只管輸出格式，不取代其他 task rule。
2. 其他規則檔若提到輸出格式，應以本檔為準。
3. 若同一次任務同時觸發多個 rules，仍以本檔判斷「寫入檔案」與「回覆使用者」的輸出方式。

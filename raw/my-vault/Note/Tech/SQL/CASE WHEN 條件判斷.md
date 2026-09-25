# CASE WHEN 條件判斷

## [#610 Triangle Judgement](https://leetcode.com/problems/triangle-judgement/)

你記得三角形條件是任意兩邊和都要大於第三邊。三個條件同時成立才是三角形，因此用 `AND`。

你原本的草稿在 `CASE` 後面寫了 `WHERE`：

```sql
SELECT
    x,
    y,
    z,
    CASE
        WHERE x + y > z
          AND x + z > y
          AND y + z > x
        THEN 'YES'
        ELSE 'NO'
    END AS triangle
FROM Triangle;
```

正確寫法是 `CASE WHEN`：

```sql
SELECT
    x,
    y,
    z,
    CASE
        WHEN x + y > z
         AND x + z > y
         AND y + z > x
        THEN 'Yes'
        ELSE 'No'
    END AS triangle
FROM Triangle;
```

`CASE WHEN` 可以想成 SQL 的 `if / else`。題目要每列都保留並新增判斷結果，所以把 `CASE` 放在 `SELECT`；`WHERE` 則用來篩選要留下的列。

## 易錯點

- 語法是 `CASE WHEN 條件 THEN 結果 ELSE 結果 END`，不要寫成 `CASE WHERE`。
- 先分清楚需求是「替每列計算分類值」還是「只留下符合條件的列」：前者用 `CASE`，後者用 `WHERE`。
- 回傳文字時注意題目要求的標籤大小寫；本題範例是 `Yes` 和 `No`。

來源：ChatGPT 對話「SQL子查詢解法」。

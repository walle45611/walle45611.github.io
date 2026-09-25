# JOIN 與反連接

## 保留需要的資料列

### [#175 Combine Two Tables](https://leetcode.com/problems/combine-two-tables/)

題目要保留每個 Person；沒有地址的人仍要出現在結果中，地址欄位顯示 `NULL`，所以 Person 放在左邊：

```sql
SELECT p.firstName, p.lastName, a.city, a.state
FROM Person p
LEFT JOIN Address a
    ON p.personId = a.personId;
```

`LEFT JOIN` 保留左表所有列；`INNER JOIN` 只留下配對成功的列。

### [#577 Employee Bonus](https://leetcode.com/problems/employee-bonus/)

找 bonus 小於 1000 或沒有 bonus 紀錄的員工：

```sql
SELECT e.name, b.bonus
FROM Employee e
LEFT JOIN Bonus b
    ON e.empId = b.empId
WHERE b.bonus < 1000
   OR b.bonus IS NULL;
```

沒有 bonus 的員工不在 Bonus 表中，所以需要 `LEFT JOIN` 並保留 `NULL`。

## Self join：同一張表扮演兩個角色

### [#181 Employees Earning More Than Their Managers](https://leetcode.com/problems/employees-earning-more-than-their-managers/)

```sql
SELECT e.name AS Employee
FROM Employee e
JOIN Employee m
    ON e.managerId = m.id
WHERE e.salary > m.salary;
```

`e` 是員工，`m` 是同一張 Employee 表中的主管。

### [#511 Game Play Analysis I](https://leetcode.com/problems/game-play-analysis-i/)

你用 self join 找沒有更早登入紀錄的那一列，但起初選了右表日期：

```sql
SELECT a1.player_id, a2.event_date AS first_login
FROM Activity a1
LEFT JOIN Activity a2
    ON a1.player_id = a2.player_id
   AND a2.event_date < a1.event_date
WHERE a2.player_id IS NULL;
```

`WHERE a2.player_id IS NULL` 留下的列沒有右表配對，因此 `a2.event_date` 也必定是 `NULL`。要輸出日期，改選左表的 `a1.event_date`：

```sql
SELECT a1.player_id, a1.event_date AS first_login
FROM Activity a1
LEFT JOIN Activity a2
    ON a1.player_id = a2.player_id
   AND a2.event_date < a1.event_date
WHERE a2.player_id IS NULL;
```

同題的 `GROUP BY + MIN()` 解法放在 [[Note/Tech/SQL/分組與資料修改]]。

## 反連接：找不到符合資料的列

### [#183 Customers Who Never Order](https://leetcode.com/problems/customers-who-never-order/)

```sql
SELECT c.name AS Customers
FROM Customers c
WHERE NOT EXISTS (
    SELECT 1
    FROM Orders o
    WHERE o.customerId = c.id
);
```

題目找的是沒有訂單的客戶，不是兩張表的交集。也可以用 `LEFT JOIN ... IS NULL` 表達相同的反連接概念。

### [#607 Sales Person](https://leetcode.com/problems/sales-person/)

找出沒有替 RED 公司下過單的業務員。內層先把 Orders 和 Company 依 `com_id` 配對，再比對外層業務員：

```sql
SELECT s.name
FROM SalesPerson s
WHERE NOT EXISTS (
    SELECT 1
    FROM Orders o
    JOIN Company c
        ON o.com_id = c.com_id
    WHERE o.sales_id = s.sales_id
      AND c.name = 'RED'
);
```

`s.sales_id` 參照外層目前檢查的業務員；只要找到一筆他替 RED 下的訂單，`NOT EXISTS` 就不成立。

## 易錯點

- 先確認題目要完整保留哪張表，再選 JOIN。`INNER JOIN` 會丟掉未配對列。
- `LEFT JOIN ... WHERE 右表欄位 IS NULL` 挑出的是右表沒配到的列。若要輸出存在的資料，通常應選左表欄位。
- `LEFT JOIN` 後的 `WHERE` 條件可能把未配對資料篩掉；要保留缺少 Bonus 的員工時，記得處理 `b.bonus IS NULL`。
- 把 `NOT EXISTS` 想成逐筆檢查的邏輯模型有助理解，但不表示資料庫一定照字面執行固定次數的迴圈。`SELECT 1` 是常見寫法，`EXISTS` 只關心有沒有列，不關心 `1` 的值。

來源：ChatGPT 對話「複習 SQL查詢」及「SQL子查詢解法」。

---
title: RESTful 和 API 關係介紹
date: 2024-09-12 13:39:35
tags: [RESTful, API]
categories: IT 技術
---

![圖 1. : Blog Image](https://imgur.com/ihJNO9r.png)

## API 介紹

Web API 可以被視為用戶端和 Web 資源之間的中介。

### **用戶端 (Clients)**

應用程式介面（*Application Programming Interface*）定義了與其他軟體系統通訊時必須遵循的規則。開發人員公開或建立 API，以便其他應用程式能夠以程式設計方式與其應用程式通訊。例如，時間表應用程式公開一個 API，要求提供員工的全名和日期範圍。收到此資訊後，它會在內部處理員工的時間表，並傳回該日期範圍內的工作時數。

<!--more-->

> **這邊的 Client 代表了 Browser，也就是操作網頁瀏覽器的人**  
用戶端是希望從 Web 存取資訊的使用者。用戶端可以是使用 API 的人或軟體系統。例如，開發人員可以編寫程式從天氣系統獲取天氣資料。或者，您可以直接在瀏覽器中存取相同的資料，這時瀏覽器就是用戶端。

### **資源 (Resource)**

> **這邊的 Resource 代表了 Database，對此應用程式有相關應用到的資料**  
資源是不同應用程式向其用戶端提供的資訊。資源可以是影像、影片、文字、數字或任何類型的資料。將資源提供給用戶端的機器也稱為伺服器。組織使用 API 共用資源並提供 Web 服務，同時維護安全、控制和身分驗證。此外，API 協助他們確定哪些用戶端可以存取特定的內部資源。

## REST 介紹

### What is REST (Representational State Transfer)

是一種 `軟體架構風格`，API 開發者可以使用若干不同的架構來設計 API。遵循 REST 架構風格的 API 稱為 REST API。實作 REST 架構的 Web 服務稱為 RESTful Web 服務。術語 RESTful API 通常是指 RESTful Web API。然而，您可以互換使用術語 REST API 和 RESTful API。

### REST Architectural Style (REST 架構風格)

- **Uniform interface (統一介面)**
  - 有唯一的 URL 表示資源的位置，例如，使用 `/api/1001` 來對資源進行操作，這樣可以避免多個定義不清楚的操作，如 `/api/Get1001`。統一介面的好處在於簡潔一致。
  - 資源路徑範例：
    - HTTP method GET `http://localhost:3000/api/1001` 表示`取得` 1001 這個資源，可能對應到資料庫中的某項資源。
    - HTTP method PUT `http://localhost:3000/api/1001` 表示`編輯` 1001 這個資源。
    - HTTP method DELETE `http://localhost:3000/api/1001` 表示`刪除` 1001 這個資源。

- **Statelessness (無狀態)**
  - 無狀態的意思是將`使用者狀態`與 `request` 隔離。這樣可以避免中間傳遞的使用者資訊被截取，並提高安全性。
  - **有狀態**操作依賴於前一個步驟，例如查詢工資的步驟：
        1. 登入系統
        2. 進入查詢工資的頁面
        3. 搜尋員工
        4. 點擊名稱查詢工資
  - 無狀態的操作則能直接執行，輸入 URL 即可得到資料。

- **Layered system (分層架構)**
  - RESTful Web Services 可以分為不同層級來處理不同的需求，如登入、應用設定或商業邏輯等。這種分層設計使軟體架構更加清晰，但對於用戶端來說這些分層是不可見的。

    [Layered system constraint in REST API](https://stackoverflow.com/questions/30303116/layered-system-constraint-in-rest-api)

- **Cacheability (快取)**
  - Client 和 Server 之間的通訊可以透過快取儲存，以減少 Client 向 Server 發送請求的次數，從而提高應用程式的效率。Response 需要明確指出是否可以快取，以避免資料不同步。

    [Redis - The Real-time Data Platform](https://redis.io/)

- **Code on demand (隨需編碼)**
  - Server 可以根據需求返回不同的 HTTP Status code。例如，操作成功回傳 200，新增資料錯誤回傳 400，認證錯誤回傳 401。這可以有效處理錯誤訊息和成功回應。

### What are the benefits of RESTful APIs?

- **Scalability (可擴展性)**
  - REST API 系統可以有效擴展，因為伺服器不需要保存過去的用戶端請求資訊，無狀態減輕了伺服器負擔。

- **Flexibility (靈活性)**
  - 支持前後端分離的架構，降低專案耦合度。

- **Independence (獨立性)**
  - RESTful API 是一種開發架構，而非特定技術。因此，API 可以用多種程式語言編寫，不會影響 API 設計。

### RESTful 設計中 URI 中不能有動詞

在 RESTful 架構中，每個網址代表一種資源，因此網址中不能有動詞，動作由 HTTP 的 `GET`、`POST`、`PUT`、`DELETE` 方法來表示。雖然這不是強制規範，但依據這個原則設計會更加清晰。畢竟 HTTP method 已經涵蓋了這些操作，不需要再在 URL 中特別表示操作類型。

### RESTful API 如何運作的

![圖 2. : RESTful API 如何運作的](https://imgur.com/z1kOtxa.png)

- 透過設計好的 API 格式，由 Client 發送 request 到 Web Server 上 (Application)

## What does the RESTful API client request contain?

- **Unique resource identifier**
  - 這邊代表了 `http://localhost/api/1001` 1001 代表了唯一的數值，可能是個 sid 或是 uid 就是代表一個東西的代號
- **常用的 HTTP Method**
  - `GET` : 獲取資源
  - `POST`： 新建資源
  - `PUT`：在伺服器更新資源（向客戶端提供改變後的所有資源）
  - `PATCH` : 在伺服器更新資源（向客戶端提供改變的屬性）
  - `DELETE`：刪除資源
  - `PATCH`：一般不用，用 PUT
- **HTTP Header**
  - 請求標頭是用戶端與伺服器之間交換的中繼資料。例如，請求標頭會指示請求和回應的格式，提供有關請求狀態的資訊等。
- **Data**
  - REST API 請求可能包含 POST、PUT 和其他 HTTP 方法成功運作的資料。
  - JSON
- **Parameters**
  - RESTful API 請求可包含參數，為伺服器提供更多關於需要做什麼的詳細資訊。以下是一些不同類型的參數：
    - 指定 URL 詳細資訊的路徑參數。
    - 請求更多資源相關資訊的查詢參數。
    - 快速驗證用戶端的 Cookie 參數。

## 參考

- [RESTful API規範（詳細版）](https://www.796t.com/content/1541631243.html)
- [什麼是 RESTful API？](https://aws.amazon.com/tw/what-is/restful-api/)
- [RESTful Web API 設計](https://learn.microsoft.com/zh-tw/azure/architecture/best-practices/api-design)
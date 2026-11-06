---
blog: true
blog_title: "JWT 真的需要雙 Token 嗎？從 Cookie Session、OAuth 到 BFF 與 Microservices 的架構取捨"
blog_date: 2026-11-06
blog_url: https://blog.walle4561.com/articles/posts/jwt-cookie-session-oauth-bff/
---

使用 JWT 那麼久了，我一直有一個疑問：

**為什麼不直接使用一個 Cookie 搭配 Token 來維持登入狀態，而要額外拆分成 Access Token 和 Refresh Token？**

如果系統為了支援 Refresh Token Rotation、撤銷機制以及登入裝置管理，最後仍然需要 Redis 或 Database 儲存狀態，那麼這樣的架構，是不是又回到了傳統的 Stateful Session？

進一步延伸，如果使用 Cookie Session 就能解決問題，我們是否還需要 JWT？而這種設計又是否適合 Microservices Architecture？

閱讀 RFC 6749、RFC 9700、RFC 10017 與 OWASP 相關文件之後，我發現這些問題其實涉及不同層次的設計決策。

## 一、Cookie、JWT 與 Stateful 並不是同一個層次的概念

首先，需要區分三件事情。

**Cookie 是瀏覽器傳送資料的機制。**

瀏覽器可以透過 Cookie 儲存 Session ID，也可以儲存 JWT。搭配 HttpOnly、Secure 與 SameSite 等屬性，可以限制 JavaScript 存取、明文傳輸及部分跨站請求行為。

**JWT 是一種 Token 格式。**

依據 RFC 7519，JWT 用於表示一組 Claims。它可以透過數位簽章確保完整性，讓接收端在具備可信任金鑰的情況下驗證 Token，而不必每次都向中央伺服器查詢。

**Stateful / Stateless 則描述伺服器管理或驗證狀態的方式。**

例如，使用 Session ID 並查詢 Redis 取得登入資訊，是典型的 Stateful Session。

相反地，如果使用 JWT，API 只驗證簽章、有效期限、Issuer、Audience 等資訊，而不查詢 Session Store，就能做到 Stateless Token Validation。

因此：

- Cookie 不等於 Stateful。
- JWT 不等於 Stateless。
- Access Token 不一定是 JWT。
- 使用 JWT 也不代表整個 Authentication System 不需要狀態。

尤其需要注意，**Token Validation 的 Stateless，與整個系統的 Stateless，是兩回事。**

## 二、為什麼要拆分 Access Token 與 Refresh Token？

在 OAuth 2.0 中，Access Token 與 Refresh Token 有不同的責任。

Access Token 用來存取受保護的資源，例如呼叫 API。

Refresh Token 則用來向 Authorization Server 取得新的 Access Token，而不必每次都重新進行授權流程。

這種拆分的核心，是將短期資源存取權限與較長期的續期能力分離。

假設 Access Token 的有效期限為 15 分鐘，而 Refresh Token 可以在符合安全政策的情況下維持較長的登入體驗。

當 Access Token 外洩時，攻擊者可利用的時間相對有限；Refresh Token 因為具備延續存取能力，則需要更嚴格的保護。

不過，短效 Token 只能降低外洩後的利用時間，並不能避免有效期限內的濫用。

另外，RFC 6749 並沒有要求所有系統都必須發放 Refresh Token。

換句話說，**雙 Token 是一種 Token Lifecycle Design，而不是 JWT 的必要組成。**

## 三、Refresh Token Rotation 是否讓系統重新變成 Stateful？

這是我原本最困惑的地方。

RFC 9700 §4.14.2 對 Public Client 的 Refresh Token Replay Detection 提出明確要求：如果發放 Refresh Token，Authorization Server 必須採用 Sender-Constrained Refresh Token 或 Refresh Token Rotation。

使用 Rotation 時，每次成功續期都會發放新的 Refresh Token，讓舊 Token 失效，並保留 Token 之間必要的關聯資訊。

如果舊 Token 再次被使用，系統便能偵測可能發生的重放攻擊。

這意味著 Authorization Server 需要管理一定程度的狀態。

但這不代表整個架構就完全等同於傳統 Stateful Session。

因為兩者管理的狀態並不相同。

傳統 Session Store 通常保存使用者的登入狀態；Refresh Token Rotation 則是管理 Token 的有效性、續期與重放偵測。

如果 API 使用短效 JWT 並在本地完成驗證，仍然可以避免每次 API Request 都查詢中央 Session Store。

所以，更精確的說法應該是：

**Stateless Access Token Validation 與 Stateful Token Lifecycle Management 可以同時存在。**

當然，這個選擇也有代價，例如 Access Token 在過期前可能無法立即撤銷，除非另外導入撤銷檢查或縮短 Token 的有效期限。

## 四、如果只是一般 Web App，Cookie Session 不就夠了？

其實很多情況下，確實足夠。

假設系統只有一個 Web Frontend 和一個 Backend API。

使用者登入後，伺服器建立 Session，瀏覽器保存 HttpOnly Session Cookie。後續請求由伺服器查詢 Session Store，確認使用者身分與登入狀態。

這樣的架構具備相對直接的登入、登出、Session 撤銷與裝置管理能力。

如果沒有跨應用程式授權、第三方 API 存取或獨立的 Resource Server，未必需要引入完整的 OAuth 2.0 與 Refresh Token 機制。

但 Cookie Session 仍需要考慮 CSRF、防止 Session Fixation、Session Expiration 與安全的 Cookie 設定。

HttpOnly 也不能阻止 XSS 惡意程式碼直接以使用者身分發送請求。

因此，Session 並非沒有安全成本，只是它解決的問題與 OAuth Token Lifecycle 並不完全相同。

## 五、BFF 是不是就回到了傳統 Web Authentication？

研究 RFC 10017 時，我發現這裡有一個容易混淆的地方。

BFF（Backend for Frontend）本身是一種架構模式，通常用於建立專門服務某一種前端的後端介面層。

但 RFC 10017 §6.1 所描述的 OAuth BFF，有更具體的定義。

在這種架構中：

Browser → Session Cookie → BFF → Access Token → Resource Server

瀏覽器只接觸 Session Cookie，而 BFF 作為 OAuth Confidential Client，負責 OAuth 授權流程、Token 管理與 API Request Forwarding。

如果需要 Refresh Token，也由 BFF 管理，而不直接暴露給瀏覽器。

需要特別區分的是：OAuth 2.0 主要是授權框架，使用者身分驗證通常可以透過 OpenID Connect（OIDC）等機制整合。

因此，即使瀏覽器只看到 Session Cookie，也不能推論整個系統沒有使用 OAuth。

另一方面，如果系統採用自己的登入機制，建立傳統 Session，再由 BFF 對後端 API 發出請求，也可以構成一般意義上的 BFF 架構。

但這就不是 RFC 10017 所定義的 OAuth BFF 流程。

**兩種架構的瀏覽器端體驗可能很類似，內部的身分與授權機制卻不相同。**

## 六、那 Cookie Session 對 Microservices 真的不友善嗎？

這個問題需要從系統邊界來看。

假設我們有 User Service、Order Service 與 Payment Service。

如果每個 Microservice 都需要拿瀏覽器傳來的 Session ID 查詢同一個 Redis，確實會產生一些問題。

例如，各個服務會依賴共用的 Session Infrastructure，必須理解 Session 的格式與驗證機制。當 Session Schema 或 Authentication Policy 變更時，也可能影響多個服務。

但這不代表使用 Cookie Session 就無法建立 Microservices。

另一種設計是：

Browser → Session Cookie → BFF / Gateway → Internal Access Token → Microservices

在這個架構中，瀏覽器與 BFF 之間使用 Cookie Session。

BFF 驗證 Session 後，再透過可信任的內部身分機制，向下游服務傳遞必要的使用者身分與授權資訊。

例如，可以使用短效、限制 Audience 與 Scope 的 JWT Access Token，讓不同的 Microservice 本地驗證 Token。

也可以在 OAuth 架構下透過 Token Exchange 處理服務間的委派需求，或使用 mTLS 驗證服務身分。

這些做法的重點，是不要讓每個 Microservice 都直接依賴瀏覽器 Session。

不過，BFF 或 Gateway 完成 Authentication，並不代表下游服務就可以省略 Authorization。

例如，Order Service 仍然必須確認使用者是否有權限讀取特定訂單；Payment Service 則需要獨立確認付款操作是否被允許。

微服務之間也需要區分：

**誰是發出請求的 Service？以及這個 Service 正代表哪一位 User 執行操作？**

這兩個身分不應混為一談。

因此，真正需要避免的不是 Cookie Session，而是讓所有服務無差別地共享同一套 Session State、盲目信任上游傳入的身分欄位，或忽略服務本身的授權責任。

## 七、所以什麼時候才真的需要雙 Token？

回到最初的問題。

我認為架構選擇應該從系統需求出發，而不是從 JWT 或 Session 哪個技術比較先進出發。

如果是單一 Web App，主要需求是維持登入狀態、支援登出與 Session 管理，傳統 Cookie Session 可能已經足夠。

如果系統有多個獨立 Resource Server，需要標準化的授權流程、跨服務 Token 驗證，或是第三方 Client 存取，就有更多理由考慮 OAuth 2.0。

如果 Client 需要在 Access Token 過期後持續取得存取權限，則可以評估 Refresh Token 是否符合需求與風險模型。

如果主要使用者是瀏覽器，而且希望避免將 OAuth Token 暴露給前端 JavaScript，OAuth BFF 也是一個值得考慮的方案。

但這些決策都需要衡量相應的工程成本。

例如，BFF 會增加一層請求處理與部署責任；Session Store 會帶來可用性與擴充方面的考量；Refresh Token Rotation 需要管理 Token Family 與並發續期；JWT 本地驗證則需要處理金鑰輪替及撤銷延遲。

不存在一種架構能在完全沒有代價的情況下，同時取得所有優點。

## 八、最後的思考

一開始，我把這個問題理解成：

「如果最後還是需要儲存狀態，那為什麼不直接使用 Session？」

但真正研究之後，我認為更值得問的是：

**哪些狀態應該由哪個元件負責？又有哪些驗證工作需要跨越服務邊界？**

Session 管理的是使用者與應用程式之間的持續登入關係。

Access Token 表達特定存取權限，讓 Resource Server 驗證請求。

Refresh Token 提供取得新 Access Token 的能力，並需要相對應的安全管理機制。

BFF 則可以在瀏覽器與後端資源之間建立一道邊界，隔離不同的憑證與安全責任。

這些技術並不是互相排斥的選擇，也不是每個系統都必須全部導入。

**Stateless 不是 JWT 架構的必然結果；雙 Token 也不是所有 Web Authentication 的標準答案。**

真正重要的是，為了哪些需求接受了哪些複雜度，以及這些複雜度是否有實際價值。

---

## References

- [RFC 6749 — OAuth 2.0 Authorization Framework](https://www.rfc-editor.org/rfc/rfc6749.html)
- [RFC 7519 — JSON Web Token](https://www.rfc-editor.org/rfc/rfc7519.html)
- [RFC 9068 — JWT Profile for OAuth 2.0 Access Tokens](https://www.rfc-editor.org/rfc/rfc9068.html)
- [RFC 9700 — OAuth 2.0 Security Best Current Practice](https://www.rfc-editor.org/rfc/rfc9700.html)
- [RFC 10017 — OAuth 2.0 for Browser-Based Applications](https://www.rfc-editor.org/rfc/rfc10017.html)
- [RFC 8693 — OAuth 2.0 Token Exchange](https://www.rfc-editor.org/rfc/rfc8693.html)
- [OWASP Microservices Security Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Microservices_Security_Cheat_Sheet.html)
- [OWASP Session Management Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html)

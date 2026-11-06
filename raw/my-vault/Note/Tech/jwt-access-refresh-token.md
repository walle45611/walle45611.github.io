---
blog: true
blog_title: JWT 登入用一顆 Cookie 就好嗎？為什麼要拆成 Access Token 和 Refresh Token
blog_date: 2026-11-06
blog_url: https://blog.walle4561.com/articles/posts/jwt-access-refresh-token/
---

使用 JWT Authentication 已經有一段時間了，但我一直有個疑問：**既然瀏覽器能用一顆 Cookie 帶上 Token，為什麼還要多做一個 Refresh Token，甚至把兩種 Token 分開存？**

一顆 Cookie 確實可以維持登入。拆成兩種 Token 的理由，是我想讓每天拿來打 API 的憑證早點過期，同時讓使用者可以繼續登入。換個角度問會更清楚：**如果有人拿走一份憑證，他能做什麼？能用多久？能不能拿它換新的憑證？**

## 一顆 Cookie 裡面放的是什麼？

Cookie 決定瀏覽器怎麼保存資料、在哪些請求附帶它。裡面可以放一個隨機的 Session ID，也可以放一個簽名過的 JWT。這兩種做法看起來都只有一顆 Cookie，後端驗證方式卻不同。

- **放 Session ID**：後端用 ID 查 Session Store，確認登入狀態。登出時停用這筆 Session，之後的請求即使帶著舊 ID，也會被拒絕。
- **放 JWT**：API 可以靠簽章和必要的 Claims（例如 `iss`、`aud`、`exp`）驗證。如果 API 只做這些檢查，刪掉瀏覽器的 Cookie 後，先前被複製的 JWT 仍可能用到過期。把 JWT 設得越久，單一外洩憑證可被重放的時間就越長。 [RFC 7519 §4](https://www.rfc-editor.org/rfc/rfc7519.html#section-4)

如果我的網站原本就會在每次請求查 Session，後端可以自己管理到期、續期與撤銷。一顆 Session Cookie 就能把這些事做好。拆 Token 的價值，在於讓 API 的驗證跟續期分開。

## 一份拿來打 API，一份拿來續期

| 憑證 | 交給誰 | 只外洩這份憑證時 |
| --- | --- | --- |
| Access Token | 提供資源的 API | 如果是 Bearer Token，持有者可以在到期前，對指定 API 執行權限允許的操作 |
| Refresh Token | 負責簽發 Token 的服務 | 如果換發條件允許，持有者可能反覆取得新的 Access Token |

API 只接受 Access Token。簽發服務則用 Refresh Token 和伺服器掌握的狀態決定要不要續期。因為一般 API 請求用不到 Refresh Token，這份長期憑證可以只出現在續期路徑。 [RFC 6749 §1.5](https://www.rfc-editor.org/rfc/rfc6749.html#section-1.5) · [RFC 9700 §4.14](https://www.rfc-editor.org/rfc/rfc9700.html#section-4.14)

假設 Access Token 的效期是 **10 分鐘**，登入最多維持 **7 天**。如果 Access Token 在簽發後第 8 分鐘被複製，攻擊者大約還有 2 分鐘可以用它呼叫 API，單憑它無法續期。若改成同一個 Bearer JWT 同時負責打 API 和維持 7 天登入，API 又只驗證簽章和 Claims，那麼在第 8 分鐘外洩時，這份 JWT 仍可能用接近 7 天。這裡的時間只是拿來比較兩種設計。

## 放在 Memory 和 HttpOnly Cookie，有什麼差別？

以同源的前端與後端為例，登入後可以把短效 Access Token 放在前端記憶體，用 `Authorization: Bearer …` 呼叫 API。Refresh Token 由後端透過 `HttpOnly`、`Secure` Cookie 設定，依部署設定合適的 `SameSite`，並用 `Path=/auth` 讓瀏覽器把 Cookie 附在續期、登出等 `/auth` 路徑下的請求。Access Token 過期或頁面重新整理後，前端可以呼叫 `/auth/refresh`，由瀏覽器帶上 Cookie 來取得新的 Access Token。

記憶體裡的 Token 通常會隨頁面重新載入消失，`localStorage` 裡的值則會留著，而且同源 JavaScript 有固定的介面可以讀取。遇到 XSS 時，惡意程式仍可能攔截請求、取得 Access Token，或直接借用瀏覽器發出已登入的請求。 [RFC 10017 §5](https://www.rfc-editor.org/rfc/rfc10017.html#section-5) · [RFC 10017 §8.4](https://www.rfc-editor.org/rfc/rfc10017.html#section-8.4)

`HttpOnly` 擋的是 JavaScript **直接讀出 Cookie 的值**。如果惡意腳本能呼叫同源的續期端點，而端點會把新 Access Token 回傳給前端，它仍可能讀到回應。Cookie 會由瀏覽器自動附帶，所以續期端點也要按部署情況處理 CSRF，例如檢查 `Origin` 或使用 CSRF Token。 [RFC 10017 §5](https://www.rfc-editor.org/rfc/rfc10017.html#section-5) · [RFC 10017 §6.1.3.3](https://www.rfc-editor.org/rfc/rfc10017.html#section-6.1.3.3)

兩種 Token 也可以都用 Cookie 傳送，例如各用一顆 Cookie，讓 Refresh Token 只跟續期與登出請求走。即使儲存方式相同，只要 API 與續期端點各自驗證該用的憑證，兩種 Token 的用途與效期仍然可以分開。

## 續期跟登出，各自能控制什麼？

Refresh Token Rotation 的做法是，每次成功續期都發一個新的 Refresh Token，並讓舊的失效。如果已經用過的舊 Token 又出現，簽發端可把它視為可能的重放，撤銷同一授權下仍有效的 Refresh Token，要求重新登入。OAuth 2.0 的安全最佳實務要求，對於有發 Refresh Token 的 public client，要用 rotation 或把 Token 綁定到持有者（sender-constrained）。 [RFC 9700 §4.14.2](https://www.rfc-editor.org/rfc/rfc9700.html#section-4.14.2)

登出時，清掉瀏覽器的憑證並撤銷伺服器端的 Refresh Token，可以停止後續換發。至於已簽發的 Access Token，如果 API 只驗簽章和 Claims，就仍可用到過期。需要立即擋下舊 Token 時，API 還要查撤銷狀態，例如檢查 JWT 的 `jti` 是否在 denylist。 [RFC 7009 §2.1](https://www.rfc-editor.org/rfc/rfc7009.html#section-2.1)

## 這跟 OAuth 的 Flow 有什麼關係？

JWT 描述的是 **Token 長什麼樣**。OAuth 2.0 的 Grant Type 描述的是 **Client 如何取得 Access Token**。例如 Authorization Code Grant 用授權碼交換 Token，PKCE 保護授權碼的交換。若授權伺服器也簽發了 Refresh Token，後續可以用 `grant_type=refresh_token` 換新的 Access Token。Access Token 可以是 JWT，也可以是由 API 透過其他方式驗證的不透明字串。 [RFC 6749 §4.1](https://www.rfc-editor.org/rfc/rfc6749.html#section-4.1) · [RFC 6749 §6](https://www.rfc-editor.org/rfc/rfc6749.html#section-6) · [RFC 9700 §2.1.1](https://www.rfc-editor.org/rfc/rfc9700.html#section-2.1.1)

回到一開始的問題，如果我做的是單一網站，而且希望後端在每次請求都能確認登入狀態，一顆 Session Cookie 就很直接。如果我希望多個 API 可以各自驗證短效 Token，又希望使用者不用每隔十分鐘重新登入，我才會把續期交給另一份可以單獨撤銷的憑證。

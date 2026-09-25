# JavaScript Visualized — Event Loop, Web APIs, (Micro)task Queue

- source: `raw/web-clipper/JavaScript Visualized - Event Loop, Web APIs, (Micro)task Queue.md`
- source link: https://www.youtube.com/watch?v=eiC58R16hb8&t=139s
- original title: JavaScript Visualized - Event Loop, Web APIs, (Micro)task Queue
- author: Lydia Hallie
- published: 2024-04-04
- source_created: 2026-09-21
- ingested_at: 2026-09-22
- type: YouTube transcript summary

## Summary

影片以瀏覽器環境拆解 call stack、Web APIs、task queue 與 microtask queue 的協作。非同步 API 將等待交由瀏覽器處理，結果則排入適當佇列；callback 不會任意插入正在執行的同步程式。

## Key Points

- **Call stack（0:32–2:01）**：影片聚焦單一執行緒與單一堆疊；長時間同步計算會阻塞後續程式。Web APIs 的存在不代表任意 JavaScript 計算都會自動移往背景。
- **Callback-based API（2:47–4:39）**：以 geolocation 為例，呼叫先註冊 callback 並啟動非同步工作；取得結果後 callback 才等待排程。
- **Timer（4:39–6:22）**：`setTimeout` 的 delay 不保證實際執行時間；計時完成後，callback 仍需等待執行中的程式與排程。
- **Microtasks（6:22–8:40）**：影片列出 Promise 的 `then`／`catch`／`finally` reaction、`await` 後續程式、`queueMicrotask` 與 MutationObserver callback。其教學模型是在同步工作結束後，先清空 microtask queue，再執行下一個 task。
- microtask 可再排入 microtask；若無限產生，會讓 task queue 一直無法獲得執行機會。
- **Promisifying（8:40–8:57）**：用 Promise 包裝 callback API 可調整使用介面；影片示範以 `resolve`／`reject` 接成功與失敗 callback。

## Worked Ordering From the Video

8:57–10:52 的題目輸出為 **5 → 1 → 3 → 4 → 2**：同步 log 5 先執行；已 fulfilled Promise 的 reaction 印出 1；第一個 microtask 印出 3，並新增印出 4 的 microtask；清空後才輪到 timer task 印出 2。此處保留逐字稿的順序解釋，未臆造未附於剪藏的完整畫面程式碼。

## Limits and Alignment

來源是瀏覽器執行模型的入門解說，不是 HTML 排程規範的完整描述；不據此推導不同 task source 的全域 FIFO、渲染時機或 Node.js 的各個 phase。影片對 Node tick-depth 設定的猜測未納入知識結論。

可與既有作業系統同步筆記對照執行順序，但 event loop 排程不等同多執行緒鎖或共享記憶體同步。

- [javascript-event-loop](../concepts/javascript-event-loop.md)
- [作業系統：競爭條件與同步](./blog-race-condition-and-synchronization.md)

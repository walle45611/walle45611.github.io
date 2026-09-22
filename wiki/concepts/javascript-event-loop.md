# JavaScript Event Loop

## Current View

目前來源聚焦瀏覽器：同步 JavaScript 在 call stack 執行，非同步 Web API 處理外部等待，完成後透過 task 或 microtask 排程後續程式。要解釋輸出順序，需分清「啟動工作」「結果就緒」與「callback 實際執行」。

## Stable Conclusions Within This Scope

- Timer 到期不等於 callback 當下執行；同步工作仍可能延後它。
- Promise reaction 與 `queueMicrotask` 使用 microtask；在來源的教學模型中，處理下一個 task 前會先清空 microtask。
- Microtask 內新增的 microtask 也要處理，因此無限續排可能使其他 task 飢餓。
- 非同步 API 的等待可交由瀏覽器，但長時間同步計算仍會阻塞目前執行緒。

## Boundaries and Adjacent Concepts

這是 browser runtime 內的排程，不直接描述 Node.js phases、跨執行緒同步或完整渲染流程。它也不等同分散式訊息佇列：後者另有投遞、ack、retry 與失敗恢復語義；這個區分是知識庫跨來源對照。

- [cloudflare-queue-consumer-modes](./cloudflare-queue-consumer-modes.md)：對照 runtime callback 排程與外部訊息消費協定。
- [作業系統：競爭條件與同步](../summaries/blog-race-condition-and-synchronization.md)：對照共享資料與臨界區問題。

## Sources

- [javascript-visualized-event-loop](../summaries/javascript-visualized-event-loop.md)

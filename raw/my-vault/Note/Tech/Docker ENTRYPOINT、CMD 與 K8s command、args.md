---
blog: true
blog_title: 'Docker ENTRYPOINT、CMD 與 K8s command、args'
blog_date: '2026-10-29'
blog_url: https://blog.walle4561.com/articles/posts/docker-entrypoint-cmd-k8s-command-args/
blog_tag: OS & Networking
blog_topic: K8s
---

# Docker ENTRYPOINT、CMD 與 K8s command、args

> [!tip] 先記這句
> `ENTRYPOINT` 決定主程式，`CMD` 提供可被取代的預設命令或參數；K8s 的 `command` 對應 ENTRYPOINT，`args` 對應 CMD。

## 一個例子看懂覆寫

```dockerfile
ENTRYPOINT ["sleep"]
CMD ["5"]
```

| 執行方式 | 實際執行 |
|---|---|
| `docker run sleeper` | `sleep 5` |
| `docker run sleeper 10` | `sleep 10`：取代 CMD |
| `docker run --entrypoint echo sleeper hello` | `echo hello`：換掉主程式 |

以上以 exec form（JSON 陣列）為準。只有 `CMD ["sleep", "5"]`、沒有 ENTRYPOINT 時，`docker run sleeper 10` 會嘗試執行 `10`，不會自動變成 `sleep 10`。

## 在 K8s 怎麼寫

```yaml
# Pod 的 spec.containers 項目
- name: sleeper
  image: sleeper
  command: ["sleep"]
  args: ["10"]
```

兩者都省略就沿用 image 的預設；只改 `args` 就保留原 ENTRYPOINT、取代預設參數。指定 `command` 而省略 `args` 時，不要假定原 image 的 CMD 仍會附加。

## 自己做 image 要不要寫？

- 基底 image 有適合的啟動設定，可以沿用；不需要在 `docker run` 或 Pod 裡重複寫。
- 做固定用途的工具：`ENTRYPOINT` 固定程式，`CMD` 放預設參數。
- 一般應用服務：`CMD ["python", "app.py"]` 也很清楚，不必為了形式強拆兩個指令。
- 沒有任何可用的預設啟動命令，就必須在執行時提供；**build 成功不代表容器知道要跑什麼**。

> [!warning] 考前易錯
> `RUN` 在建置 image 時執行；CMD／ENTRYPOINT 在啟動容器時生效。exec form 不自動展開 `$VAR`、管線或 `&&`；需要 shell 時明確使用 `sh -c`。啟動腳本最後用 `exec` 交棒，讓主程式直接接收停止訊號。

相關：[[K8s 中的 Pod 是什麼]]、[[K8s Container Runtime 與 dockershim 遷移]]。

## 參考

- [Dockerfile：CMD 與 ENTRYPOINT](https://docs.docker.com/reference/dockerfile/#understand-how-cmd-and-entrypoint-interact)
- [Kubernetes：容器的 command 與 args](https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/)

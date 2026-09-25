# K8s HA、Quorum 與 Split-brain

Split-brain 是分散式系統因網路分割而出現互相衝突的 active side，不專指 Kubernetes master。對使用 Raft 的 datastore，提交新寫入需要多數 voting members，少數派不能自行形成合法的提交多數。

$$
\mathrm{quorum}=\lfloor N/2\rfloor+1
$$

| Voting members | Quorum | 可容忍同時故障數 |
|---|---|---|
| 1 | 1 | 0 |
| 2 | 2 | 0 |
| 3 | 2 | 1 |
| 4 | 3 | 1 |
| 5 | 3 | 2 |

避免衝突提交的是 quorum 與共識協定；使用奇數能以較少成員取得相同故障容忍能力。Control-plane Node 數量不能直接等同於 datastore voting members，必須確認實際部署。

MicroK8s 的 HA datastore 使用 dqlite；worker-only 節點不會增加 control-plane HA 能力。不要把 dqlite 的操作方式寫成 etcd 的操作方式。參考：[MicroK8s High Availability](https://microk8s.io/docs/high-availability/)。

原對話決定先練 CNI 網路，因此這裡保留概念，不把故障注入列為目前網路 Lab 的必要步驟。未來驗證時可觀察正常、失去一個 voter、失去 quorum，以及 1／2 網路分割的寫入行為。控制平面失去 quorum 不表示現有容器立即消失，但排程與需要 datastore 的控制操作會受影響。

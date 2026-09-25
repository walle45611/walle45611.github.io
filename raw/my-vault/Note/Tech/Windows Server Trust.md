---
Type:
  - windows Server AD
---
## 概述

兩個domain必須要有信任關西trust relationship，才可以存取對發的網域資源，加入到新的ADDS網域會自動信任上一層的父系網域，父系網域也會信任下層的，這些具有two-way transitive，稱為kerberos trust，網域信任也可以是單向信任
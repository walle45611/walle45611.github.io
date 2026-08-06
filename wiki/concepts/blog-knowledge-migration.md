# Blog 文章納入 LLM Wiki

- source: `blog/source/_posts/`
- ingested_at: 2026-08-03
- type: concept

## Scope

這個概念頁記錄舊 Blog Repo 的文章如何納入 LLM Wiki。Blog Repo 保留為原始文章與程式碼的來源；LLM Wiki 保存可搜尋、可交叉連結的摘要，後續閱讀與維護以 Wiki 頁面為主。

## Source Boundary

- 原始文章位於 `blog/source/_posts/`，本次只讀取與複製其知識內容，沒有修改 Blog Repo。
- 摘要中的部署指令、版本、套件與環境描述屬於原文章在發布時的內容，不應直接視為目前最新操作規格。
- 需要完整程式碼、圖片或原始上下文時，仍應回到對應的 Blog source。

## Imported Collections

### 基礎資料結構、演算法與競賽

- [blog-algorithm-complexity](../summaries/blog-algorithm-complexity.md)
- [blog-binary-trees-1](../summaries/blog-binary-trees-1.md)
- [blog-codeforces-0815](../summaries/blog-codeforces-0815.md)
- [blog-direct-image](../summaries/blog-direct-image.md)
- [blog-leetcode-3106](../summaries/blog-leetcode-3106.md)
- [blog-queue-1](../summaries/blog-queue-1.md)
- [blog-stack-1](../summaries/blog-stack-1.md)
- [blog-tree-introduc](../summaries/blog-tree-introduc.md)
- [blog-uva-1586-227](../summaries/blog-uva-1586-227.md)
- [blog-uva-272-340-0816](../summaries/blog-uva-272-340-0816.md)
- [blog-virtual-judge-hdu-1232](../summaries/blog-virtual-judge-hdu-1232.md)
- [blog-virtual-judge-p2249](../summaries/blog-virtual-judge-p2249.md)

### 系統、網路與部署

- [blog-domjudge-install-guide](../summaries/blog-domjudge-install-guide.md)
- [blog-django-uwsgi-nginx](../summaries/blog-django-uwsgi-nginx.md)
- [blog-induction-ldap](../summaries/blog-induction-ldap.md)
- [blog-isms-context-leadership-planning](../summaries/blog-isms-context-leadership-planning.md)
- [blog-k8s-introduction-pods](../summaries/blog-k8s-introduction-pods.md)
- [blog-race-condition-and-synchronization](../summaries/blog-race-condition-and-synchronization.md)
- [blog-setup-openssh-server-in-windows-with-guacamole-sftp](../summaries/blog-setup-openssh-server-in-windows-with-guacamole-sftp.md)
- [blog-windows-l2tp-connect-error-720](../summaries/blog-windows-l2tp-connect-error-720.md)
- [microk8s-production-deployment-guide](../summaries/microk8s-production-deployment-guide.md)
- [nuitka-and-docker-for-high-performance-and-secure-python-deployment](../summaries/nuitka-and-docker-for-high-performance-and-secure-python-deployment.md)

### 開發工具與工作流程

- [blog-git-worktree](../summaries/blog-git-worktree.md)
- [blog-introduce-vim](../summaries/blog-introduce-vim.md)
- [blog-neovim-settings](../summaries/blog-neovim-settings.md)
- [blog-setting-hexo](../summaries/blog-setting-hexo.md)
- [blog-setting-hexo-ga](../summaries/blog-setting-hexo-ga.md)
- [blog-style-guide-for-python-code](../summaries/blog-style-guide-for-python-code.md)
- [blog-vscode-extension-profile](../summaries/blog-vscode-extension-profile.md)
- [vllm-gemma-4-lora-two-pitfalls](../summaries/vllm-gemma-4-lora-two-pitfalls.md)

## Maintenance Boundary

新的 Blog 文章若要成為長期知識，應先在這裡建立摘要、保留來源路徑，再視內容是否值得整合進既有概念頁；不要把 Blog Repo 的整份 Hexo 專案當成 LLM Wiki 的規則或來源區。

## Related Concepts

- [agentic-knowledge-base-maintenance](./agentic-knowledge-base-maintenance.md)


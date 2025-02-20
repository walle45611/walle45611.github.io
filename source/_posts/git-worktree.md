---
title: 使用 Git Worktree 提高開發效率與維持線性 Commit Log
date: "2025-02-20"
categories:
    - Git
    - 開發流程
    - 軟體工程
tags:
    - Git Worktree
    - Git Flow
    - 開發效率
abbrlink: "4e9"
---

# 使用 Git Worktree 提高開發效率與維持線性 Commit Log

最近在專案開發中，我發現自己經常遇到這些問題：

- 修 Bug 時要來回切換分支，結果 `stash` 來 `stash` 去，不小心還搞丟了一些變更。
- 切換分支之後還要等一堆 `npm install` 或是 `docker-compose up`，光是環境準備就花掉大半時間。
- `git log` 變得又亂又複雜，充滿 merge commit，視覺污染嚴重。

結果後來發現 **Git Worktree** 這個超級好用的工具，讓我可以同時開多個分支開發，完全不用 `stash`，而且還能維持漂亮的 Git 歷史！

這篇文章我會用比較口語的方式來介紹 **Hotfix + Worktree** 的 Git 流程，讓你開發更順、commit log 更乾淨！🚀

<!--more-->

---

## **我的 Git 分支管理策略**

### **1. Hotfix：直接在 `develop` 上開臨時分支修 Bug**

假設今天老闆突然衝過來說：「欸這個 Bug 超緊急，馬上修！」
這時候，我們不會直接改 `develop`，而是用 `git worktree` 檢出 `develop`，然後在裡面開 `hotfix` 分支來修復問題。

#### **步驟：**

```bash
# 建立裸倉庫（bare repository）
git clone --bare <repo-url> project.git

# 新增 develop 的 worktree
git --git-dir=project.git worktree add develop develop

# 進入 worktree
cd develop
git checkout -b hotfix/some-bugfix
```

然後開始修 Bug，修完之後發 PR，等合併後就刪掉這個分支。

```bash
# 修好 Bug，提交 PR
git commit -am "Fix some critical bug"
git push origin hotfix/some-bugfix
```

PR 合併後，刪除本地與遠端的 hotfix 分支：

```bash
git checkout develop
git branch -d hotfix/some-bugfix
git push origin --delete hotfix/some-bugfix
```

這樣就確保 **hotfix 只是短暫存在**，修完就閃，完全不影響主要的 `develop` 分支。

---

### **2. 新功能開發：使用 `git worktree` 並行作業**

寫新功能的時候，如果還在用 `git checkout` 來回切換，那真的太折騰自己了！
`git worktree` 讓我們 **同時開啟多個分支的工作環境**，完全不用 `stash` 來回切換。

#### **步驟：**

```bash
# 設定 worktree
mkdir workspace
cd workspace
git --git-dir=../project.git worktree add feature-branch feature/new-feature
```

這樣你就可以在 `workspace/feature-branch` 目錄寫新功能，而 `project.git` 只是一個純儲存的裸倉庫，不會干擾開發環境！

進入 `feature-branch` 開始開發：

```bash
cd feature-branch
# 寫新功能...
git commit -am "Implement new feature"
```

功能完成後，推上遠端並發 PR：

```bash
git push origin feature/new-feature
```

PR 合併後，就可以刪除這個 worktree：

```bash
cd ..
git worktree remove feature-branch
```

這樣就確保我們的 Git 紀錄乾淨俐落，每個新功能都有獨立的開發環境，而專案目錄下只會有 worktree 產生的版本，不會干擾裸倉庫！

---

## **為什麼這樣做能保持線性的 Git Log？**

1. **Hotfix 是短期存在的分支**：
    - 直接從 `develop` 建立，PR 合併後刪除，確保 commit log 不會有多餘的 merge commit。
2. **新功能開發使用 `worktree` 搭配裸倉庫**：
    - 避免 `checkout` 來回切換，確保每個新功能都在獨立的工作環境。
    - 倉庫目錄不會被開發環境影響，確保整潔。
3. **PR 維持乾淨的 commit log**：
    - 在 PR 合併時使用 `squash and merge`，確保 `develop` 上的歷史紀錄保持清晰。

這樣的做法可以讓專案維持良好的 Git 歷史，開發流程也更加順暢！

---

## **結論**

透過這種 **Hotfix + Worktree + Bare Repository** 的開發方式，我們可以：
✅ 確保 `develop` 分支的歷史紀錄清晰、線性。
✅ 避免頻繁 `stash` 切換，開發效率更高。
✅ 使用 `worktree` 讓新功能與主分支分離，不影響彼此。
✅ **裸倉庫確保專案目錄保持簡潔，不被開發文件污染**。
✅ 快速刪除不必要的分支，確保遠端與本地倉庫整潔。

這樣的 Git 工作流不僅適合個人開發，也適合團隊協作，推薦給所有開發者使用！🚀

---

**參考資源：**

- [Git Worktree 官方文件](https://git-scm.com/docs/git-worktree)
- [Miniasp 部落格-使用 git worktree 管理一個本地儲存庫下的多個工作目錄副本-保哥](https://blog.miniasp.com/post/2023/10/29/git-worktree-manage-multiple-working-directories)
- [YouTube-Git's Best And Most Unknown Feature-ThePrimeagen
  ](https://www.youtube.com/watch?v=2uEqYw-N8uE&t=433s)

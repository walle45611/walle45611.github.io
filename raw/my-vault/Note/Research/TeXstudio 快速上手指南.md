# TeXstudio 快速上手指南

給教授快速閱讀與修改論文使用。以下以 Windows 的 TeXstudio 畫面為例，假設已安裝 MiKTeX 與 TeXstudio，並收到完整的論文資料夾。

> [!tip] 先記住這個流程
> 開啟 `main.tex` → 找到要修改的章節 → 修改文字 → 儲存全部 → 按 `F5` 編譯並看 PDF。想從 PDF 找回文字位置，在 PDF 上按住 `Ctrl` 並點一下該段文字。

安裝與範本請看 [[IEEE LaTeX 範本安裝與使用指南]]；公式、表格與引用語法請看 [[LaTeX 論文協作使用指南]]。

## 1. 開啟論文

在 TeXstudio 選擇 **File → Open…**，或按 `Ctrl + O`。

![[Assets/Note/Research/TeXstudio 快速上手指南/TeXstudio-開啟檔案選單.png|760]]

進入論文資料夾，先選擇 **`main.tex`**。以下截圖的路徑是 `cmu-sage-ai/docs/paper/`；實際位置依收到的資料夾而定。

![[Assets/Note/Research/TeXstudio 快速上手指南/TeXstudio-選擇主文件.png|760]]

這份論文的檔案用途如下：

| 檔案或資料夾 | 用途 |
|---|---|
| `main.tex` | 整篇論文的編譯入口 |
| `sections/` | 各章節正文，例如 Introduction、Results |
| `media/` | 論文圖片 |
| `cmu-llm-wiki.bib` | 參考文獻資料 |
| `ieeecolor.cls`、`generic.sty` | 期刊版面與樣式設定，通常保留原樣 |

需要修改章節時，再開啟 `sections/` 中對應的 `.tex`。請保留完整資料夾，避免缺少圖片、文獻或樣式檔。

## 2. 第一次編譯要求安裝套件

若 MiKTeX 跳出 **Package Installation**，表示論文需要的套件尚未安裝。以下範例缺少 `cite.sty`，可保持網路連線並按 **Install**，等待安裝完成；必要時再按 `F5`。

![[Assets/Note/Research/TeXstudio 快速上手指南/TeXstudio-缺少套件安裝.png|520]]

這個畫面是補裝 LaTeX 套件，不是要求修改正文。MiKTeX 初次安裝畫面與更新方式請看 [[IEEE LaTeX 範本安裝與使用指南#Windows：MiKTeX]]。

## 3. 修改文字並看結果

![[Assets/Note/Research/TeXstudio 快速上手指南/TeXstudio-編輯與PDF預覽.png|1000]]

畫面左側是文件結構，中間是可修改的 LaTeX 原始碼，右側是排版後的 PDF，下方 **Messages / Log** 顯示編譯訊息。

1. 從上方分頁或左側文件結構找到要修改的章節。
2. 在中間編輯區修改正文。先保留 `\section{...}`、`\cite{...}`、`\label{...}` 等指令及大括號。
3. 選擇 **File → Save All**（`Ctrl + Shift + S`）儲存全部檔案。
4. 回到 `main.tex` 分頁，按 **`F5`（Build & View）**，重新產生並查看 PDF。

**按 F5 編譯並查看 PDF 示範**（點開動畫可放大查看）：

![[Assets/Note/Research/TeXstudio 快速上手指南/TeXstudio-F5-編譯與預覽.gif|1000]]

文字修改後需要重新編譯，PDF 才會更新。`F5` 請在編輯器主視窗操作；獨立 PDF 視窗中的同名按鍵可能有不同用途。

截圖下方顯示 `Process exited normally`，表示該次編譯程序正常結束；仍可檢查 PDF 與警告訊息。若出現錯誤，先查看下方第一個錯誤，再檢查剛才修改的文字。

## 4. 像 Overleaf 一樣定位 PDF 與原始碼

TeXstudio 的 **SyncTeX** 可以在原始碼與 PDF 的對應位置之間跳轉。

| 想做的事 | 操作 |
|---|---|
| 從原始碼找到 PDF 的位置 | 在編輯區的文字上按 `Ctrl + 滑鼠左鍵`；或按右鍵選 **Go To PDF** |
| 從 PDF 找到要修改的原始碼 | 在 PDF 段落上按 `Ctrl + 滑鼠左鍵`；或按右鍵選 **Jump to Source** |

例如教授在 PDF 看到某句需要修改，可先 `Ctrl + 點擊` 該句，回到原始碼修改，儲存並按 `F5`，再檢查結果。定位通常落在對應文字附近。

**Ctrl + 點擊操作示範**（點開動畫可放大查看）：

![[Assets/Note/Research/TeXstudio 快速上手指南/TeXstudio-Ctrl-Click-同步定位.gif|1000]]

如果無法跳轉，先確認已成功編譯，且使用 TeXstudio 的內建 PDF 檢視器。編譯需要啟用 `-synctex=1`；上方截圖的編譯訊息已包含此選項。若改動很多文字，重新編譯後再定位。

操作依據：[TeXstudio 官方 PDF 檢視與同步定位說明](https://texstudio-org.github.io/viewing.html)。

## 常用操作速查

| 操作 | Windows 快捷鍵 |
|---|---|
| 開啟檔案 | `Ctrl + O` |
| 儲存目前檔案 | `Ctrl + S` |
| 儲存全部檔案 | `Ctrl + Shift + S` |
| 編譯並查看 PDF | `F5`（編輯器主視窗） |
| 查看 PDF | `F7` |
| 搜尋文字 | `Ctrl + F` |
| 原始碼與 PDF 互相定位 | `Ctrl + 滑鼠左鍵` |

若快捷鍵經過自訂，以選單顯示為準。編譯操作可參考 [TeXstudio 官方編譯說明](https://texstudio-org.github.io/compiling.html)。

Thesaurus 是英文同義詞工具，與 PDF 定位無關；日常閱讀與修改論文可以先略過。替換學術用詞時仍需確認語境，例如 `significant` 可能涉及統計顯著性的意思。

---
blog: true
blog_title: "IEEE LaTeX 範本安裝與使用指南"
blog_date: 2026-09-26
blog_url: https://blog.walle4561.com/articles/posts/ieee-latex-template-guide/
---

# IEEE LaTeX 範本安裝與使用指南

本篇依序整理編譯環境、編輯器與 IEEE 範本的下載及安裝，最後確認範本可以編譯。Windows 與 macOS 只需選擇自己使用的系統安裝。

閱讀順序：**本篇完成安裝與下載 → [[TeXstudio 快速上手指南]] 學習軟體操作 → [[LaTeX 論文協作使用指南]] 修改論文內容**。

## 1. 安裝 LaTeX 編譯環境

### Windows：MiKTeX

Windows 建議使用 **MiKTeX**。

官方說明：[Install MiKTeX on Windows](https://miktex.org/howto/install-miktex)。

1. 從 [MiKTeX 下載頁](https://miktex.org/download) 下載並執行 **Basic MiKTeX Installer**。
2. 閱讀並接受授權條款後，選擇個人安裝（per-user）；這也是官方建議的安裝方式。安裝目錄可保留預設值。
3. 在設定頁選擇預設紙張大小，並設定缺少套件時的安裝行為：`Always` 會自動安裝、`Ask me first` 會先詢問、`Never` 則不安裝。希望編譯時自動補齊套件可選 `Always`，下載套件時需連線至網際網路。
4. 確認設定後按 `Start`，完成安裝並關閉安裝程式。
5. 開啟 **MiKTeX Console**，在 `Updates` 檢查並完成更新。
6. 重新開啟 PowerShell。

安裝畫面參考：勾選接受授權條款後按「下一步」，接著等待套件安裝完成。

![[Assets/Note/Research/IEEE LaTeX 範本安裝與使用指南/MiKTeX-授權條款.png|560]]

![[Assets/Note/Research/IEEE LaTeX 範本安裝與使用指南/MiKTeX-安裝進度.png|560]]

確認安裝：

```powershell
pdflatex --version
latexmk --version
```

如果找不到 `latexmk`，可在 MiKTeX Console 的 `Packages` 搜尋並安裝。若執行時出現 Perl 錯誤，再安裝 [Strawberry Perl](https://strawberryperl.com/)。

### macOS：MacTeX

使用 Homebrew 安裝 MacTeX：

```bash
brew install --cask mactex
```

重新開啟 Terminal 後確認：

```bash
pdflatex --version
latexmk --version
```

也可以直接從 [MacTeX](https://www.tug.org/mactex/) 下載官方安裝程式。

## 2. 下載與安裝編輯器

### TeXstudio

1. 開啟 [TeXstudio 官方網站](https://www.texstudio.org/)，下載符合自己作業系統的安裝檔。
2. 完成安裝後開啟 TeXstudio。
3. 接著準備下節的 IEEE 範本或收到的完整論文資料夾。

MiKTeX／MacTeX 負責編譯，TeXstudio 提供文字編輯與 PDF 檢視介面。開檔、儲存、編譯及 `Ctrl + 點擊` 定位操作請看 [[TeXstudio 快速上手指南]]。

### 其他選擇：VS Code

也可以安裝 [Visual Studio Code](https://code.visualstudio.com/)，並在 Extensions 安裝 **LaTeX Workshop**。使用 VS Code 開啟整個範本資料夾，不要只開啟單一 `.tex`。選擇一套編輯器即可。

## 3. 下載 IEEE 範本

不同期刊的格式可能不同，投稿時應以目標期刊提供的最新版範本為準。若已收到完整論文資料夾，可直接使用，不必另外下載範本。

1. 開啟 [IEEE Template Selector](https://template-selector.ieee.org/secure/templateSelector/publicationType)。
2. 選擇投稿類型與目標期刊。
3. 選擇 LaTeX 格式並下載。
4. 完整解壓縮範本，不要只取出 `.tex`。

常見檔案如下：

```text
IEEE-template/
├── main.tex          # 論文內容與編譯入口
├── IEEEtran.cls      # IEEE 版面設定
├── journal.sty       # 期刊補充樣式，檔名可能不同
├── references.bib    # 參考文獻
└── figures/          # 圖片
```

> [!important]
> `.cls`、`.sty`、logo 與 `.tex` 應一起保留。這些檔案控制 IEEE 的雙欄、字型、標題、圖表和參考文獻格式，通常不需要修改。

## 4. 開啟並確認範本可以編譯

使用 TeXstudio 時，開啟主文件 `main.tex`，在編輯器主視窗按 `F5` 編譯並查看 PDF。完整操作與畫面說明請看 [[TeXstudio 快速上手指南]]。確認可以產生 PDF 後，即可依 [[LaTeX 論文協作使用指南]] 修改標題、作者、摘要及正文。

**按 F5 編譯並查看 PDF 示範：**

![[Assets/Note/Research/TeXstudio 快速上手指南/TeXstudio-F5-編譯與預覽.gif|1000]]

### 使用指令編譯（選用）

先找出主文件，通常是 `main.tex`，並包含：

```latex
\documentclass{IEEEtran}
\begin{document}
...
\end{document}
```

進入範本資料夾後執行：

```bash
latexmk -pdf -interaction=nonstopmode -halt-on-error main.tex
```

成功後會產生 `main.pdf`。若主文件名稱不同，請更換指令中的檔名。

清除編譯暫存檔（以下 `-C` 也會刪除產生的 PDF）：

```bash
latexmk -C main.tex
```

## 5. 安裝與編譯常見問題

### 缺少 LaTeX 套件

Windows 安裝 MiKTeX 時可選 `Always` 自動安裝缺少套件，或選 `Ask me first` 並在提示時同意安裝；這些偏好可在安裝後調整。下載套件時需保持網路連線。套件安裝提示畫面請看 [[TeXstudio 快速上手指南#2. 第一次編譯要求安裝套件]]。

內容語法、特殊字元及引用錯誤請看 [[LaTeX 論文協作使用指南#14. 常見錯誤]]。

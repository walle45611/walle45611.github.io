# LaTeX 論文協作使用指南

本篇整理撰寫論文時最常用的 LaTeX 基本語法。安裝環境與 IEEE 官方範本的使用方式請參考 [[IEEE LaTeX 範本安裝與使用指南]]。

若要先學會開啟論文、編譯與 PDF 定位，請看 [[TeXstudio 快速上手指南]]。

## 1. 文件基本結構

一份完整的 LaTeX 文件通常包含：

```latex
\documentclass{article}

\begin{document}

\title{Paper Title}
\author{Author Name}
\date{}
\maketitle

\section{Introduction}
This is the paper content.

\end{document}
```

- `\documentclass`：指定文件類型或期刊範本。
- `\begin{document}`：正文開始。
- `\end{document}`：正文結束。
- `\maketitle`：顯示標題與作者。

使用 IEEE 範本時，請保留範本原有的 `\documentclass`。

## 2. 標題與作者

```latex
\title{Paper Title}
\author{Author Name}
```

作者單位與電子郵件通常放在 `\thanks{...}`。不同期刊的作者格式不同，建議保留範例原有結構，只替換文字。

## 3. 摘要與關鍵字

```latex
\begin{abstract}
Abstract text.
\end{abstract}

\begin{IEEEkeywords}
Keyword one, keyword two, keyword three.
\end{IEEEkeywords}
```

## 4. 段落與註解

原始檔中的單次換行通常只會被視為空格。若要建立新段落，請留一個空白行：

```latex
This is the first paragraph.

This is the second paragraph.
```

使用 `%` 加入不會顯示在 PDF 中的註解：

```latex
% This is a comment.
This text will appear in the paper.
```

## 5. 標題層級

```latex
\section{Introduction}
\subsection{Related Work}
\subsubsection{Evaluation Metrics}
```

LaTeX 會自動處理標題格式與編號，不要手動輸入章節編號。

## 6. 文字格式

```latex
\textbf{bold text}
\textit{italic text}
\texttt{file-name.tex}
\underline{underlined text}
```

顯示特殊字元時需要加上反斜線：

| 字元 | LaTeX 寫法 |
|---|---|
| `%` | `\%` |
| `&` | `\&` |
| `_` | `\_` |
| `#` | `\#` |
| `$` | `\$` |

例如：

```latex
The accuracy was 86.3\%.
Research \& Development
```

## 7. 條列內容

### 無編號條列

```latex
\begin{itemize}
\item First item.
\item Second item.
\end{itemize}
```

### 有編號條列

```latex
\begin{enumerate}
\item First step.
\item Second step.
\end{enumerate}
```

## 8. 標籤與交叉引用

使用 `\label` 建立標籤，再用 `\ref` 引用。不要手動寫死章節、圖片或表格編號。

```latex
\section{Methodology}
\label{sec:methodology}

See Section~\ref{sec:methodology}.
```

常見標籤命名：

```text
sec:methodology    章節
fig:architecture   圖片
tab:results        表格
eq:f1-score        公式
```

`~` 是不可斷行空格，可避免名稱與編號被拆到不同行。

## 9. 圖片

```latex
\begin{figure}[!t]
\centering
\includegraphics[width=\columnwidth]{figures/example.pdf}
\caption{Example figure.}
\label{fig:example}
\end{figure}
```

正文引用：

```latex
Fig.~\ref{fig:example} shows the system architecture.
```

- `\centering`：圖片置中。
- `\columnwidth`：符合單欄寬度。
- `\caption`：圖說。
- `\label`：圖片引用標籤。

橫跨雙欄的圖片可將 `figure` 改為 `figure*`，並使用 `\textwidth`。

## 10. 表格

```latex
\begin{table}[!t]
\caption{Experimental Results}
\label{tab:results}
\centering
\begin{tabular}{lcc}
\hline
Method & Accuracy & F1 \\
\hline
Baseline & 80.0\% & 0.75 \\
Proposed & 86.3\% & 0.82 \\
\hline
\end{tabular}
\end{table}
```

正文引用：

```latex
Table~\ref{tab:results} summarizes the results.
```

表格中：

- `l`：靠左欄位。
- `c`：置中欄位。
- `r`：靠右欄位。
- `&`：分隔欄位。
- `\\`：結束一列。

## 11. 數學公式

### 行內公式

```latex
The learning rate was $2\times10^{-5}$.
```

### 獨立公式

```latex
\begin{equation}
F_1 = 2\frac{PR}{P+R}
\label{eq:f1-score}
\end{equation}
```

引用公式：

```latex
Equation~\eqref{eq:f1-score} defines the F1 score.
```

常用數學語法：

```latex
x^2                    % 上標
x_i                    % 下標
x_{i+1}                % 多字元下標
\frac{a}{b}            % 分數
\sqrt{x}               % 根號
\sum_{i=1}^{n} x_i     % 總和
\times                 % 乘號
\pm                    % 正負號
\le, \ge, \neq        % 比較符號
```

## 12. 文獻引用

參考文獻通常存放在 `.bib` 檔：

```bibtex
@article{example2025,
  author  = {First Author and Second Author},
  title   = {Article Title},
  journal = {Journal Name},
  year    = {2025}
}
```

正文使用 citation key 引用：

```latex
Previous work examined this problem \cite{example2025}.
```

一次引用多篇文獻：

```latex
Several approaches have been proposed \cite{key1,key2,key3}.
```

LaTeX 會依 bibliography style 自動排列與編號，不要手動輸入 `[1]`。

IEEE 範本的主文件通常保留以下設定；若期刊範本使用其他設定，請沿用原樣：

```latex
\bibliographystyle{IEEEtran}
\bibliography{references}
```

## 13. 拆分章節檔案

較長的論文可將章節拆成多個 `.tex`：

```latex
\input{sections/introduction}
\input{sections/methodology}
\input{sections/results}
```

章節檔只需放正文：

```latex
\section{Introduction}
\label{sec:introduction}

Introduction content.
```

被 `\input` 的檔案不需要再次加入 `\documentclass`、`\begin{document}` 或 `\end{document}`。

## 14. 常見錯誤

### `Undefined control sequence`

通常是指令拼錯，或沒有載入所需套件。

### `Missing } inserted`

檢查 `{` 與 `}` 是否成對：

```latex
\textbf{Correct text}
```

### `Misplaced alignment tab character &`

一般文字中的 `&` 必須寫成 `\&`；只有表格內才用 `&` 分欄。

### 引用顯示 `[?]` 或圖表顯示 `??`

檢查 `\cite`、`\ref` 與 `\label` 的名稱，再重新編譯整份主文件。

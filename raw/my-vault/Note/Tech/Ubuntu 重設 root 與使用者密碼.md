# Ubuntu 重設 root 與使用者密碼

## 已停在 grub> 時如何處理

先按 **Esc** 嘗試返回選單；若仍在命令列，嘗試載入正常模式：

```text
insmod normal
normal
```

若出現找不到檔案等錯誤，先查看裝置與路徑：

```text
ls
set
```

`ls` 可能列出 `(hd0,gpt1)`、`(hd0,gpt2)`、`(hd0,msdos1)` 或 LVM 裝置；逐一查看實際列出的裝置，例如：

```text
ls (hd0,gpt2)/
ls (hd0,gpt2)/boot/grub/
```

**僅在已確認該分割區含有 `/boot/grub/` 時**，才依實際結果設定；以下 `(hd0,gpt2)` 只是示例：

```text
set root=(hd0,gpt2)
set prefix=(hd0,gpt2)/boot/grub
insmod normal
normal
```

若 `/boot` 是獨立分割區，GRUB 看到的路徑可能是 `/grub/`，需依 `ls` 結果調整 `prefix`。回到選單後，再依上方 Recovery Mode 步驟操作；若模組或設定檔缺失，需另外診斷開機問題。

## 透過 Recovery Mode 重設

1. 開啟 VM 的主控台／Console，讓鍵盤焦點位於 VM 畫面，再重新啟動。這一步要在開機主控台操作。
2. 依 VM 的開機模式，在韌體畫面結束、Ubuntu 開始載入前叫出 GRUB：
   - **UEFI**：反覆按 **Esc**，直到出現 GRUB 選單。
   - **傳統 BIOS**：按住 **Shift**，直到出現 GRUB 選單。

   若已看到 Ubuntu 載入畫面，需重新啟動再試；實際按鍵時機依平台而異。若出現 `grub>` 命令列，先依上方「已停在 grub> 時如何處理」返回選單。
3. 本次畫面直接顯示 **Recovery Mode**，選取該項目並按 **Enter**。若其他主機的選單不同，才先進入 **Advanced options for Ubuntu**，再選帶有 **(recovery mode)** 的核心項目。
4. 在 Recovery Menu 選擇 **root — Drop to root shell prompt**。
5. 確認已進入 Linux root shell，再將根檔案系統重新掛載成可寫入：

   ```bash
   mount -o remount,rw /
   ```

6. 依要重設的帳號執行其中一個指令：

   ```bash
   passwd root
   ```

   輸入新密碼兩次；輸入時畫面通常不顯示字元。確認出現密碼更新成功訊息。

7. 執行 `sync`，再輸入 `exit` 回到 Recovery Menu，選擇 **resume** 繼續開機；正常登入後確認新密碼可用。


## 出處

- [Ubuntu Wiki：Recovery mode](https://wiki.ubuntu.com/Recovery_mode)
- [Ubuntu Community Help：LostPassword](https://help.ubuntu.com/community/LostPassword)
- [GNU GRUB Manual：normal](https://www.gnu.org/software/grub/manual/grub/html_node/normal.html)
- [GNU GRUB Manual：GRUB only offers a rescue shell](https://www.gnu.org/software/grub/manual/grub/html_node/GRUB-only-offers-a-rescue-shell.html)

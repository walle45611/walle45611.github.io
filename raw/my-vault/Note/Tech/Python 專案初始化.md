## 📦 專案環境結構圖像構思

- 整體以流程圖呈現，強調各步驟邏輯關係與實作順序
    
- 每個區塊使用簡潔圖示輔助說明
    

---

### 🧪 Step 0: 安裝 Homebrew（macOS 用戶）

- 圖示：杯子 icon + Mac 終端機
    
- 說明：macOS 上常見的套件管理工具，可用來安裝如 `uv`、`docker` 等工具。
    
- 安裝指令：
    
    ```bash
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    ```
    
- 安裝完成後，請依終端顯示指示設定 `brew` 路徑（Intel 與 Apple Silicon 路徑不同）
    
- 驗證安裝：
    
    ```bash
    brew --version
    ```
    

---

### 🧩 Step 1: 建立 `.env` 檔案

- 圖示：文件 icon + 鍵盤輸入
    
- 說明：建立 `.env` 並填入 DB_USERNAME 等資訊
    
- 指令：
    
    ```bash
    echo "DB_USERNAME=root" >> .env
    echo "DB_PASSWORD=your_password" >> .env
    echo "DB_HOST=localhost" >> .env
    echo "DB_PORT=3306" >> .env
    echo "DB_NAME=sportdb" >> .env
    ```
    

---

### 🐳 Step 2: 安裝與啟動 Docker

- 圖示：Docker 鯨魚圖 + 綠色 Up 圓點
    
- 說明：安裝 Docker Desktop 並執行版本檢查
    
- 安裝方式：
    
    - Windows/macOS 用戶可至 [Docker 官網下載](https://www.docker.com/products/docker-desktop)
        
    - 安裝完成後請重啟系統（如有提示），並打開 Docker Desktop
        
    - **macOS 可選擇使用 Homebrew 安裝：**
        
        ```bash
        brew install --cask docker
        open /Applications/Docker.app
        ```
        
- 指令：
    
    ```bash
    docker --version
    ```
    

---

### ⚡ Step 3: 安裝 `uv`

- 圖示：閃電 icon + 瀏覽器跳轉安裝頁
    
- 說明：依據作業系統執行官方安裝指令（[前往官網](https://docs.astral.sh/uv/getting-started/installation/#__tabbed_1_2)）
    
- Windows：
    
    ```powershell
    powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/uv/install.ps1 | iex"
    ```
    
- macOS / Linux（官網指令）：
    
    ```bash
    curl -LsSf https://astral.sh/uv/install.sh | sh
    ```
    
- **macOS 可選擇使用 Homebrew 安裝：**
    
    ```bash
    brew install astral-sh/tap/uv
    ```
    

---

### 🐍 Step 4: 使用 `uv sync` 建立虛擬環境

- 圖示：虛擬盒子圖 + `(.venv)` 終端畫面
    
- 說明：執行 `uv sync` 產生虛擬環境並啟動 `.venv`
    
- 指令：
    
    ```bash
    uv sync
    
    # 啟動虛擬環境
    # Windows
    .venv\Scripts\activate.bat
    
    # macOS / Linux
    source .venv/bin/activate
    ```
    

---

### 🛠️ Step 5: 啟動 Docker 資料庫

- 圖示：資料庫圖示 + `docker-compose.yml` 文件圖
    
- 說明：使用 docker-compose 啟動 Mariadb 資料庫容器
    
- 指令：
    
    ```bash
    docker-compose up --build -d
    docker ps
    ```
    

---

### 🧱 Step 6: 使用 Alembic 建表

- 圖示：建築圖示 + 箭頭 pointing to SQL
    
- 說明：執行 Alembic 遷移指令同步資料表
    
- 指令：
    
    ```bash
    alembic upgrade head
    ```
    

---

### 🧰 Step 7: 連接資料庫

- 圖示：DataGrip 圖示 + 連線線條圖
    
- 說明：用 GUI 工具（如 DataGrip）連上 MariaDB
    
- 設定參數：
    
    ```text
    Host: localhost
    Port: 3306
    Username: <你的 DB_USERNAME>
    Password: <你的 DB_PASSWORD>
    Database: <你的 DB_NAME>
    Driver: MariaDB
    ```
    

---

### 🗂️ Step 8: Git 初始流程

- 圖示：Git 標誌 + 分支圖示
    
- 說明：包含 Git 初始化與提交流程
    
- 指令：
    
    ```bash
    git init
    echo ".env\n.venv/\n__pycache__/\n*.pyc" >> .gitignore
    git add .
    git commit -m "初始化專案環境"
    git remote add origin https://github.com/your-name/your-repo.git
    git push -u origin main
    ```
    
- 補充：若不確定該忽略哪些檔案，可使用 [gitignore.io](https://www.toptal.com/developers/gitignore) 自動產生 `.gitignore`
    

---

## 📌 備註

- `.env` 與 `.venv/` 記得加到 `.gitignore`
    
- 推薦使用 Obsidian 作為知識庫整合 Markdown 文件
# 版本V1.0
#### 此版本实现的基础功能
1. 注册登录
2. 显示题目
3. 提供编程文本框，后端可以接收rust程序
4. 后端用docker运行自动判题，并返回json格式信息
5. 已建立好数据库，测试了基本的数据库交互功能
#### 目前已知bug：
1. 无需注册也会显示登录成功
程序运行成功无报错
2. 提示按钮按设计应该变成绿色，但是仍旧显示灰色，不同状态，结果的圆形按钮都会对应不同的颜色并显示提示词）

# 注意
大家添加功能尽量单独的功能模块写成一个代码文件，不要交叉到别的模块。

# sqlx作用
在项目中，`sqlx` 是一个用于与 MySQL 数据库交互的异步库。它的主要作用是执行 SQL 查询和管理数据库操作。以下是 `sqlx` 在项目中的具体用途：

---

### **1. 数据库连接**
- `sqlx` 通过 `rocket_db_pools` 提供的连接池与 MySQL 数据库建立连接。
- 在代码中，`Db` 是一个通过 `rocket_db_pools` 定义的数据库连接池：
  ```rust
  #[derive(Database)]
  #[database("mysql_database")]
  struct Db(sqlx::MySqlPool);
  ```
- 这个连接池会在 Rocket 启动时初始化，并在路由中通过 `Connection<Db>` 注入。

---

### **2. 执行 SQL 查询**
- `sqlx` 提供了多种方法来执行 SQL 查询，例如：
  - `sqlx::query`：用于动态查询。
  - `sqlx::query!`：用于编译时验证的查询（需要 `offline` 功能）。
- 在项目中，`sqlx` 被用来执行以下操作：
  1. **创建表**：
     ```rust
     sqlx::query(
         r#"
         CREATE TABLE IF NOT EXISTS run_logs (
             id INT AUTO_INCREMENT PRIMARY KEY,
             code TEXT NOT NULL,
             created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
         )
         "#
     )
     .execute(&**db)
     .await
     .expect("Failed to create table");
     ```
     - 这段代码会在数据库中创建一个 `run_logs` 表（如果不存在），用于存储用户提交的代码。

  2. **插入数据**：
     ```rust
     sqlx::query(
         r#"
         INSERT INTO run_logs (code) VALUES (?)
         "#
     )
     .bind(&input.code) // 手动绑定参数
     .execute(&**db)
     .await
     .expect("Failed to insert code into database");
     ```
     - 这段代码会将用户提交的代码插入到 `run_logs` 表中。

---

### **3. 异步操作**
- `sqlx` 是一个异步库，支持高效的非阻塞数据库操作。
- 在项目中，所有数据库操作（如 `execute` 和 `bind`）都是异步的，使用 `.await` 来等待操作完成。

---

### **4. 编译时查询验证（可选）**
- 如果启用了 `sqlx` 的 `offline` 功能（如 Cargo.toml 中所示），`sqlx::query!` 宏可以在编译时验证 SQL 查询的正确性。
- 这需要运行 `cargo sqlx prepare` 生成查询验证数据。

---

### **5. 数据库功能总结**
在项目中，`sqlx` 的主要功能包括：
1. **管理数据库连接**：通过 `rocket_db_pools` 提供的连接池与 MySQL 数据库交互。
2. **执行 SQL 查询**：
   - 创建表（`CREATE TABLE`）。
   - 插入数据（`INSERT INTO`）。
3. **支持异步操作**：提高数据库操作的性能。
4. **编译时查询验证**（如果启用 `offline` 功能）。

---

### **扩展功能**
如果需要扩展数据库功能，可以使用 `sqlx` 实现以下功能：
1. **查询记录**：
   - 查询 `run_logs` 表中的所有记录。
   ```rust
   let logs = sqlx::query!("SELECT * FROM run_logs")
       .fetch_all(&**db)
       .await
       .expect("Failed to fetch logs");
   ```
2. **删除记录**：
   - 删除指定的记录。
   ```rust
   sqlx::query!("DELETE FROM run_logs WHERE id = ?", id)
       .execute(&**db)
       .await
       .expect("Failed to delete log");
   ```
3. **分页查询**：
   - 实现分页功能以支持大规模数据的高效访问。

---

### **总结**
`sqlx` 是项目中用于与 MySQL 数据库交互的核心库，主要用于：
- 创建和管理数据库表。
- 存储用户提交的代码。
- 提供高效的异步数据库操作。



# 模块化
将用户注册、登录、题目管理和判题功能拆分到单独的文件中实现，以便更好地组织代码和模块化管理。

---

### **项目结构**
```plaintext
src/
├── main.rs            # 主入口文件，负责启动服务
├── routes/            # 路由模块
│   ├── auth.rs        # 用户注册和登录功能
│   ├── problems.rs    # 题目管理功能
│   ├── judge.rs       # 判题功能
├── models/            # 数据模型模块
│   ├── user.rs        # 用户相关数据模型
│   ├── problem.rs     # 题目相关数据模型
│   ├── test_case.rs   # 测试用例相关数据模型
├── db.rs              # 数据库连接和初始化
```

---

### **实现步骤**

#### **1. 主入口文件：`src/main.rs`**
`main.rs` 文件负责启动服务并挂载路由。



#### **2. 数据库模块：`src/db.rs`**
负责初始化数据库连接。



#### **3. 用户功能模块：`src/routes/auth.rs`**
实现用户注册和登录功能。


#### **4. 题目管理模块：`src/routes/problems.rs`**
实现题目管理功能。



#### **5. 判题功能模块：`src/routes/judge.rs`**
实现代码提交和判题功能。



### **总结**
- **模块化实现**：
  - 用户功能在 `src/routes/auth.rs`。
  - 题目管理在 `src/routes/problems.rs`。
  - 判题功能在 `src/routes/judge.rs`。

- **主入口文件**：
  - `main.rs` 负责挂载路由和启动服务。

- **数据库模块**：
  - `db.rs` 负责初始化数据库连接。


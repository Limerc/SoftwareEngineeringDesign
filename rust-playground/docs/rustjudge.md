# Rust 在线判题系统 - 开发者文档

## 概述
本项目是一个使用 **Rust 的 Rocket 框架** 构建的轻量级在线判题系统。它允许用户注册、登录、浏览编程题目、提交 Rust 代码进行评测，并查看运行结果。后端使用 **MySQL** 进行持久化存储，并通过 **Docker** 安全地编译和运行提交的代码。

## 架构

### 后端
- **框架**：Rocket（异步）
- **数据库 ORM**：sqlx（结合 `rocket_db_pools` 使用）
- **密码哈希**：bcrypt
- **代码执行**：基于 Rust 工具链的 Docker 容器

### 前端
- 基于 **HTML/CSS/JS** 的客户端
- **语法高亮**：通过 CodeMirror 集成

---

## 后端模块

### `main.rs`
Rocket 应用程序的入口点。它挂载了路由并附加了数据库连接池：
```rust
rocket::build()
    .attach(Db::init())
    .mount("/auth", routes![auth::register, auth::login])
    .mount("/problems", routes![problems::get_problems, problems::get_problem_details])
    .mount("/judge", routes![judge::submit_code])
```

### `db.rs`
定义了与 Rocket 兼容的数据库连接池包装器：
```rust
#[derive(Database)]
#[database("mysql_database")]
pub struct Db(sqlx::MySqlPool);
```

### `models`
- `User`：定义用户的数据库模型。
- `Problem`：定义题目的元数据。
- `TestCase`：定义与题目关联的测试用例模型。

### `routes`
#### `auth.rs`
- **POST /auth/register**：注册新用户并存储哈希密码。
- **POST /auth/login**：验证用户凭据。

#### `problems.rs`
- **GET /problems**：获取可用题目列表。
- **GET /problems/<id>`**：根据 ID 获取题目详情。

#### `judge.rs`
- **POST /judge**：接收题目 ID 和代码。对于每个测试用例：
  1. 将用户代码写入 user_code.rs
  2. 使用 Docker 容器编译并运行代码
  3. 将实际输出与预期输出进行比较

---

## 前端细节

### 页面与元素
- 题目列表面板（页面加载时自动加载）
- 题目详情部分（选择题目后显示）
- 代码编辑器（CodeMirror 替换 `<textarea>`）
- 提交按钮触发对后端的请求
- 结果容器显示服务器响应

### CodeMirror 集成
- 通过 CDN 在 `index.html` 中添加
- JS 初始化将 `textarea` 替换为增强的编辑器

---

## Docker 要求
确保已安装 Docker，并且主机可以运行以下命令：
```bash
docker run --rm -v /tmp/user_code.rs:/user_code.rs rust:latest sh -c "rustc /user_code.rs && ./user_code"
```
注意：必须在具有适当权限的情况下测试此命令。

---

## 环境配置
配置位于 `Rocket.toml` 文件中：
```toml
[default]
address = "x.x.x.x"
port = 8000

[default.databases.mysql_database]
url = "mysql://joe:<数据库密码>@127.0.0.1/learn_rocket"
```

---

## 安全注意事项
- 所有用户密码在存储前都使用 bcrypt 进行哈希处理。
- 每次提交的 Docker 运行都是隔离的，以防止系统被破坏。
- 在生产环境中建议增加沙箱和资源限制。

---

## 未来改进
- 添加用户会话和 JWT 支持
- 保存提交历史和评测日志
- 使用现代框架改进前端 UI
- 通过每个镜像配置支持多种语言
- 使用异步文件 I/O 和更好的 Docker 编排（例如容器池）

---

## 作者
由来自广东省的计算机科学专业学生开发。

欢迎贡献或提出问题！
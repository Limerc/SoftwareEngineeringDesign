# SoftwareEngineeringDesign
软件工程大作业
# 版本v0.1
### 此版本功能仅实现了浏览器输入rust代码，后端docker运行返回代码运行结果
### 本模块负责人：ws
以下是这些文件和文件夹的用途说明：

---

### **1. `Cargo.lock`**
- **用途**：记录项目依赖的具体版本。
- **作用**：确保项目在不同环境中使用相同的依赖版本，保证构建的一致性。
- **注意**：通常不需要手动修改，由 `cargo` 自动生成和管理。

---

### **2. `Cargo.toml`**
- **用途**：Rust 项目的配置文件。
- **作用**：
  - 定义项目的元信息（如名称、版本、作者等）。
  - 声明项目的依赖项及其版本。
- **示例内容**：
  ```toml
  [package]
  name = "rust-playground"
  version = "0.1.0"
  edition = "2021"

  [dependencies]
  rocket = { version = "0.5.0-rc.2", features = ["json"] }
  ```

---
### **3. `Rocket.toml`**
- **用途**：Rocket 框架的配置文件。
- **作用**：
  - 配置 Rocket 框架的运行参数，例如端口、环境（开发/生产）等。
- **示例内容**：
```toml

  [default]
address = "x.x.x.x"#部署主机IP
port = 8002  #端口，填个不冲突的就好

[default.databases.mysql_database]
url = "mysql://<数据库管理员名>:<密码>@127.0.0.1/<数据库名>"

```

---


- **核心文件**：
  - `Cargo.toml`：项目配置文件。
  - `src/`：存放源代码。
  - `static/`：存放前端静态资源。
- **辅助文件**：
  - `Rocket.toml`：Rocket 框架配置。
  - `Cargo.lock`：依赖版本锁定文件。


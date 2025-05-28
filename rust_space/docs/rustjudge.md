# Rust Online Judge System - Developer Documentation

## Overview
This project is a lightweight online judge system built using the **Rocket framework** in Rust. It enables users to register, login, browse programming problems, submit Rust code for evaluation, and view results. The backend uses **MySQL** for persistent storage and **Docker** to securely compile and run submitted code.

## Architecture

### Backend
- **Framework**: Rocket (Async)
- **Database ORM**: sqlx (with `rocket_db_pools`)
- **Password Hashing**: bcrypt
- **Code Execution**: Docker container with Rust toolchain

### Frontend
- **HTML/CSS/JS** based client
- **Syntax Highlighting**: Integrated via CodeMirror

---

## Backend Modules

### `main.rs`
Entry point of the Rocket application. It mounts routes and attaches the database pool:
```rust
rocket::build()
    .attach(Db::init())
    .mount("/auth", routes![auth::register, auth::login])
    .mount("/problems", routes![problems::get_problems, problems::get_problem_details])
    .mount("/judge", routes![judge::submit_code])
```

### `db.rs`
Defines the Rocket-compatible database pool wrapper:
```rust
#[derive(Database)]
#[database("mysql_database")]
pub struct Db(sqlx::MySqlPool);
```

### `models`
- `User`: Defines the user schema.
- `Problem`: Defines problem metadata.
- `TestCase`: Defines test case schema linked to a problem.

### `routes`
#### `auth.rs`
- **POST /auth/register**: Register a new user with hashed password.
- **POST /auth/login**: Validate user credentials.

#### `problems.rs`
- **GET /problems**: Fetch list of available problems.
- **GET /problems/<id>`**: Fetch problem details by ID.

#### `judge.rs`
- **POST /judge**: Accepts problem ID and code. For each test case:
  1. Writes user code to `/tmp/user_code.rs`
  2. Runs Docker container to compile and execute
  3. Compares actual output with expected output

---

## Frontend Details

### Pages & Elements
- Problem list panel (auto-loaded on load)
- Problem detail section (displays when a problem is selected)
- Code editor (CodeMirror replaces `<textarea>`)
- Submit button triggers fetch to backend
- Result container displays server response

### CodeMirror Integration
- Added via CDN in `index.html`
- JS initialization replaces `textarea` with enhanced editor

---

## Docker Requirement
Ensure Docker is installed and the host can run the following:
```bash
docker run --rm -v /tmp/user_code.rs:/user_code.rs rust:latest sh -c "rustc /user_code.rs && ./user_code"
```
Note: This must be tested with appropriate permissions.

---

## Environment Configuration
Located in `Rocket.toml`:
```toml
[default]
address = "139.59.103.151"
port = 8000

[default.databases.mysql_database]
url = "mysql://joe:ws985211@127.0.0.1/learn_rocket"
```

---

## Security Notes
- All user passwords are hashed with bcrypt before storing.
- Docker runs are isolated per submission to prevent system compromise.
- Additional sandboxing and resource limits are recommended for production.

---

## Future Improvements
- Add user session and JWT support
- Save submission history and evaluation logs
- Improve frontend UI with modern frameworks
- Support multiple languages via per-image configuration
- Use async file I/O and better Docker orchestration (e.g. container pool)

---

## Author
Developed by a Computer Science major from Guangdong Province. Deployed using DigitalOcean server with GitHub Student Pack.

Feel free to contribute or raise issues!


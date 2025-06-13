use mysql::Pool;
use urlencoding::encode;
#[derive(Debug)]
pub struct DBConnection {
    pub user: String,
    pub password: String,
    pub ip: String,
    pub port: i16,
    pub db_name: String,
    pub pool: Option<Pool>,
}

impl Default for DBConnection {
    fn default() -> Self {
        Self {
            user: String::from("root"),
            ip: String::from("localhost"),
            port: 3306,
            password: String::from(""),
            db_name: String::from(""),
            pool: None,
        }
    }
}

impl DBConnection {
    pub fn ge_pool(&mut self) {
        let encoded_password = encode(&self.password).into_owned();
        let dsn = format!(
            "mysql://{}:{}@{}:{}/{}",
            self.user, encoded_password, self.ip, self.port, self.db_name
        );
        self.pool = Some(Pool::new(dsn.as_str()).unwrap());
    }
}

//! Конфиг для подключения к базе и настройками для сервиса
//!
//! # Examples
//!
//! ```
//! extern crate rbac;
//! extern crate toml;
//! use rbac::mods::config::Config;
//! fn main() {
//!     let cstr = r#"
//!         [server]
//!         host = "0.0.0.0"
//!         port = "8000"
//!         workers = 1
//!         [db]
//!         host = "0.0.0.0"
//!         port = "3306"
//!         user = "user"
//!         pass = "pass"
//!         database = "name"
//!         query_timestamp = "SELECT timestamp from auth_timestamp where `index` = 0"
//!         query_items = "SELECT name, data from auth_item"
//!         query_assignments = "SELECT user_id, name, data from auth_assignment"
//!         query_relations = "SELECT parent, child from auth_item_child  ORDER BY parent DESC"
//!         [options]
//!         timer = 160
//!     "#;
//!     let conf: Config = toml::from_str(&cstr).unwrap();
//!     assert_eq!(conf.get_bind(), "0.0.0.0:8000");
//!     assert_eq!(conf.get_dsn(), "mysql://user:pass@0.0.0.0:3306");
//!     assert_eq!(conf.get_query_timestamp(), "auth_timestamp");
//!     assert_eq!(conf.get_query_items(), "auth_item");
//!     assert_eq!(conf.get_query_assignments(), "auth_assignment");
//!     assert_eq!(conf.get_query_relations(), "auth_item_child");
//! }
//! ```

use std::env;
use std::fs::File;
use std::io::Read;
use toml;

#[derive(Deserialize, Debug)]
struct ServerConfig {
    host: String,
    port: String,
    workers: u8
}

#[derive(Deserialize, Debug)]
struct DbConfig {
    host: String,
    port: String,
    user: String,
    pass: String,
    query_timestamp: String,
    query_items: String,
    query_assignments: String,
    query_relations: String,
}

#[derive(Deserialize, Debug)]
struct OptionsConfig {
    timer: u64
}

/// Структура с настроками для сервиса
#[derive(Deserialize, Debug)]
pub struct Config {
    server: ServerConfig,
    db: DbConfig,
    options: OptionsConfig
}

impl Config {
    /// получаем время для таймера проверки актуальности настроек рбак
    pub fn get_timer(&self) -> u64 {
        self.options.timer
    }

    /// получаем количество воркеров для сервера
    pub fn get_workers(&self) -> u8 {
        self.server.workers
    }

    /// получаем строку с айпи и портом на котором будет висеть сервис
    pub fn get_bind(&self) -> String {
        let mut out: String = "".to_string();
        out.push_str(&self.server.host);
        out.push_str(":");
        out.push_str(&self.server.port);
        out
    }
    /// получаем строку для подключения к бд
    pub fn get_dsn(&self) -> String {
        let mut out: String = "mysql://".to_string();
        out.push_str(&self.db.user);
        out.push_str(":");
        out.push_str(&self.db.pass);
        out.push_str("@");
        out.push_str(&self.db.host);
        out.push_str(":");
        out.push_str(&self.db.port);
        out.push_str("/?prefer_socket=false");
        out
    }

    pub fn get_query_timestamp(&self) -> &String {
        &self.db.query_timestamp
    }
    pub fn get_query_items(&self) -> &String {
        &self.db.query_items
    }
    pub fn get_query_assignments(&self) -> &String {
        &self.db.query_assignments
    }
    pub fn get_query_relations(&self) -> &String {
        &self.db.query_relations
    }
}
/// получаем структуру конфига из файла, переданного через аргумент коммандной строки
///
/// rbac --config=config.toml
///
pub fn load_config() -> Config {
    let config_file: String = get_config_file_name().unwrap();
    let mut chdl = match File::open(&config_file) {
        Ok(f) => f
        ,
        Err(e) => panic!("Error occurred config file: {} - Err: {}", &config_file, e)
    };

    let mut cstr = String::new();
    match chdl.read_to_string(&mut cstr) {
        Ok(s) => s
        ,
        Err(e) => panic!("Error Reading config file: {}", e)
    };
    toml::from_str(&cstr).unwrap()
}
/// получаем имя файла из аргументов коммандной строки
/// если не нашли вернём ошибку
fn get_config_file_name() -> Result<String, &'static str> {
    let args: Vec<String> = env::args().collect();
    let config_prefix: &str = "--config=";
    for arg in args {
        if &arg[0..config_prefix.len()] == config_prefix {
            return Ok(arg[config_prefix.len()..].to_string());
        }
    }
    return Err("You should set --config= param");
}
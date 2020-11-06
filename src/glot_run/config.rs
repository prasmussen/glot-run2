use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use crate::glot_run::api;
use crate::glot_run::run;

#[derive(Clone, Debug)]
pub struct Config {
    pub server: ServerConfig,
    pub api: api::ApiConfig,
    pub run: run::Config,
}


#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub listen_addr: String,
    pub listen_port: u16,
    pub worker_threads: u16,
    pub data_root: Arc<Mutex<PathBuf>>,
}

impl ServerConfig {
    pub fn listen_addr_with_port(&self) -> String {
        format!("{}:{}", self.listen_addr, self.listen_port)
    }
}


pub fn users_path(path: &Path) -> PathBuf {
    path.join("users.json")
}

pub fn languages_path(path: &Path) -> PathBuf {
    path.join("languages.json")
}

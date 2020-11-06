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
    pub data_root: Arc<Mutex<DataRoot>>,
}

impl ServerConfig {
    pub fn listen_addr_with_port(&self) -> String {
        format!("{}:{}", self.listen_addr, self.listen_port)
    }
}



#[derive(Clone, Debug)]
pub struct DataRoot(PathBuf);


impl DataRoot {
    pub fn new(path: PathBuf) -> DataRoot {
        DataRoot(path)
    }

    pub fn root_path(&self) -> PathBuf {
        self.0.clone()
    }

    pub fn users_path(&self) -> PathBuf {
        self.0.join("users.json")
    }

    pub fn languages_path(&self) -> PathBuf {
        self.0.join("languages.json")
    }
}

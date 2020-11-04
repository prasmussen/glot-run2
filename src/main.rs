mod glot_run;

use std::process;
use std::fs;
use std::io;
use std::fmt;
use std::time::Duration;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use glot_run::config;
use glot_run::environment;
use glot_run::api;
use glot_run::datastore;
use glot_run::file;
use glot_run::user;
use glot_run::language;


fn main() {
    env_logger::init();

    match start() {
        Ok(()) => {}

        Err(err) => {
            log::error!("{}", err);
            process::exit(1)
        }
    }
}

enum Error {
    BuildConfig(environment::Error),
    PrepareDataDirectory(io::Error),
    DatastoreInit(file::WriteJsonError),
    StartServer(api::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::BuildConfig(err) => {
                write!(f, "Failed to build config: {}", err)
            }

            Error::PrepareDataDirectory(err) => {
                write!(f, "Failed to prepare data directory: {}", err)
            }

            Error::DatastoreInit(err) => {
                write!(f, "Failed to init datastore: {}", err)
            }

            Error::StartServer(err) => {
                write!(f, "Failed to start api server: {}", err)
            }
        }
    }
}


fn start() -> Result<(), Error> {
    let env = environment::get_environment();
    let config = build_config(&env)
        .map_err(Error::BuildConfig)?;

    prepare_datastore(&config)?;

    log::info!("Listening on {} with {} worker threads", config.server.listen_addr_with_port(), config.server.worker_threads);

    api::start(api::Config{
        listen_addr: config.server.listen_addr_with_port(),
        worker_threads: config.server.worker_threads,
        handler_config: config,
        handler: handle_request,
    }).map_err(Error::StartServer)
}

fn handle_request(config: &config::Config, mut request: tiny_http::Request) {

    let result = match router(&config, &mut request) {
        Ok(data) => {
            api::success_response(request, &data)
        }

        Err(err) => {
            api::error_response(request, err)
        }
    };

    match result {
        Ok(()) => {},

        Err(err) => {
            log::error!("Failure while sending response: {}", err)
        }
    }
}

fn router(config: &config::Config, request: &mut tiny_http::Request) -> Result<api::SuccessResponse, api::ErrorResponse> {
    let path = request
        .url()
        .trim_start_matches('/')
        .trim_end_matches('/')
        .split('/')
        .filter(|str| !str.is_empty())
        .collect::<Vec<&str>>();

    match (path.as_slice(), request.method()) {
        ([] , tiny_http::Method::Get) => {
            api::root::handle(config, request)
        }

        (["admin", "users"], tiny_http::Method::Get) => {
            api::admin::users::list::handle(config, request)
        }

        (["admin", "users"], tiny_http::Method::Post) => {
            api::admin::users::create::handle(config, request)
        }

        (["admin", "users", user_id], tiny_http::Method::Get) => {
            api::admin::users::get::handle(config, request, &user_id.to_string())
        }

        (["admin", "users", user_id], tiny_http::Method::Put) => {
            api::admin::users::update::handle(config, request, &user_id.to_string())
        }

        (["admin", "users", user_id], tiny_http::Method::Delete) => {
            api::admin::users::delete::handle(config, request, &user_id.to_string())
        }

        (["admin", "languages"], tiny_http::Method::Get) => {
            api::admin::languages::list::handle(config, request)
        }

        (["admin", "languages"], tiny_http::Method::Put) => {
            api::admin::languages::create::handle(config, request)
        }

        (["admin", "languages", language_id], tiny_http::Method::Get) => {
            api::admin::languages::get::handle(config, request, &language_id.to_string())
        }

        (["admin", "languages", language_id], tiny_http::Method::Delete) => {
            api::admin::languages::delete::handle(config, request, &language_id.to_string())
        }

        _ => {
            api::not_found::handle(config, request)
        }
    }
}


fn build_config(env: &environment::Environment) -> Result<config::Config, environment::Error> {
    let server = build_server_config(env)?;
    let api = build_api_config(env)?;

    Ok(config::Config{
        server,
        api,
    })
}

fn build_server_config(env: &environment::Environment) -> Result<config::ServerConfig, environment::Error> {
    let listen_addr = environment::lookup(env, "SERVER_LISTEN_ADDR")?;
    let listen_port = environment::lookup(env, "SERVER_LISTEN_PORT")?;
    let worker_threads = environment::lookup(env, "SERVER_WORKER_THREADS")?;
    let data_root: PathBuf = environment::lookup(env, "SERVER_DATA_ROOT")?;

    Ok(config::ServerConfig{
        listen_addr,
        listen_port,
        worker_threads,
        data_root: Arc::new(Mutex::new(data_root)),
    })
}

fn build_api_config(env: &environment::Environment) -> Result<api::ApiConfig, environment::Error> {
    let access_token = environment::lookup(env, "API_ACCESS_TOKEN")?;

    Ok(api::ApiConfig{
        access_token,
    })
}


fn prepare_datastore(config: &config::Config) -> Result<(), Error> {
    let data_root = config.server.data_root.lock().unwrap();

    fs::create_dir_all(&*data_root)
        .map_err(Error::PrepareDataDirectory)?;

    let users_path = config::users_path(&data_root);
    let languages_path = config::languages_path(&data_root);

    datastore::init::<user::User>(&users_path)
        .map_err(Error::DatastoreInit)?;

    datastore::init::<language::Language>(&languages_path)
        .map_err(Error::DatastoreInit)?;

    Ok(())
}


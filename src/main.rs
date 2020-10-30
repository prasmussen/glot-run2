mod glot_run;

use std::process;
use std::time::Duration;

use glot_run::config;
use glot_run::environment;
use glot_run::api;


fn main() {
    env_logger::init();

    match start() {
        Ok(()) => {}

        Err(Error::BuildConfig(err)) => {
            log::error!("Failed to build config: {}", err);
            process::exit(1)
        }

        Err(Error::StartServer(err)) => {
            log::error!("Failed to start api server: {}", err);
            process::exit(1)
        }
    }
}

enum Error {
    BuildConfig(environment::Error),
    StartServer(api::Error),
}

fn start() -> Result<(), Error> {
    let env = environment::get_environment();
    let config = build_config(&env)
        .map_err(Error::BuildConfig)?;

    log::info!("Listening on {} with {} worker threads", config.server.listen_addr_with_port(), config.server.worker_threads);

    api::start(api::Config{
        listen_addr: config.server.listen_addr_with_port(),
        worker_threads: config.server.worker_threads,
        handler_config: config,
        handler: handle_request,
    }).map_err(Error::StartServer)
}


fn handle_request(config: &config::Config, mut request: tiny_http::Request) {

    let handler = router(&request);

    let result = match handler(&config, &mut request) {
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

fn router(request: &tiny_http::Request) -> fn(&config::Config, &mut tiny_http::Request) -> Result<Vec<u8>, api::ErrorResponse> {
    match (request.method(), request.url()) {
        (tiny_http::Method::Get, "/") => {
            api::root::handle
        }

        _ => {
            api::not_found::handle
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

    Ok(config::ServerConfig{
        listen_addr,
        listen_port,
        worker_threads,
    })
}

fn build_api_config(env: &environment::Environment) -> Result<api::ApiConfig, environment::Error> {
    let access_token = environment::lookup(env, "API_ACCESS_TOKEN")?;

    Ok(api::ApiConfig{
        access_token,
    })
}

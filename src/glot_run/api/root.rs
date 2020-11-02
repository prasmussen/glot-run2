use crate::glot_run::config;
use crate::glot_run::api;


const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");


#[derive(Debug, serde::Serialize)]
struct Response {
    name: String,
    version: String,
    description: String,
}

pub fn handle(_: &config::Config, _: &mut tiny_http::Request) -> Result<Vec<u8>, api::ErrorResponse> {

    api::prepare_json_response(&Response{
        name: "glot-run".to_string(),
        version: VERSION.unwrap_or("unknown").to_string(),
        description: "Api for managing users and languages for docker-run".to_string(),
    })
}

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

    let response = Response{
        name: "glot-run".to_string(),
        version: VERSION.unwrap_or("unknown").to_string(),
        description: "Api for managing users and languages for docker-run".to_string(),
    };

    serde_json::to_vec_pretty(&response).map_err(|err| {
        api::ErrorResponse{
            status_code: 500,
            body: serde_json::to_vec_pretty(&api::ErrorBody{
                error: "response.serialize".to_string(),
                message: format!("Failed to serialize response: {}", err),
            }).unwrap_or_else(|_| err.to_string().as_bytes().to_vec())
        }
    })
}

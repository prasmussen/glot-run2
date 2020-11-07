use crate::glot_run::config;
use crate::glot_run::api;


const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");


#[derive(Debug, serde::Serialize)]
struct Response {
    name: String,
    description: String,
    version: String,
    urls: ApiUrls,
}

#[derive(Debug, serde::Serialize)]
struct ApiUrls {
    admin: String,
    languages: String,
    images: String,
}

pub fn handle(config: &config::Config, _: &mut tiny_http::Request) -> Result<api::SuccessResponse, api::ErrorResponse> {

    api::prepare_json_response(&Response{
        name: "glot-run".to_string(),
        version: VERSION.unwrap_or("unknown").to_string(),
        description: "Api for managing users and languages and run code via docker-run".to_string(),
        urls: ApiUrls{
            admin: format!("{}/admin", config.server.base_url),
            languages: format!("{}/languages", config.server.base_url),
            images: format!("{}/images", config.server.base_url),
        }
    })
}

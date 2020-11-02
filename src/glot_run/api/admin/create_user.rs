use crate::glot_run::config;
use crate::glot_run::api;
use crate::glot_run::user;
use crate::glot_run::file;
use crate::glot_run::datastore;


const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");


#[derive(Debug, serde::Deserialize)]
struct CreateUserRequest {
    token: String,
}


pub fn handle(config: &config::Config, request: &mut tiny_http::Request) -> Result<Vec<u8>, api::ErrorResponse> {

    let createReq: CreateUserRequest = serde_json::from_reader(request.as_reader())
        .map_err(|err| api::ErrorResponse{
            status_code: 400,
            body: serde_json::to_vec(&api::ErrorBody{
                error: "request.parse".to_string(),
                message: format!("Failed to parse json from request: {}", err),
            }).unwrap_or_else(|_| err.to_string().as_bytes().to_vec())
        })?;

    let user = user::new(&createReq.token);

    let data_root = config.server.data_root.lock().unwrap();
    let users_path = config::users_path(&data_root);
    let res = datastore::add_entry(&users_path, &user.id.to_string(), &user);

    match res {
        Ok(()) => {
            serde_json::to_vec_pretty(&user).map_err(|err| {
                api::ErrorResponse{
                    status_code: 500,
                    body: serde_json::to_vec_pretty(&api::ErrorBody{
                        error: "response.serialize".to_string(),
                        message: format!("Failed to serialize response: {}", err),
                    }).unwrap_or_else(|_| err.to_string().as_bytes().to_vec())
                }
            })
        }

        Err(err) => {
            Err(api::ErrorResponse{
                status_code: 500,
                body: serde_json::to_vec(&api::ErrorBody{
                    error: "datastore".to_string(),
                    message: err.to_string(),
                }).unwrap_or_else(|_| err.to_string().as_bytes().to_vec())
            })
        }
    }
}

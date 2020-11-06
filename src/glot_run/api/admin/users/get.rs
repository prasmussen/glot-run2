use crate::glot_run::config;
use crate::glot_run::api;
use crate::glot_run::user;
use crate::glot_run::datastore;



pub fn handle(config: &config::Config, request: &mut tiny_http::Request, user_id: &str) -> Result<api::SuccessResponse, api::ErrorResponse> {
    api::check_access_token(&config.api.access_token, request)?;

    let data_root = config.server.data_root.lock().unwrap();
    let user = datastore::get_entry::<user::User>(&data_root.users_path(), user_id).
        map_err(handle_datastore_error)?;

    api::prepare_json_response(&user)
}


fn handle_datastore_error(err: datastore::GetError) -> api::ErrorResponse {
    match err {
        datastore::GetError::Read(_) => {
            api::ErrorResponse{
                status_code: 500,
                body: api::ErrorBody{
                    error: "datastore".to_string(),
                    message: err.to_string(),
                }
            }
        }

        datastore::GetError::NotFound() => {
            api::ErrorResponse{
                status_code: 404,
                body: api::ErrorBody{
                    error: "not_found".to_string(),
                    message: err.to_string(),
                }
            }
        }
    }
}

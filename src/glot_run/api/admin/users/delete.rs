use crate::glot_run::config;
use crate::glot_run::api;
use crate::glot_run::user;
use crate::glot_run::datastore;



pub fn handle(config: &config::Config, request: &mut tiny_http::Request, user_id: &str) -> Result<api::SuccessResponse, api::ErrorResponse> {
    api::check_access_token(&config.api.admin_access_token, request)?;

    let data_root = config.server.data_root.lock().unwrap();
    datastore::remove_entry::<user::User>(&data_root.users_path(), user_id)
        .map_err(handle_datastore_error)?;

    Ok(api::prepare_empty_response())
}

fn handle_datastore_error(err: datastore::AddError) -> api::ErrorResponse {

    api::ErrorResponse{
        status_code: 500,
        body: api::ErrorBody{
            error: "datastore".to_string(),
            message: err.to_string(),
        }
    }
}

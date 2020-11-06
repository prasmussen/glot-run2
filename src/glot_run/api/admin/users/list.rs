use crate::glot_run::config;
use crate::glot_run::api;
use crate::glot_run::user;
use crate::glot_run::datastore;
use crate::glot_run::file;



pub fn handle(config: &config::Config, _: &mut tiny_http::Request) -> Result<api::SuccessResponse, api::ErrorResponse> {

    let data_root = config.server.data_root.lock().unwrap();
    let users = datastore::list_values::<user::User>(&data_root.users_path())
        .map_err(handle_datastore_error)?;

    api::prepare_json_response(&users)
}

fn handle_datastore_error(err: file::ReadJsonError) -> api::ErrorResponse {
    api::ErrorResponse{
        status_code: 500,
        body: api::ErrorBody{
            error: "datastore".to_string(),
            message: err.to_string(),
        }
    }
}

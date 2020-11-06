use crate::glot_run::config;
use crate::glot_run::api;
use crate::glot_run::language;
use crate::glot_run::datastore;



pub fn handle(config: &config::Config, request: &mut tiny_http::Request, language_id: &str) -> Result<api::SuccessResponse, api::ErrorResponse> {
    api::check_access_token(&config.api.access_token, request)?;

    let data_root = config.server.data_root.lock().unwrap();
    let language = datastore::get_entry::<language::Language>(&data_root.languages_path(), language_id)
        .map_err(handle_datastore_error)?;

    api::prepare_json_response(&language)
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

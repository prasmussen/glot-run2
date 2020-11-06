use crate::glot_run::config;
use crate::glot_run::api;
use crate::glot_run::language;
use crate::glot_run::datastore;



pub fn handle(config: &config::Config, request: &mut tiny_http::Request) -> Result<api::SuccessResponse, api::ErrorResponse> {
    api::check_access_token(&config.api.access_token, request)?;

    let language_data: language::LanguageData = api::read_json_body(request)?;
    let language = language::new(&language_data);

    let data_root = config.server.data_root.lock().unwrap();
    let language = datastore::add_entry(&data_root.languages_path(), &language.id, &language)
        .map_err(handle_datastore_error)?;

    api::prepare_json_response(&language)
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

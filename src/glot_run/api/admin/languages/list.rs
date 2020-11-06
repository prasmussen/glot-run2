use crate::glot_run::config;
use crate::glot_run::api;
use crate::glot_run::language;
use crate::glot_run::datastore;
use crate::glot_run::file;



pub fn handle(config: &config::Config, _: &mut tiny_http::Request) -> Result<api::SuccessResponse, api::ErrorResponse> {

    let data_root = config.server.data_root.lock().unwrap();
    let mut languages = datastore::list_values::<language::Language>(&data_root.languages_path())
        .map_err(handle_datastore_error)?;

    languages.sort_by_key(|language| language.name.clone());

    api::prepare_json_response(&languages)
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

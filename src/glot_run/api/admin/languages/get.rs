use crate::glot_run::config;
use crate::glot_run::api;
use crate::glot_run::language;
use crate::glot_run::datastore;



pub fn handle(config: &config::Config, _: &mut tiny_http::Request, language_id: &str) -> Result<api::SuccessResponse, api::ErrorResponse> {

    let data_root = config.server.data_root.lock().unwrap();
    let res = datastore::get_entry::<language::Language>(&data_root.languages_path(), language_id);

    match res {
        Ok(language) => {
            api::prepare_json_response(&language)
        }

        Err(err) => {
            Err(api::ErrorResponse{
                status_code: status_code(&err),
                body: api::ErrorBody{
                    error: "datastore".to_string(),
                    message: err.to_string(),
                }
            })
        }
    }
}


fn status_code(err: &datastore::GetError) -> u16 {
    match err {
        datastore::GetError::Read(_) =>
            500,

        datastore::GetError::NotFound() =>
            404,
    }
}

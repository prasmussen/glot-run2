use crate::glot_run::config;
use crate::glot_run::api;
use crate::glot_run::language;
use crate::glot_run::datastore;




pub fn handle(config: &config::Config, _: &mut tiny_http::Request, language_id: &str) -> Result<api::SuccessResponse, api::ErrorResponse> {

    let data_root = config.server.data_root.lock().unwrap();
    let languages_path = config::languages_path(&data_root);
    let res = datastore::remove_entry::<language::Language>(&languages_path, language_id);

    match res {
        Ok(()) => {
            Ok(api::prepare_empty_response())
        }

        Err(err) => {
            Err(api::ErrorResponse{
                status_code: 500,
                body: api::ErrorBody{
                    error: "datastore".to_string(),
                    message: err.to_string(),
                }
            })
        }
    }
}

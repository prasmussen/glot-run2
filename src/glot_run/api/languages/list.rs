use crate::glot_run::config;
use crate::glot_run::api;
use crate::glot_run::language;
use crate::glot_run::datastore;


#[derive(Debug, serde::Serialize)]
pub struct Language {
    name: String,
}


pub fn handle(config: &config::Config, request: &mut tiny_http::Request) -> Result<api::SuccessResponse, api::ErrorResponse> {

    let data_root = config.server.data_root.lock().unwrap();
    let languages_path = config::languages_path(&data_root);
    let res = datastore::list_values::<language::Language>(&languages_path);

    match res {
        Ok(languages) => {
            api::prepare_json_response(&response_body(languages))
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


fn response_body(languages: Vec<language::Language>) -> Vec<Language> {
    languages.iter().map(|language| {
        Language{
            name: language.name.clone(),
        }
    })
    .collect()
}

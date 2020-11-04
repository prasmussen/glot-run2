use crate::glot_run::config;
use crate::glot_run::api;
use crate::glot_run::language;
use crate::glot_run::datastore;


#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, serde::Serialize)]
pub struct Language {
    version: String,
}


pub fn handle(config: &config::Config, request: &mut tiny_http::Request, language_name: &str) -> Result<api::SuccessResponse, api::ErrorResponse> {

    let data_root = config.server.data_root.lock().unwrap();
    let languages_path = config::languages_path(&data_root);
    let res = datastore::list_values::<language::Language>(&languages_path).map(|languages| {
        languages
            .iter()
            .filter(|language| language.name == language_name)
            .map(to_language)
            .collect::<Vec<Language>>()
    });

    match res {
        Ok(mut languages) => {
            languages.sort();

            if !languages.is_empty() {
                api::prepare_json_response(&languages)
            } else {
                Err(api::ErrorResponse{
                    status_code: 404,
                    body: api::ErrorBody{
                        error: "not_found".to_string(),
                        message: "Language not found".to_string(),
                    }
                })
            }
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


fn to_language(language: &language::Language) -> Language {
    Language{
        version: language.version.clone(),
    }
}

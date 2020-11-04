use crate::glot_run::config;
use crate::glot_run::api;
use crate::glot_run::language;
use crate::glot_run::datastore;


#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, serde::Serialize)]
pub struct Language {
    name: String,
}


pub fn handle(config: &config::Config, request: &mut tiny_http::Request) -> Result<api::SuccessResponse, api::ErrorResponse> {

    let data_root = config.server.data_root.lock().unwrap();
    let languages_path = config::languages_path(&data_root);
    let res = datastore::list_values::<language::Language>(&languages_path).map(|languages| {
        languages
            .iter()
            .map(to_language)
            .collect::<Vec<Language>>()
    });

    match res {
        Ok(mut languages) => {
            languages.sort();
            languages.dedup();

            api::prepare_json_response(&languages)
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
        name: language.name.clone(),
    }
}

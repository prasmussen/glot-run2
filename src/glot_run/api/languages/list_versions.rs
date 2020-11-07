use crate::glot_run::config;
use crate::glot_run::api;
use crate::glot_run::language;
use crate::glot_run::datastore;
use crate::glot_run::file;
use crate::glot_run::util;


#[derive(Debug, Eq, PartialEq, serde::Serialize)]
pub struct Language {
    version: String,
    url: String,
}


pub fn handle(config: &config::Config, _: &mut tiny_http::Request, language_name: &str) -> Result<api::SuccessResponse, api::ErrorResponse> {

    let data_root = config.server.data_root.lock().unwrap();
    let mut languages = datastore::list_values::<language::Language>(&data_root.languages_path()).map(|languages| {
        languages
            .iter()
            .filter(|language| language.name == language_name)
            .map(|language| to_language(config, language))
            .collect::<Vec<Language>>()
    }).map_err(handle_datastore_error)?;

    languages.sort_by_key(|language| language.version.clone());

    util::err_if_false(!languages.is_empty(), api::ErrorResponse{
        status_code: 404,
        body: api::ErrorBody{
            error: "not_found".to_string(),
            message: "Language not found".to_string(),
        }
    })?;

    api::prepare_json_response(&languages)
}



fn to_language(config: &config::Config, language: &language::Language) -> Language {
    Language{
        version: language.version.clone(),
        url: format!("{}/languages/{}/{}", config.server.base_url, language.name, language.version),
    }
}


fn handle_datastore_error(err: file::ReadJsonError) -> api::ErrorResponse{
    api::ErrorResponse{
        status_code: 500,
        body: api::ErrorBody{
            error: "datastore".to_string(),
            message: err.to_string(),
        }
    }
}

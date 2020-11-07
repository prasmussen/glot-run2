use crate::glot_run::config;
use crate::glot_run::api;
use crate::glot_run::language;
use crate::glot_run::datastore;
use crate::glot_run::file;


#[derive(Debug, Eq, PartialEq, serde::Serialize)]
pub struct Image {
    name: String,
    url: String,
}


pub fn handle(config: &config::Config, _: &mut tiny_http::Request) -> Result<api::SuccessResponse, api::ErrorResponse> {

    let data_root = config.server.data_root.lock().unwrap();
    let mut images = datastore::list_values::<language::Language>(&data_root.languages_path()).map(|languages| {
        languages
            .iter()
            .map(to_image)
            .collect::<Vec<Image>>()
    }).map_err(handle_datastore_error)?;

    images.sort_by_key(|language| language.name.clone());
    images.dedup();

    api::prepare_json_response(&images)
}


fn to_image(language: &language::Language) -> Image {
    Image{
        name: language.name.clone(),
        url: format!("https://hub.docker.com/r/glot/{}/", language.name),
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

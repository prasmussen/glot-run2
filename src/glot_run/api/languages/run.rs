use crate::glot_run::config;
use crate::glot_run::api;
use crate::glot_run::language;
use crate::glot_run::datastore;
use crate::glot_run::run;


#[derive(Debug)]
pub struct Options {
    pub language: String,
    pub version: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct RequestBody {
    pub files: Vec<run::File>,
    pub stdin: Option<String>,
    pub command: Option<String>,
}


pub fn handle(config: &config::Config, request: &mut tiny_http::Request, options: Options) -> Result<api::SuccessResponse, api::ErrorResponse> {

    let req_body: RequestBody = api::read_json_body(request)?;
    let data_root = config.server.data_root.lock().unwrap();
    let language = datastore::find_value::<_, language::Language>(&data_root.languages_path(), |language| {
        language.name == options.language && language.version == options.version
    }).map_err(handle_datastore_error)?;

    let run_result = run::run(&config.run, run::RunRequest{
        image: language.image.clone(),
        payload: run::RunRequestPayload{
            language: language.name.clone(),
            files: req_body.files.clone(),
            stdin: req_body.stdin.clone(),
            command: req_body.command.clone(),
        }
    }).map_err(handle_run_error)?;

    api::prepare_json_response(&run_result)
}


fn handle_datastore_error(err: datastore::GetError) -> api::ErrorResponse {
    match err {
        datastore::GetError::NotFound() => {
            api::ErrorResponse{
                status_code: 404,
                body: api::ErrorBody{
                    error: "not_found".to_string(),
                    message: "Language or version not found".to_string(),
                }
            }
        }

        _ => {
            api::ErrorResponse{
                status_code: 500,
                body: api::ErrorBody{
                    error: "datastore".to_string(),
                    message: err.to_string(),
                }
            }
        }
    }
}


fn handle_run_error(err: run::Error) -> api::ErrorResponse{
    match err {
        run::Error::SerializeRequest(serde_err) => {
            api::ErrorResponse{
                status_code: 400,
                body: api::ErrorBody{
                    error: "run.request.body".to_string(),
                    message: format!("Failed to serialize run request: {}", serde_err)
                }
            }
        }

        // TODO: improve
        _ => {
            api::ErrorResponse{
                status_code: 500,
                body: api::ErrorBody{
                    error: "run".to_string(),
                    message: err.to_string(),
                }
            }
        }
    }
}

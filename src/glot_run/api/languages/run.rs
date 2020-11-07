use crate::glot_run::config;
use crate::glot_run::api;
use crate::glot_run::language;
use crate::glot_run::user;
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
    let data_root = config.server.data_root.lock().unwrap();
    check_user(request, &data_root)?;

    let language = datastore::find_value::<_, language::Language>(&data_root.languages_path(), |language| {
        language.name == options.language && language.version == options.version
    }).map_err(handle_datastore_error)?;

    // Unlock mutex
    drop(data_root);

    let req_body: RequestBody = api::read_json_body(request)?;

    let run_result = run::run(&config.run, run::RunRequest{
        image: language.image,
        payload: run::RunRequestPayload{
            language: language.name,
            files: req_body.files,
            stdin: req_body.stdin,
            command: req_body.command,
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


// TODO: Send proper status codes
// 400 is returned in all cases temporarily until we can improve error handling in glot-www
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

        run::Error::DeserializeResponse(io_err) => {
            api::ErrorResponse{
                status_code: 400,
                body: api::ErrorBody{
                    error: "run.response.body".to_string(),
                    message: format!("Failed to serialize run request: {}", io_err)
                }
            }
        }

        run::Error::ResponseNotOk(error_response) => {
            api::ErrorResponse{
                status_code: 400,
                body: error_response.body,
            }
        }
    }
}


fn check_user(request: &tiny_http::Request, data_root: &config::DataRoot) -> Result<(), api::ErrorResponse> {
    let auth_token = api::get_auth_token(request).ok_or_else(api::authorization_error)?;

    let _ = datastore::find_value::<_, user::User>(&data_root.users_path(), |user| {
        user.token == auth_token
    }).map_err(handle_user_not_found)?;

    Ok(())
}

fn handle_user_not_found(err: datastore::GetError) -> api::ErrorResponse {
    match err {
        datastore::GetError::NotFound() => {
            api::authorization_error()
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


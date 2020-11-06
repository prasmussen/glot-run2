use crate::glot_run::config;
use crate::glot_run::api;
use crate::glot_run::user;
use crate::glot_run::datastore;


#[derive(Debug, serde::Deserialize)]
struct RequestBody {
    token: String,
}

pub fn handle(config: &config::Config, request: &mut tiny_http::Request, user_id: &str) -> Result<api::SuccessResponse, api::ErrorResponse> {

    let reqBody: RequestBody = api::read_json_body(request)?;
    let data_root = config.server.data_root.lock().unwrap();
    let res = datastore::update_entry::<_, user::User>(&data_root.users_path(), user_id, |user| {
        user::update_token(user, &reqBody.token)
    });

    match res {
        Ok(user) => {
            api::prepare_json_response(&user)
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


fn status_code(err: &datastore::UpdateError) -> u16 {
    match err {
        datastore::UpdateError::NotFound() =>
            404,

        _ =>
            500,
    }
}

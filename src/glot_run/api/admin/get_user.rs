use crate::glot_run::config;
use crate::glot_run::api;
use crate::glot_run::user;
use crate::glot_run::datastore;



pub fn handle(config: &config::Config, _: &mut tiny_http::Request, user_id: &str) -> Result<api::SuccessResponse, api::ErrorResponse> {

    let data_root = config.server.data_root.lock().unwrap();
    let users_path = config::users_path(&data_root);
    let res = datastore::get_entry::<user::User>(&users_path, user_id);

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


fn status_code(err: &datastore::GetError) -> u16 {
    match err {
        datastore::GetError::Read(_) =>
            500,

        datastore::GetError::NotFound() =>
            404,
    }
}

use crate::glot_run::config;
use crate::glot_run::api;
use crate::glot_run::user;
use crate::glot_run::datastore;




pub fn handle(config: &config::Config, _: &mut tiny_http::Request, user_id: &str) -> Result<api::SuccessResponse, api::ErrorResponse> {

    let data_root = config.server.data_root.lock().unwrap();
    let res = datastore::remove_entry::<user::User>(&data_root.users_path(), user_id);

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

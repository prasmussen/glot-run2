use crate::glot_run::config;
use crate::glot_run::api;
use crate::glot_run::user;
use crate::glot_run::datastore;



pub fn handle(config: &config::Config, request: &mut tiny_http::Request) -> Result<api::SuccessResponse, api::ErrorResponse> {

    let data_root = config.server.data_root.lock().unwrap();
    let res = datastore::list_values::<user::User>(&data_root.users_path());

    match res {
        Ok(users) => {
            api::prepare_json_response(&users)
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

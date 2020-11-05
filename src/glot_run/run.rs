use std::io;
use std::fmt;
use crate::glot_run::util;


#[derive(Debug, serde::Serialize)]
pub struct RunRequest {
    pub image: String,
    pub payload: RunRequestPayload,
}


#[derive(Debug, serde::Serialize)]
pub struct RunRequestPayload {
    pub language: String,
    pub files: Vec<File>,
    pub stdin: Option<String>,
    pub command: Option<String>,
}


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct File {
    pub name: String,
    pub content: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct RunResult {
    pub stdout: String,
    pub stderr: String,
    pub error: String,
}


// TODO: get url and token from config
pub fn run(run_request: RunRequest) -> Result<RunResult, Error> {
    let body = serde_json::to_vec(&run_request)
        .map_err(Error::SerializeRequest)?;

    let response = ureq::post("http://localhost:8088/run")
        .set("X-Access-Token", "magmatic-handyman-confirm-cauldron")
        .set("Content-Type", "application/json")
        .send_bytes(&body);

    util::err_if_false(response.ok(), Error::ResponseNotOk());

    response.into_json_deserialize()
        .map_err(Error::DeserializeResponse)
}


pub enum Error {
    SerializeRequest(serde_json::Error),
    DeserializeResponse(io::Error),
    ResponseNotOk(),
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::SerializeRequest(err) => {
                write!(f, "Failed to serialize request body: {}", err)
            }

            Error::DeserializeResponse(err) => {
                write!(f, "Failed to deserialize response body: {}", err)
            }

            // TODO: improve
            Error::ResponseNotOk() => {
                write!(f, "Response not ok")
            }
        }
    }
}


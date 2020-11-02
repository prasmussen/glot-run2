use std::time;


pub fn rfc3339(system_time: time::SystemTime) -> String {
    let dt: chrono::DateTime<chrono::Utc> = system_time.into();
    dt.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}


pub fn err_if_false<E>(value: bool, err: E) -> Result<(), E> {
    if value {
        Ok(())
    } else {
        Err(err)
    }
}

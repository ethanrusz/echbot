use chrono;
use crate::Error;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Session {
    ret_msg: String,
    session_id: String,
    timestamp: String,
}

pub(crate) async fn get_random_god() -> Result<(), Error> {
    let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S").to_string();
    let method = "createsession";
    let dev_id: String = std::env::var("DEV_ID")
        .expect("Missing DEV_ID");
    let auth_key = std::env::var("AUTH_KEY")
        .expect("Missing AUTH_KEY");

    let hash = md5::compute(format!("{}{}{}{}", dev_id, method, auth_key, timestamp));
    let signature = format!("{:x}", hash);

    let request = format!("https://api.smitegame.com/smiteapi.svc/createsessionJson/{}/{}/{}", dev_id, signature, timestamp);
    println!("{}", request);
    let response = reqwest::get(&request).await?;

    let sessions: Vec<Session> = response.json().await?;
    println!("{:?}", sessions);

    Ok(())
}

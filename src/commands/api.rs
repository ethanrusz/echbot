use md5::Digest;
use rand::seq::SliceRandom;
use reqwest::{Error, Response};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Session {
    ret_msg: String,
    session_id: String,
    timestamp: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct God {
    #[serde(rename = "Name")]
    name: String,
    ret_msg: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Profile {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Personal_Status_Message")]
    pub personal_status_message: Option<String>,
    pub hz_player_name: Option<String>,
    #[serde(rename = "HoursPlayed")]
    pub hours_played: i32,
    #[serde(rename = "Losses")]
    pub losses: i32,
    #[serde(rename = "Wins")]
    pub wins: i32,
    #[serde(rename = "Team_Name")]
    pub clan: Option<String>,
    #[serde(rename = "Level")]
    pub level: i32,
    #[serde(rename = "Platform")]
    pub platform: Option<String>,
    #[serde(rename = "Leaves")]
    pub leaves: i32,
    pub ret_msg: Option<String>,
}

async fn get_utc_timestamp() -> String {
    chrono::Utc::now().format("%Y%m%d%H%M%S").to_string()
}

async fn get_signature(
    dev_id: &String,
    method: &str,
    auth_key: &String,
    timestamp: &String,
) -> String {
    let hash: Digest = md5::compute(format!("{dev_id}{method}{auth_key}{timestamp}"));
    format!("{:x}", hash)
}

async fn create_session() -> Result<Session, Error> {
    let dev_id: String = std::env::var("DEV_ID").expect("Missing DEV_ID");
    let auth_key: String = std::env::var("AUTH_KEY").expect("Missing AUTH_KEY");

    let timestamp: String = get_utc_timestamp().await;
    let signature: String = get_signature(&dev_id, "createsession", &auth_key, &timestamp).await;

    let request: String = format!(
        "https://api.smitegame.com/smiteapi.svc/createsessionJson/{dev_id}/{signature}/{timestamp}"
    );

    let response: Response = reqwest::get(&request).await?;
    let session: Session = response.json().await?;
    Ok(session)
}

async fn get_gods() -> Result<Vec<God>, Error> {
    let dev_id: String = std::env::var("DEV_ID").expect("Missing DEV_ID");
    let auth_key: String = std::env::var("AUTH_KEY").expect("Missing AUTH_KEY");

    let session_id: String = create_session().await?.session_id;

    let timestamp: String = get_utc_timestamp().await;
    let signature: String = get_signature(&dev_id, "getgods", &auth_key, &timestamp).await;

    let request: String = format!(
        "https://api.smitegame.com/smiteapi.svc/getgodsJson/{dev_id}/{signature}/{session_id}/{timestamp}/1"
    );

    let response: Response = reqwest::get(&request).await?;
    let gods: Vec<God> = response.json().await?;
    Ok(gods)
}

pub async fn get_random_god() -> Result<String, Error> {
    let gods: Vec<God> = get_gods().await?;
    let god: &God = gods.choose(&mut rand::thread_rng()).unwrap();
    let name: String = god.name.clone();
    Ok(name)
}

pub async fn get_player(player: String) -> Result<Vec<Profile>, Error> {
    let dev_id: String = std::env::var("DEV_ID").expect("Missing DEV_ID");
    let auth_key: String = std::env::var("AUTH_KEY").expect("Missing AUTH_KEY");

    let session_id: String = create_session().await?.session_id;

    let timestamp: String = get_utc_timestamp().await;
    let signature: String = get_signature(&dev_id, "getplayer", &auth_key, &timestamp).await;

    let request: String = format!(
        "https://api.smitegame.com/smiteapi.svc/getplayerJson/{dev_id}/{signature}/{session_id}/{timestamp}/{player}"
    );

    let response: Response = reqwest::get(&request).await?;
    let profiles: Vec<Profile> = response.json().await?;
    Ok(profiles)
}

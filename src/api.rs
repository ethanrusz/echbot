use chrono;
use crate::Error;

fn get_utc_timestamp() -> String {
    return chrono::Utc::now().format("%Y%m%d%H%M%S").to_string();
}

fn generate_signature(timestamp: String) -> String {
    let dev_id = std::env::var("DEV_ID")
        .expect("Missing DEV_ID");
    let auth_key = std::env::var("AUTH_KEY")
        .expect("Missing AUTH_KEY");
    let signature = md5::compute(format!("{}createsession{}{}", dev_id, auth_key, timestamp));

    return format!("{:?}", signature);
}

pub(crate) async fn create_session() -> Result<(), Error> {
    let timestamp = get_utc_timestamp();
    let signature: String = generate_signature(timestamp);
    println!("{}", signature);
    Ok(())
}

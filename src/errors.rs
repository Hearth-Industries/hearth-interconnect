use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Deserialize,Debug,Serialize,Clone)]
pub struct ErrorReport {
    pub error: String,
    pub request_id: String,
    pub job_id: String,
    pub guild_id: Option<String>
}
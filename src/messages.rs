use schemars::JsonSchema;
use crate::errors::ErrorReport;
use crate::worker_communication::{DirectWorkerCommunication, Job, JobEvent};
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Clone,Debug)]
pub struct LeaveAction {
    pub guild_id: u64
}

#[derive(Clone,Debug)]
pub struct PlayAudioAction {
    pub url: String
}

#[derive(Deserialize,Debug,Serialize,Clone)]
pub struct JobRequest {
    pub request_id: String,
    pub guild_id: String
}

#[derive(Deserialize,Debug,Serialize,Clone,JsonSchema)]
pub struct ExternalQueueJobResponse {
    pub job_id: String,
    pub worker_id: String,
    pub guild_id: String
}

#[derive(Deserialize,Debug,Serialize,Clone)]
pub struct Analytics {
    cpu_usage: u8,
    memory_usage: u8,
    jobs_running: u32,
    disk_usage: u8,
    worker_id: String
}

#[derive(Deserialize,Debug,Serialize,Clone)]
pub struct PingPongResponse {
    pub worker_id: String
}

#[derive(Deserialize,Debug,Serialize,Clone,JsonSchema)]
pub struct Metadata {
    pub duration: Option<u64>, // Duration in seconds
    pub position: Option<u64>,
    pub sample_rate: Option<u32>,
    pub job_id: String,
    pub guild_id: String
}

#[derive(Deserialize,Debug,Serialize,Clone)]
pub struct JobExpired {
    pub guild_id: String,
    pub job_id: String,
}

#[derive(Deserialize,Debug,Serialize,Clone)]
pub struct ShutdownAlert {
    pub worker_id: String,
    pub affected_guild_ids: Vec<String>
}

#[derive(Deserialize,Debug,Serialize,Clone)]
#[serde(tag = "type")]
pub enum Message {
    InternalWorkerAnalytics(Analytics),
    InternalWorkerQueueJob(Job),
    InternalPingPongRequest,
    InternalPongResponse(PingPongResponse),
    // External
    ExternalQueueJob(JobRequest),
    ExternalQueueJobResponse(ExternalQueueJobResponse),
    ExternalMetadataResult(Metadata),
    ExternalJobExpired(JobExpired),
    // Other
    DirectWorkerCommunication(DirectWorkerCommunication),
    ErrorReport(ErrorReport),
    WorkerShutdownAlert(ShutdownAlert)
}
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
    pub guild_id: String,
    pub voice_channel_id: String,
    pub request_id: String,
}

#[derive(Deserialize,Debug,Serialize,Clone)]
pub struct ExternalQueueJobResponse {
    pub job_id: String,
    pub worker_id: String
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
    // Other
    DirectWorkerCommunication(DirectWorkerCommunication),
    ErrorReport(ErrorReport)
}
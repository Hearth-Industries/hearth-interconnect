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

// All other job communication is passed directly to worker instead of running through scheduler
#[derive(Deserialize,Debug,Serialize,Clone)]
#[serde(tag = "type")]
pub enum MessageType {
    // Internal
    InternalWorkerAnalytics,
    InternalWorkerQueueJob,
    InternalPingPongRequest,
    InternalPongResponse,
    // External
    ExternalQueueJob,
    ExternalQueueJobResponse,
    // Other
    DirectWorkerCommunication,
    ErrorReport
}

#[derive(Deserialize,Debug,Serialize,Clone)]
pub struct JobRequest {
    pub guild_id: String,
    pub voice_channel_id: String,
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
    disk_usage: u8
}

#[derive(Deserialize,Debug,Serialize,Clone)]
pub struct Message {
    pub message_type: MessageType, // Handles how message should be parsed
    pub analytics: Option<Analytics>, // Analytics sent by each worker
    pub queue_job_request: Option<JobRequest>,
    pub queue_job_internal: Option<Job>,
    pub request_id: String, // Unique string provided by client to identify this request
    pub worker_id: Option<String>, // ID Unique to each worker
    pub direct_worker_communication: Option<DirectWorkerCommunication>,
    pub external_queue_job_response: Option<ExternalQueueJobResponse>,
    pub job_event: Option<JobEvent>,
    pub error_report: Option<ErrorReport>
}
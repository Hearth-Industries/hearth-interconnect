use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Deserialize,Debug,Serialize,Clone)]
#[serde(tag = "type")]
pub enum DWCActionType {
    LeaveChannel,
    SeekToPosition,
    LoopXTimes,
    ForceStopLoop,
    LoopForever,
    PlayDirectLink,
    PlayFromYoutube,
    PlayFromSoundcloud,
    SetPlaybackVolume,
    PausePlayback,
    ResumePlayback,
    GetTrackCompleteTimestamp,
    QueueTracks,
    GetMetaData
}

#[derive(Deserialize,Debug,Serialize,Clone)]
pub struct Job {
    pub guild_id: String,
    pub voice_channel_id: String,
    pub job_id: String,
    pub worker_id: String,
    pub request_id: String
}

#[derive(Deserialize,Debug,Serialize,Clone)]
pub struct DirectWorkerCommunication {
    pub job_id: String,
    pub guild_id: Option<String>,
    pub play_audio_url: Option<String>,
    pub action_type: DWCActionType,
    pub request_id: Option<String>,
    pub new_volume: Option<f32>,
    pub seek_position: Option<u64>,
    pub loop_times: Option<usize>,
    pub worker_id: String,
}

#[derive(Deserialize,Debug,Serialize,Clone)]
pub enum JobEventType {
    ChannelLeave,
    ChannelMove,
    AudioEnd,
    AudioStart,
}

#[derive(Deserialize,Debug,Serialize,Clone)]
pub struct JobEvent {
    pub event_type: JobEventType
}
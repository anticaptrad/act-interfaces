#![forbid(unsafe_code)]
//! Transport-neutral Anticaptrad contract types.
//!
//! This crate contains data shapes only. Authentication, authorization,
//! persistence, transport, and validation policy belong to consuming crates.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

pub mod creator_media;
pub use creator_media::*;

pub const CREATOR_MEDIA_PROJECT_SCHEMA: &str =
    include_str!("../schemas/v1/creator-media-project.schema.json");
pub const CREATOR_RENDER_RECEIPT_SCHEMA: &str =
    include_str!("../schemas/v1/creator-render-receipt.schema.json");
pub const YOUTUBE_CONTROL_REQUEST_SCHEMA: &str =
    include_str!("../schemas/v1/youtube-control-request.schema.json");
pub const YOUTUBE_CONTROL_RESPONSE_SCHEMA: &str =
    include_str!("../schemas/v1/youtube-control-response.schema.json");
pub const YOUTUBE_LIFECYCLE_EVENT_SCHEMA: &str =
    include_str!("../schemas/v1/youtube-lifecycle-event.schema.json");

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum YoutubeAction {
    Channel,
    Videos,
    Analytics,
    ExportAnalytics,
    Jobs,
    StartUpload,
    ProcessUpload,
    ProcessAllUploads,
    PublishVideo,
    UpdateVideo,
    CreatePlaylist,
    AddToPlaylist,
    IngestGmail,
    SendDigest,
    PartnerStatus,
    PartnerOwners,
    PartnerClaims,
    AdminStatus,
    WorkspaceUsers,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct YoutubeControlRequest {
    pub action: YoutubeAction,
    pub payload: Map<String, Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum YoutubeControlResponse {
    Success(YoutubeControlSuccess),
    Failure(YoutubeControlFailure),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct YoutubeControlSuccess {
    pub ok: bool,
    pub request_id: String,
    pub duration_ms: u64,
    pub data: Value,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct YoutubeControlFailure {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    pub error: YoutubeControlError,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct YoutubeControlError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum YoutubeLifecyclePhase {
    Requested,
    Succeeded,
    Failed,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct YoutubeLifecycleEvent {
    pub request_id: String,
    pub action: YoutubeAction,
    pub phase: YoutubeLifecyclePhase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutating: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Map<String, Value>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_uses_the_schema_field_names() {
        let request = YoutubeControlRequest {
            action: YoutubeAction::PublishVideo,
            payload: Map::new(),
            idempotency_key: Some("request-123".into()),
        };

        assert_eq!(
            serde_json::to_value(request).unwrap(),
            serde_json::json!({
                "action": "publishVideo",
                "payload": {},
                "idempotencyKey": "request-123"
            })
        );
    }

    #[test]
    fn request_rejects_unknown_fields() {
        let result = serde_json::from_value::<YoutubeControlRequest>(serde_json::json!({
            "action": "channel",
            "payload": {},
            "authorization": "must-not-be-in-contracts"
        }));

        assert!(result.is_err());
    }

    #[test]
    fn embedded_schemas_remain_available_to_consumers() {
        assert!(CREATOR_MEDIA_PROJECT_SCHEMA.contains("licensedStock"));
        assert!(CREATOR_RENDER_RECEIPT_SCHEMA.contains("publicEligible"));
        assert!(YOUTUBE_CONTROL_REQUEST_SCHEMA.contains("idempotencyKey"));
        assert!(YOUTUBE_CONTROL_RESPONSE_SCHEMA.contains("durationMs"));
        assert!(YOUTUBE_LIFECYCLE_EVENT_SCHEMA.contains("requested"));
    }
}

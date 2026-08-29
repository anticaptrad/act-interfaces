//! Versioned creator-media project and render-receipt data shapes.
//!
//! These types intentionally model control-plane metadata only. Native media
//! bytes, executable filter strings, credentials, and provider tokens are not
//! part of this contract.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CreatorMediaProject {
    pub schema_version: String,
    pub project_id: String,
    pub creator_handle: String,
    pub title: String,
    pub assets: Vec<CreatorAsset>,
    pub timeline: Vec<TimelineSegment>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub captions: Vec<Caption>,
    pub audio: AudioPlan,
    pub outputs: Vec<OutputPlan>,
    pub publication: PublicationPlan,
    pub review: Review,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CreatorAsset {
    pub asset_id: String,
    pub kind: AssetKind,
    pub relative_path: String,
    pub sha256: String,
    pub media_type: MediaType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
    pub rights: Rights,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AssetKind {
    Camera,
    StockVideo,
    StockImage,
    SoundCue,
    Music,
    Voiceover,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum MediaType {
    #[serde(rename = "video/mp4")]
    VideoMp4,
    #[serde(rename = "video/quicktime")]
    VideoQuicktime,
    #[serde(rename = "video/webm")]
    VideoWebm,
    #[serde(rename = "image/png")]
    ImagePng,
    #[serde(rename = "image/jpeg")]
    ImageJpeg,
    #[serde(rename = "audio/wav")]
    AudioWav,
    #[serde(rename = "audio/mpeg")]
    AudioMpeg,
    #[serde(rename = "audio/flac")]
    AudioFlac,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Rights {
    pub source_type: SourceType,
    pub owner: String,
    pub approved: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SourceType {
    OriginalCamera,
    LicensedStock,
    Generated,
    SoundCue,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TimelineSegment {
    pub segment_id: String,
    pub kind: SegmentKind,
    pub start_ms: u64,
    pub duration_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_in_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextOverlay>,
    pub motion: Motion,
    pub crop: Crop,
    pub audio_gain_db: f64,
    pub transition: Transition,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SegmentKind {
    Source,
    TitleCard,
    Interstitial,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TextOverlay {
    pub heading: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subheading: Option<String>,
    pub position: TextPosition,
    pub accent_color: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TextPosition {
    Center,
    LowerThird,
    UpperThird,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Motion {
    Static,
    PanLeft,
    PanRight,
    TiltUp,
    TiltDown,
    ZoomIn,
    ZoomOut,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Crop {
    Fit,
    Fill,
    FaceAware,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Transition {
    pub kind: TransitionKind,
    pub duration_ms: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TransitionKind {
    Cut,
    Fade,
    Dissolve,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Caption {
    pub start_ms: u64,
    pub end_ms: u64,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AudioPlan {
    pub target_lufs: f64,
    pub true_peak_db: f64,
    pub cues: Vec<CuePlacement>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CuePlacement {
    pub asset_id: String,
    pub start_ms: u64,
    pub gain_db: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct OutputPlan {
    pub output_id: String,
    pub kind: OutputKind,
    pub relative_path: String,
    pub aspect_ratio: AspectRatio,
    pub width: u32,
    pub height: u32,
    pub duration_ms: u64,
    pub safe_area_percent: f64,
    pub privacy_status: PrivacyStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_window: Option<SourceWindow>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum OutputKind {
    Master,
    Clip,
    Variation,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum AspectRatio {
    #[serde(rename = "16:9")]
    Landscape,
    #[serde(rename = "9:16")]
    Portrait,
    #[serde(rename = "1:1")]
    Square,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PrivacyStatus {
    Private,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SourceWindow {
    pub start_ms: u64,
    pub end_ms: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PublicationPlan {
    pub provider: PublicationProvider,
    pub channel_handle: String,
    pub channel_id: String,
    pub privacy_status: PrivacyStatus,
    pub rights_confirmed: bool,
    pub allow_public: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PublicationProvider {
    Youtube,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Review {
    pub state: ReviewState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewed_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ReviewState {
    Draft,
    NeedsReview,
    Approved,
    Rejected,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CreatorRenderReceipt {
    pub schema_version: String,
    pub render_id: String,
    pub project_id: String,
    pub status: RenderStatus,
    pub source: SourceLineage,
    pub outputs: Vec<OutputReceipt>,
    pub toolchain: Toolchain,
    pub review_state: ReceiptReviewState,
    pub publication: PublicationReceipt,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure: Option<RenderFailure>,
    pub created_at: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RenderStatus {
    Succeeded,
    Failed,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SourceLineage {
    pub project_sha256: String,
    pub assets: Vec<SourceAsset>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SourceAsset {
    pub asset_id: String,
    pub sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct OutputReceipt {
    pub output_id: String,
    pub kind: OutputKind,
    pub relative_path: String,
    pub sha256: String,
    pub size_bytes: u64,
    pub duration_ms: u64,
    pub width: u32,
    pub height: u32,
    pub video_codec: VideoCodec,
    pub audio_codec: AudioCodec,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_window: Option<SourceWindow>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoCodec {
    H264,
    Hevc,
    Av1,
    Vp9,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AudioCodec {
    Aac,
    Opus,
    Flac,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Toolchain {
    pub renderer: String,
    pub renderer_version: String,
    pub ffmpeg_version: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ReceiptReviewState {
    NeedsReview,
    Approved,
    Rejected,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PublicationReceipt {
    pub provider: PublicationProvider,
    pub channel_handle: String,
    pub channel_id: String,
    pub privacy_status: PrivacyStatus,
    pub private_upload_eligible: bool,
    pub public_eligible: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RenderFailure {
    pub code: RenderFailureCode,
    pub message: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum RenderFailureCode {
    #[serde(rename = "INVALID_PROJECT")]
    InvalidProject,
    #[serde(rename = "ASSET_NOT_FOUND")]
    AssetNotFound,
    #[serde(rename = "ASSET_DIGEST_MISMATCH")]
    AssetDigestMismatch,
    #[serde(rename = "RIGHTS_NOT_APPROVED")]
    RightsNotApproved,
    #[serde(rename = "UNSUPPORTED_EFFECT")]
    UnsupportedEffect,
    #[serde(rename = "INVALID_OUTPUT")]
    InvalidOutput,
    #[serde(rename = "RENDERER_UNAVAILABLE")]
    RendererUnavailable,
    #[serde(rename = "RENDER_FAILED")]
    RenderFailed,
    #[serde(rename = "PROBE_FAILED")]
    ProbeFailed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reviewed_example_deserializes_to_typed_contract() {
        let project: CreatorMediaProject =
            serde_json::from_str(include_str!("../examples/v1/creator-media-project.json"))
                .expect("reviewed creator project example should deserialize");

        assert_eq!(project.creator_handle, "@anticaptrad");
        assert_eq!(project.publication.channel_id, "UC-Gloecwemo_Mh-VAjnUipg");
        assert_eq!(project.review.state, ReviewState::Approved);
        assert!(project
            .outputs
            .iter()
            .any(|output| { output.kind == OutputKind::Clip && output.duration_ms == 40_000 }));
    }

    #[test]
    fn output_contract_uses_private_visibility_and_clip_window() {
        let output = OutputPlan {
            output_id: "clip-vertical-01".into(),
            kind: OutputKind::Clip,
            relative_path: "outputs/clip-vertical-01.mp4".into(),
            aspect_ratio: AspectRatio::Portrait,
            width: 1080,
            height: 1920,
            duration_ms: 40_000,
            safe_area_percent: 10.0,
            privacy_status: PrivacyStatus::Private,
            source_window: Some(SourceWindow {
                start_ms: 60_000,
                end_ms: 100_000,
            }),
        };

        assert_eq!(
            serde_json::to_value(output).unwrap(),
            serde_json::json!({
                "outputId": "clip-vertical-01",
                "kind": "clip",
                "relativePath": "outputs/clip-vertical-01.mp4",
                "aspectRatio": "9:16",
                "width": 1080,
                "height": 1920,
                "durationMs": 40000,
                "safeAreaPercent": 10.0,
                "privacyStatus": "private",
                "sourceWindow": {"startMs": 60000, "endMs": 100000}
            })
        );
    }

    #[test]
    fn rejects_unknown_project_fields() {
        let project = serde_json::json!({
            "schemaVersion": "1.0",
            "projectId": "project-01",
            "creatorHandle": "@anticaptrad",
            "title": "Concept",
            "assets": [],
            "timeline": [],
            "audio": {"targetLufs": -16.0, "truePeakDb": -1.5, "cues": []},
            "outputs": [],
            "publication": {
                "provider": "youtube",
                "channelHandle": "@anticaptrad",
                "channelId": "UC-Gloecwemo_Mh-VAjnUipg",
                "privacyStatus": "private",
                "rightsConfirmed": true,
                "allowPublic": false
            },
            "review": {"state": "draft"},
            "shellCommand": "forbidden"
        });

        assert!(serde_json::from_value::<CreatorMediaProject>(project).is_err());
    }
}

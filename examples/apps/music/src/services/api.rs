use axum::{
    routing::{get, post},
    response::Json,
    Router,
    extract::Json as ExtractJson,
    extract::Query,
};
use crate::models::{Track, Album, Artist, Video, PlaybackHistory, Playlist};
use nucleus_std::photon::{Model, Op};
use serde::{Deserialize, Serialize};
// use rand::seq::SliceRandom; // Needs cargo add rand
use std::time::{SystemTime, UNIX_EPOCH};

// ...

pub fn routes() -> Router {
    Router::new()
        .route("/api/tracks", get(list_tracks))
        .route("/api/albums", get(list_albums))
        .route("/api/artists", get(list_artists))
        .route("/api/videos", get(list_videos))
        .route("/api/history", post(update_history))
        .route("/api/recommendations", get(get_recommendations))
        .route("/api/playlists", get(list_playlists).post(create_playlist))
        .route("/api/featured", get(get_featured))
        .route("/api/grouped", get(get_grouped_content))
}

// ...

async fn get_featured() -> Json<Option<Video>> {
    // Get all videos
    let videos = Video::query().all::<Video>().await.unwrap_or_default();
    if videos.is_empty() { return Json(None); }
    
    // Simple random pick
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
    let idx = nanos as usize % videos.len();
    Json(Some(videos.into_iter().nth(idx).unwrap()))
}

#[derive(Serialize)]
struct GroupedContent {
    #[serde(rename = "Trending Now")]
    trending: Vec<Video>,
    #[serde(rename = "New Releases")]
    new_releases: Vec<Video>,
    #[serde(rename = "Continue Watching")]
    continue_watching: Vec<Video>, // Simplified, usually from history
}

async fn get_grouped_content() -> Json<GroupedContent> {
    // In a real app, these would be complex distinct queries.
    // For demo, we manipulate the video list.
    let all_videos = Video::query().order_by("id", "DESC").all::<Video>().await.unwrap_or_default();
    
    let trending = all_videos.clone().into_iter().take(5).collect();
    let new_releases = all_videos.clone().into_iter().skip(2).take(5).collect();
    let continue_watching = all_videos.into_iter().take(2).collect();

    Json(GroupedContent {
        trending,
        new_releases,
        continue_watching,
    })
}

async fn list_tracks() -> Json<Vec<Track>> {
    let tracks = Track::query().order_by("id", "DESC").all().await.unwrap_or_default();
    Json(tracks)
}

async fn list_albums() -> Json<Vec<Album>> {
    let albums = Album::query().all().await.unwrap_or_default();
    Json(albums)
}

async fn list_artists() -> Json<Vec<Artist>> {
    let artists = Artist::query().all().await.unwrap_or_default();
    Json(artists)
}

async fn list_videos() -> Json<Vec<Video>> {
    let videos = Video::query().order_by("id", "DESC").all().await.unwrap_or_default();
    Json(videos)
}

async fn list_playlists() -> Json<Vec<Playlist>> {
    let playlists = Playlist::query().order_by("created_at", "DESC").all::<Playlist>().await.unwrap_or_default();
    Json(playlists)
}

#[derive(Deserialize)]
struct CreatePlaylistReq {
    title: String,
    description: Option<String>,
}

async fn create_playlist(ExtractJson(payload): ExtractJson<CreatePlaylistReq>) -> Json<bool> {
    Playlist::create()
        .value("title", payload.title)
        .value("description", payload.description.unwrap_or_default())
        .execute().await.ok();
    Json(true)
}

#[derive(Deserialize)]
struct HistoryUpdate {
    media_type: String,
    media_id: i64,
    position: i64,
}

async fn update_history(ExtractJson(payload): ExtractJson<HistoryUpdate>) -> Json<bool> {
    // Check if exists
    let existing = PlaybackHistory::query()
        .r#where("media_type", payload.media_type.as_str())
        .filter_op("media_id", Op::Eq, payload.media_id)
        .first::<PlaybackHistory>().await.unwrap_or(None);

    if let Some(_hist) = existing {
         PlaybackHistory::create()
            .value("media_type", payload.media_type)
            .value("media_id", payload.media_id)
            .value("position", payload.position)
            .execute().await.ok();
    } else {
        PlaybackHistory::create()
            .value("media_type", payload.media_type)
            .value("media_id", payload.media_id)
            .value("position", payload.position)
            .execute().await.ok();
    }
    Json(true)
}

#[derive(Deserialize)]
struct RecQuery {
    mode: String, // "recent", "mix"
}

async fn get_recommendations(Query(params): Query<RecQuery>) -> Json<Vec<Track>> {
    if params.mode == "recent" {
         let tracks = Track::query().order_by("id", "DESC").limit(10).all().await.unwrap_or_default();
         return Json(tracks);
    }
    // Default or mix
    let tracks = Track::query().limit(5).all().await.unwrap_or_default(); // "Random" (simplified)
    Json(tracks)
}

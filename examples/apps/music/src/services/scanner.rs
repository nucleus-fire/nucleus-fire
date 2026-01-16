use crate::models::{Artist, Album, Track, Video}; 
use crate::services::metadata::MetadataService; 
use nucleus_std::photon::{Model, Op};
use lofty::{Probe, TaggedFileExt, Accessor, AudioFile};
use walkdir::WalkDir;

pub async fn scan_library(root_path: &str) {
    println!("Scanning library at: {}", root_path);

    for entry in WalkDir::new(root_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        
        // Skip hidden files
        if path.file_name().map(|s| s.to_string_lossy().starts_with(".")).unwrap_or(false) {
            continue;
        }

        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
        
        // --- VIDEO SCANNING ---
        if ["mp4", "mkv", "avi", "mov", "webm"].contains(&ext.as_str()) {
            // Check if exists
            let path_str = path.to_string_lossy().to_string();
            let existing = Video::query().r#where("path", path_str.clone()).first::<Video>().await.unwrap_or(None);
            
            if existing.is_none() {
                let (title, year, plot, cast) = MetadataService::enrich_video(&path_str).await;
                println!("Found Video: {}", title);
                
                Video::create()
                    .value("title", title)
                    .value("path", path_str)
                    .value("year", year.unwrap_or(0))
                    .value("plot", plot)
                    .value("cast", cast)
                    .value("duration", 0) // Getting video duration requires ffmpeg/ffprobe usually
                    .execute()
                    .await
                    .ok();
            }
            continue; // Skip audio processing
        }

        // --- AUDIO SCANNING ---
        if !["mp3", "flac", "m4a", "wav"].contains(&ext.as_str()) {
            continue;
        }

        // Read Metadata
        if let Ok(tagged_file) = Probe::open(path).map_err(|_| "err").and_then(|p| p.read().map_err(|_| "err")) {
            let tag = tagged_file.primary_tag().or_else(|| tagged_file.first_tag());
            
            let title = tag.and_then(|t| t.title().map(|s| s.into_owned())).unwrap_or("Unknown Title".to_string());
            let artist_name = tag.and_then(|t| t.artist().map(|s| s.into_owned())).unwrap_or("Unknown Artist".to_string());
            let album_title = tag.and_then(|t| t.album().map(|s| s.into_owned())).unwrap_or("Unknown Album".to_string());
            let track_num = tag.and_then(|t| t.track()).unwrap_or(1) as i64;
            let duration = tagged_file.properties().duration().as_secs() as i64;
            
            // Enrich Genre
            let genre = MetadataService::guess_genre(&artist_name);

            // 1. Get or Create Artist
            let artist_id = get_or_create_artist(&artist_name).await;

            // 2. Get or Create Album
            let album_id = get_or_create_album(&album_title, artist_id).await;

            // 3. Create Track
            // println!("Found Track: {} - {}", artist_name, title);
            
            // Check if track exists by path
            let existing = Track::query()
                .r#where("path", path.to_string_lossy().to_string())
                .first::<Track>().await.unwrap_or(None);

            if existing.is_none() {
                Track::create()
                    .value("title", title)
                    .value("path", path.to_string_lossy().to_string())
                    .value("artist_id", artist_id)
                    .value("album_id", album_id)
                    .value("duration", duration)
                    .value("track_number", track_num)
                    .value("genre", genre) // New field
                    .execute()
                    .await
                    .ok();
            }
        }
    }
    println!("Scan Media complete.");
}

async fn get_or_create_artist(name: &str) -> i64 {
    // Check if exists
    if let Some(artist) = Artist::query().r#where("name", name).first::<Artist>().await.unwrap_or(None) {
        return artist.id;
    }

    // Create and return the new ID
    Artist::create()
        .value("name", name)
        .insert_get_id()
        .await
        .unwrap()
}

async fn get_or_create_album(title: &str, artist_id: i64) -> i64 {
    // Check if exists (composite check roughly)
    if let Some(album) = Album::query()
        .r#where("title", title)
        .filter_op("artist_id", Op::Eq, artist_id)
        .first::<Album>()
        .await
        .unwrap_or(None) {
        return album.id;
    }

    // Create and return the new ID
    Album::create()
        .value("title", title)
        .value("artist_id", artist_id)
        .insert_get_id()
        .await
        .unwrap()
}

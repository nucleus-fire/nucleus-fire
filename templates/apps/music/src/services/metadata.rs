use std::path::Path;

pub struct MetadataService;

impl MetadataService {
    /// Simulate fetching movie metadata
    pub async fn enrich_video(path: &str) -> (String, Option<i64>, String, String) {
        let p = Path::new(path);
        let filename = p.file_stem().unwrap_or_default().to_string_lossy();

        // Simple heuristic: "Movie Title (2025)"
        let mut title = filename.to_string();
        let mut year = None;

        // Try to parse year from end
        if let Some(start_paren) = title.rfind('(') {
            if let Some(end_paren) = title.rfind(')') {
                if end_paren > start_paren {
                    let year_str = &title[start_paren + 1..end_paren];
                    if let Ok(y) = year_str.parse::<i64>() {
                        year = Some(y);
                        title = title[..start_paren].trim().to_string();
                    }
                }
            }
        }

        // Mock Enrichment (Real app would call TMDb)
        let plot = format!("The plot of {} is very exciting and dramatic.", title);
        let cast = r#"["Actor A", "Actor B", "Actor C"]"#.to_string();

        (title, year, plot, cast)
    }

    /// Simulate fetching genre/mood
    pub fn guess_genre(artist: &str) -> String {
        match artist.to_lowercase().as_str() {
            a if a.contains("rock") => "Rock".to_string(),
            a if a.contains("pop") => "Pop".to_string(),
            a if a.contains("jazz") => "Jazz".to_string(),
            a if a.contains("symphony") => "Classical".to_string(),
            _ => "Unknown".to_string(), // Default
        }
    }
}

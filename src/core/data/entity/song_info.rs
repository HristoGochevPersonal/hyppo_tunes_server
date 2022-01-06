pub struct SongInfo {
    pub name: String,
    pub artist: String,
    pub image_path: Option<String>,
}

impl SongInfo {
    pub fn new(name: String, artist: String, image_path: Option<String>) -> Self {
        SongInfo { name, artist, image_path }
    }
}
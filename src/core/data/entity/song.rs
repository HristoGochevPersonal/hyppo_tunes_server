pub struct Song {
    pub name: String,
    pub artist: String,
    pub image_path: Option<String>,
    pub file_path: String,
}

impl Song {
    pub fn new(name: String, artist: String, image_path: Option<String>, file_path: String) -> Self {
        Song { name, artist, image_path, file_path }
    }
}
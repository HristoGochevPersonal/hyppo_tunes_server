use crate::config::CONFIG;
use crate::core::data::context::songs_system_db_context::SongsSystemDbContext;
use crate::core::data::entity::song::Song;
use crate::core::data::entity::song_info::SongInfo;
use itertools::Itertools;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::{fs, process};

lazy_static! {
    pub static ref SONGS_REPOSITORY: SongsRepository = SongsRepository::new();
}

pub struct SongsRepository {
    songs_db_context: SongsSystemDbContext,
}

impl SongsRepository {
    pub fn new() -> Self {
        let db_path = &CONFIG.files_database_path;
        let songs_db_context = match SongsSystemDbContext::new(db_path) {
            Ok(context) => context,
            Err(_) => {
                eprintln!("Could not create db context");
                process::exit(1);
            }
        };
        if fs::create_dir_all(&CONFIG.files_folder_path).is_err() {
            eprintln!("Could not create files directory");
            process::exit(1);
        };
        SongsRepository { songs_db_context }
    }

    pub fn find_song_path(&self, name: &str, artist: &str) -> Option<String> {
        let path = match self.songs_db_context.select_song_file_path(name, artist) {
            Ok(path) => path,
            Err(e) => {
                eprintln!("{}", e);
                return None;
            }
        };

        Some(format!("{}{}", &CONFIG.files_folder_path, path))
    }

    pub fn find_song_infos(&self, keyword: &str, count: usize) -> Vec<SongInfo> {
        match self.songs_db_context.select_song_infos(keyword, count) {
            Ok(song_infos) => song_infos,
            Err(_) => Vec::new(),
        }
    }

    pub fn insert_song(&self, song: Song) -> bool {
        match self.songs_db_context.insert_song(song) {
            Ok(_) => true,
            Err(err) => {
                eprintln!("{}", err);
                false
            }
        }
    }

    pub fn delete_song(&self, song: Song) -> bool {
        match self.songs_db_context.delete_song(song) {
            Ok(_) => true,
            Err(err) => {
                eprintln!("{}", err);
                false
            }
        }
    }

    pub fn fetch_song_from_path(&self, mut path: PathBuf) -> Option<Song> {
        if !matches!(path.extension(), Some(ext) if ext == "mp3") {
            return None;
        }

        let file_full_name = path.file_name().and_then(OsStr::to_str)?.to_string();

        if !path.set_extension("") {
            return None;
        }

        let (artist, name) = path
            .file_name()
            .and_then(OsStr::to_str)
            .and_then(|x| x.split(" - ").collect_tuple())?;

        Some(Song::new(
            name.to_string(),
            artist.to_string(),
            None,
            file_full_name,
        ))
    }

    pub fn auto_update(&self) -> bool {
        let songs_repository = &CONFIG.files_folder_path;
        let path = Path::new(songs_repository);
        let read_dir = match path.read_dir() {
            Ok(iterator) => iterator,
            Err(_) => return false,
        };
        for file_path in read_dir.flatten().map(|file| file.path()) {
            if let Some(song) = self.fetch_song_from_path(file_path) {
                self.insert_song(song);
            }
        }
        true
    }

    pub fn validate(&self) -> bool {
        let songs = match self.songs_db_context.select_all_songs() {
            Ok(songs) => songs,
            Err(_) => return false,
        };
        for song in songs {
            let file_path = Path::new(&song.file_path);
            if !file_path.exists() && !self.delete_song(song) {
                println!("Error while validating")
            }
        }
        true
    }
}

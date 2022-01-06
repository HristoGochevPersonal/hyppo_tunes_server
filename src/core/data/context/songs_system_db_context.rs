use crate::core::data::entity::song::Song;
use crate::core::data::entity::song_info::SongInfo;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use std::sync::{Arc, Mutex};

pub struct SongsSystemDbContext {
    connection_pool: Arc<Mutex<Pool<SqliteConnectionManager>>>,
}

impl SongsSystemDbContext {
    pub fn new(db_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let sqlite_connection_manager = SqliteConnectionManager::file(db_path);
        let sqlite_pool = r2d2::Pool::new(sqlite_connection_manager)?;
        let connection = sqlite_pool.get()?;
        connection.execute(
            "create table if not exists Songs
                (
                    Name       text not null,
                    Artist     text not null,
                    Image_path text,
                    File_path  text not null,
                    primary key (Name, Artist)
                );",
            [],
        )?;
        connection.execute(
            "CREATE VIEW if not exists SongInfos as
                 select Artist || Name as Keyword, Name, Artist, Image_path
                 from Songs;",
            [],
        )?;
        let connection_pool = Arc::new(Mutex::new(sqlite_pool));

        Ok(SongsSystemDbContext { connection_pool })
    }

    pub fn select_song_file_path(
        &self,
        name: &str,
        artist: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let pool = Arc::clone(&self.connection_pool);
        let pool_lock = match pool.lock() {
            Ok(lock) => lock,
            Err(_) => return Err("Database pool poisoned".into()),
        };
        let connection = pool_lock.get()?;

        let mut select_file_path_statement = connection.prepare_cached(
            "select File_path from Songs where Artist like ?1 and Name like ?2 LIMIT 1",
        )?;

        let mut iterator = select_file_path_statement
            .query_map(params![artist, name], |row| {
                let file: String = row.get(0)?;
                Ok(file)
            })?
            .take(1)
            .flatten();

        match iterator.next() {
            None => Err("Could not find song path".into()),
            Some(first_iteration) => Ok(first_iteration),
        }
    }

    pub fn select_song_infos(
        &self,
        keyword: &str,
        count: usize,
    ) -> Result<Vec<SongInfo>, Box<dyn std::error::Error>> {
        let pool = Arc::clone(&self.connection_pool);

        let pool_lock = match pool.lock() {
            Ok(lock) => lock,
            Err(_) => return Err("Database pool poisoned".into()),
        };

        let connection = pool_lock.get()?;

        let mut select_song_infos_statement = connection
            .prepare_cached("select Name,Artist,Image_path from SongInfos where Keyword like ?1")?;

        let params = params![format!("%{}%", keyword)];

        let iterator = select_song_infos_statement
            .query_map(params, |row| {
                let name: String = row.get(0)?;
                let artist: String = row.get(1)?;
                let image_path: Option<String> = row.get(2)?;
                Ok(SongInfo::new(name, artist, image_path))
            })?
            .take(count);

        let mut output = Vec::with_capacity(count);

        for song_info in iterator.flatten() {
            output.push(song_info);
        }

        Ok(output)
    }

    pub fn select_all_songs(&self) -> Result<Vec<Song>, Box<dyn std::error::Error>> {
        let pool = Arc::clone(&self.connection_pool);
        let pool_lock = match pool.lock() {
            Ok(lock) => lock,
            Err(_) => return Err("Database pool poisoned".into()),
        };
        let connection = pool_lock.get()?;
        let mut select_songs_statement =
            connection.prepare_cached("select Name,Artist,File_path from Songs")?;

        let params = params![];

        let output = select_songs_statement
            .query_map(params, |row| {
                let name: String = row.get(0)?;
                let artist: String = row.get(1)?;
                let file_path: String = row.get(2)?;
                Ok(Song::new(name, artist, None, file_path))
            })?
            .flatten()
            .collect();

        Ok(output)
    }

    pub fn delete_song(&self, song: Song) -> Result<(), Box<dyn std::error::Error>> {
        let pool = self.connection_pool.clone();
        let pool_lock = match pool.lock() {
            Ok(lock) => lock,
            Err(_) => return Err("Database pool poisoned".into()),
        };
        let connection = pool_lock.get()?;

        let mut delete_song_statement =
            connection.prepare_cached("delete from Songs where Name like ?1 and Artist like ?2")?;
        delete_song_statement.execute(params![song.name, song.artist])?;

        Ok(())
    }

    pub fn insert_song(&self, song: Song) -> Result<(), Box<dyn std::error::Error>> {
        let pool = self.connection_pool.clone();

        let pool_lock = match pool.lock() {
            Ok(lock) => lock,
            Err(_) => return Err("Database pool poisoned".into()),
        };

        let connection = pool_lock.get()?;

        let mut insert_song_statement = connection.prepare_cached(
            "insert into Songs (Name, Artist, Image_path, File_path) \
            values (?1, ?2, ?3, ?4) \
            on conflict(Name,Artist) \
            do update set Image_path=?3, File_path=?4",
        )?;
        insert_song_statement.execute(params![
            song.name,
            song.artist,
            song.image_path,
            song.file_path
        ])?;

        Ok(())
    }
}

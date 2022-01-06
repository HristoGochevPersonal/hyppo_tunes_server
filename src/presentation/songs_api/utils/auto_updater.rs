use crate::core::repository::songs_repository::SONGS_REPOSITORY;
use hotwatch::blocking::{Flow, Hotwatch};
use hotwatch::notify::DebouncedEvent;
use std::thread;

pub fn start(path: String) -> bool {
    if !SONGS_REPOSITORY.validate() || !SONGS_REPOSITORY.auto_update() {
        return false;
    }

    thread::spawn(|| {
        let mut hotwatch = Hotwatch::new().expect("Could not start auto-updater");
        hotwatch
            .watch(path, |event| {
                match event {
                    DebouncedEvent::Create(path) => {
                        if let Some(song) = SONGS_REPOSITORY.fetch_song_from_path(path) {
                            SONGS_REPOSITORY.insert_song(song);
                        }
                    }
                    DebouncedEvent::Remove(path) => {
                        if let Some(song) = SONGS_REPOSITORY.fetch_song_from_path(path) {
                            SONGS_REPOSITORY.delete_song(song);
                        }
                    }
                    _ => (),
                };
                Flow::Continue
            })
            .expect("Could not start auto-updater");
        hotwatch.run();
    });

    true
}

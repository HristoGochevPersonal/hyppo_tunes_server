#[macro_use]
extern crate lazy_static;

use crate::config::{init_config, CONFIG};
use crate::presentation::songs_api::startup;
use crate::presentation::songs_api::utils::auto_updater;

mod config;
mod core;
mod presentation;

mod songs {
    tonic::include_proto!("songs");
}

mod song_infos {
    tonic::include_proto!("song_infos");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_config();

    if CONFIG.update_automatically && !auto_updater::start(CONFIG.files_folder_path.clone()) {
        panic!("Could not start auto-updater");
    }

    if let Err(e) = startup::start(CONFIG.start_locally, CONFIG.port).await {
        eprintln!("Server error occurred: {}", e);
    }
    Ok(())
}

use core::fmt;
use std::path::Path;
use std::{env, process};

lazy_static! {
    pub static ref CONFIG: Config = create_config();
}

fn create_config() -> Config {
    let arguments: Vec<String> = std::env::args().collect();
    let config = match Config::new(&arguments) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Problem passing arguments:\n{}", e);
            eprintln!("Usage: music-server-rs -p [port: 0-65536] -f [music folder path] -u(Optional) -e(Optional)");
            eprintln!(
                "Where -p represents the port on which the server will be started, default is 8980"
            );
            eprintln!("Where -f represents the server file system root, default is the directory of the server executable");
            eprintln!(
                "Where -u sets whether to automatically update the server data, default is true"
            );
            eprintln!("Where -e sets whether to start the server locally, default is false");
            process::exit(1);
        }
    };
    config
}

pub fn init_config() {
    println!(
        "Port: {}\n\
        File system root: {}\n\
        Update database automatically: {}",
        CONFIG.port, CONFIG.file_system_root, CONFIG.update_automatically
    );
}

pub struct Config {
    pub update_automatically: bool,
    pub start_locally: bool,
    pub port: u16,
    pub file_system_root: String,
    pub files_folder_path: String,
    pub files_database_path: String,
}

impl Config {
    pub fn new(arguments: &[String]) -> Result<Self, String> {
        let mut port: Option<String> = Some(String::from("8980"));
        let mut file_system_root = find_current_dir();
        let mut update_automatically = true;
        let mut start_locally = false;

        for i in 1..arguments.len() {
            let argument: &str = &arguments[i];
            match argument {
                "-e" => start_locally = true,
                "-u" => {
                    if i + 1 < arguments.len() && !arguments[i + 1].starts_with('-') {
                        update_automatically = arguments[i + 1].parse().unwrap_or(true)
                    }
                }
                "-p" => {
                    if i + 1 < arguments.len() && !arguments[i + 1].starts_with('-') {
                        port = Some(arguments[i + 1].clone());
                    }
                }
                "-f" => {
                    if i + 1 < arguments.len() && !arguments[i + 1].starts_with('-') {
                        file_system_root = Some(arguments[i + 1].clone());
                    }
                }
                _ => {}
            }
        }

        // Port
        let port = match port.and_then(|port| port.parse::<u16>().ok()) {
            None => return Err(String::from("Port not specified or illegal")),
            Some(port) => port,
        };

        // Files system root
        let file_system_root = match file_system_root {
            None => return Err(String::from("Music files folder not specified")),
            Some(root) => root,
        };

        let path = Path::new(&file_system_root);
        if !path.exists() {
            return Err(format!("Music files folder missing: {}", file_system_root));
        }

        // Files folder path
        let files_folder_path = format!("{}files/", file_system_root);

        // Files database path
        let files_database_path = format!("{}files_database.sqlite", file_system_root);

        // Create config
        let config = Config {
            update_automatically,
            start_locally,
            port,
            file_system_root,
            files_folder_path,
            files_database_path,
        };
        Ok(config)
    }
}

fn find_current_dir() -> Option<String> {
    env::current_dir()
        .ok()
        .map(|path| path.into_os_string())
        .and_then(|path| path.into_string().ok())
        .map(|path| format!("{}/", path))
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Port: {}\n\
               File system root: {}\n\
               Files folder path: {}\n\
               Files database path: {}\n\
               Update database automatically: {}\n\
               Run for emulator: {}",
            self.port,
            self.file_system_root,
            self.files_folder_path,
            self.files_database_path,
            self.update_automatically,
            self.start_locally
        )
    }
}

use std::fs::Metadata;
use std::future::Future;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub struct AsyncFileReader {
    file: File,
    metadata: Metadata,
}

impl AsyncFileReader {
    pub async fn new(path: &str) -> Option<AsyncFileReader> {
        let file_path = Path::new(path);
        if !file_path.exists() {
            return None;
        }
        let file = File::open(file_path).await.ok()?;
        let metadata = file.metadata().await.ok()?;
        Some(AsyncFileReader { file, metadata })
    }

    pub async fn read_at_once(&mut self) -> Vec<u8> {
        let available_bytes = self.metadata.len() as usize;
        if available_bytes == 0 {
            return Vec::new();
        }
        let mut output = vec![0; available_bytes];
        let read_size = match self.file.read(&mut output).await {
            Ok(read_size) => read_size,
            Err(_) => return Vec::new(),
        };
        if read_size == 0 || read_size != available_bytes {
            Vec::new()
        } else {
            output
        }
    }

    pub async fn start_reading<F, Fut>(&mut self, callback: F) -> bool
    where
        F: Fn(Vec<u8>) -> Fut,
        Fut: Future<Output = bool>,
    {
        let mut read_bytes: u64 = 0;
        let res = loop {
            let available_bytes = self.metadata.len() - read_bytes;
            if available_bytes == 0 {
                break read_bytes == self.metadata.len();
            }
            let bytes_to_read = if available_bytes >= 16384 {
                16384
            } else {
                available_bytes
            } as usize;
            let mut output = vec![0; bytes_to_read];
            let read_size = match self.file.read(&mut output).await {
                Ok(read_size) => read_size,
                Err(_) => break false,
            };
            if read_size == 0 || read_size != bytes_to_read {
                break false;
            } else {
                read_bytes += read_size as u64;
                if !callback(output).await {
                    break false;
                }
            }
        };
        res
    }
}

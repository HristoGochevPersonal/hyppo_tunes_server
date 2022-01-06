use crate::core::repository::songs_repository::SONGS_REPOSITORY;
use crate::presentation::songs_api::utils::async_file_reader::AsyncFileReader;
use crate::songs::{
    songs_service_server::SongsService, Chunk as SongChunk, Request as SongRequest,
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct SongsSenderService;

#[tonic::async_trait]
impl SongsService for SongsSenderService {
    type GetStream = ReceiverStream<Result<SongChunk, Status>>;

    async fn get(
        &self,
        request: Request<SongRequest>,
    ) -> Result<Response<Self::GetStream>, Status> {
        let (tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            let request_ref: &SongRequest = request.get_ref();
            let name: &String = &request_ref.name;
            let artist: &String = &request_ref.artist;

            println!("Received request for song: {}-{}", name, artist);

            let song_path = match SONGS_REPOSITORY.find_song_path(name, artist) {
                Some(path) => path,
                None => {
                    eprintln!("Song could not be found: {}-{}", name, artist);
                    return;
                }
            };

            println!("Starting to send song: {}-{}", name, artist);

            let mut reader = match AsyncFileReader::new(&song_path).await {
                Some(reader) => reader,
                None => {
                    eprintln!("Song could not be read: {}-{}", name, artist);
                    return;
                }
            };

            let mut result_of_reading = reader
                .start_reading(|buffer| async {
                    let chunk = SongChunk {
                        buffer,
                        ready: false,
                    };
                    if let Err(e) = tx.send(Ok(chunk)).await {
                        eprintln!("Error occurred while sending data:\n{}", e);
                        return false;
                    };
                    true
                })
                .await;
            let exit_chunk = SongChunk {
                buffer: Vec::new(),
                ready: result_of_reading,
            };
            if let Err(e) = tx.send(Ok(exit_chunk)).await {
                eprintln!("Error occurred while sending data:\n{}", e);
                result_of_reading = false;
            };
            let result_message = if result_of_reading {
                format!("Successfully sent song: {}-{}", name, artist)
            } else {
                format!("Could not fully send song: {}-{}", name, artist)
            };
            println!("{}", result_message);
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

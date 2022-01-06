use crate::core::repository::songs_repository::SONGS_REPOSITORY;
use crate::presentation::songs_api::utils::async_file_reader::AsyncFileReader;
use crate::song_infos::{
    song_infos_service_server::SongInfosService, Request as SongInfosRequest,
    Response as SongInfosResponse,
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct SongInfosSenderService;

#[tonic::async_trait]
impl SongInfosService for SongInfosSenderService {
    type GetByNameStream = ReceiverStream<Result<SongInfosResponse, Status>>;

    async fn get_by_name(
        &self,
        request: Request<SongInfosRequest>,
    ) -> Result<Response<Self::GetByNameStream>, Status> {
        let (tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            let keyword: &String = &request.get_ref().name;

            println!("Received request for song info: {}", keyword);

            let song_infos = SONGS_REPOSITORY.find_song_infos(keyword, 8);

            if song_infos.is_empty() {
                println!("No matches found for: {}", keyword);
                return;
            }

            println!("Starting to send song infos for: {}", keyword);

            for song_info in song_infos {
                let mut image_bytes: Vec<u8> = Vec::new();
                if let Some(path) = song_info.image_path {
                    if let Some(mut reader) = AsyncFileReader::new(&path).await {
                        image_bytes = reader.read_at_once().await;
                    }
                }
                let song_infos_response = SongInfosResponse {
                    name: song_info.name,
                    artist: song_info.artist,
                    image: image_bytes,
                };
                if let Err(e) = tx.send(Ok(song_infos_response)).await {
                    eprintln!("Error occurred while sending data:\n{}", e);
                };
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tonic::transport::Server;

use crate::presentation::songs_api::services::song_infos_sender_service::SongInfosSenderService;
use crate::presentation::songs_api::services::songs_sender_service::SongsSenderService;
use crate::song_infos::song_infos_service_server::SongInfosServiceServer as SongInfosServiceBuilder;
use crate::songs::songs_service_server::SongsServiceServer as SongsServiceBuilder;

pub async fn start(start_locally: bool, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let ip_address = if start_locally {
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))
    } else {
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))
    };
    let address = SocketAddr::new(ip_address, port);
    println!("Starting server on {}", address);
    let songs_svc = SongsServiceBuilder::new(SongsSenderService);
    let song_infos_svc = SongInfosServiceBuilder::new(SongInfosSenderService);
    Server::builder()
        .add_service(songs_svc)
        .add_service(song_infos_svc)
        .serve(address)
        .await?;

    Ok(())
}


use rspotify::client::Spotify;
use rspotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::util::get_token;

#[tokio::main]
async fn main() {
    let mut spotify_oauth = SpotifyOAuth::default().build();
    match get_token(&mut spotify_oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();

            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let playlist_id = String::from("37i9dQZEVXbeihcByisIXZ");
            let playlists = spotify.playlist(&playlist_id, None, None).await;
            println!("{:?}", playlists);
        }
        None => println!("auth failed"),
    };
}
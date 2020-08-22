use rspotify::client::Spotify;
use rspotify::model::track::{FullTrack, SimplifiedTrack};
use rspotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::util::get_token;

#[derive(Debug)]
struct ExpandedPlaylistEntry {
    playlist_track: FullTrack,
    album_tracks: Vec<SimplifiedTrack>,
}

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
            let playlist_id = "37i9dQZEVXbeihcByisIXZ";
            do_thing(spotify, playlist_id).await;
        }
        None => println!("auth failed"),
    };
}

async fn do_thing(spotify: Spotify, playlist_id: &str) {
    // Also have spotify.user_playlist_tracks which could simplify things
    let playlist = spotify.playlist(playlist_id, None, None).await.unwrap();
    let expanded_playlist = get_expanded_playlist(spotify, playlist).await;
    println!("{:?}", expanded_playlist);
}

async fn get_expanded_playlist(
    spotify: Spotify,
    playlist: rspotify::model::playlist::FullPlaylist,
) -> Vec<ExpandedPlaylistEntry> {
    let mut vec = Vec::new();
    for pt in playlist.tracks.items {
        let playlist_track = pt.track.unwrap();
        let album_tracks = spotify
            .album_track(&playlist_track.album.id.as_ref().unwrap(), 30, 0)
            .await
            .unwrap()
            .items;

        let entry = ExpandedPlaylistEntry {
            playlist_track: playlist_track,
            album_tracks: album_tracks,
        };
        vec.push(entry);
    }

    vec
}

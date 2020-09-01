use rspotify::blocking::client::Spotify;
use rspotify::blocking::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::blocking::util::get_token;
use rspotify::model::track::{FullTrack, SimplifiedTrack};

#[derive(Debug)]
struct ExpandedPlaylistEntry {
    playlist_track: FullTrack,
    album_tracks: Vec<SimplifiedTrack>,
}

use druid::widget::prelude::*;
use druid::widget::{Flex, Label, Padding, Scroll};
use druid::{AppLauncher, LocalizedString, WindowDesc};

pub fn main() {
    let window = WindowDesc::new(build_widget).title(
        LocalizedString::new("spotify-explorer-window-title").with_placeholder("Spotify explorer"),
    );
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(0u32)
        .expect("launch failed");
}

fn build_widget() -> impl Widget<u32> {
    let entries = get_expanded_playlist();
    let mut col = Flex::column();
    for entry in entries {
        let playlist_track_id = entry.playlist_track.id.unwrap();

        for album_track in entry.album_tracks {
            let mut text = String::new();
            if album_track.id.unwrap() == playlist_track_id {
                text.push_str("-> ");
            } else {
                text.push_str("   ");
            }
            text.push_str(&album_track.name);
            col.add_child(Padding::new(3.0, Label::new(text)));
        }
    }

    Scroll::new(col)
}

fn get_expanded_playlist() -> Vec<ExpandedPlaylistEntry> {
    let mut spotify_oauth = SpotifyOAuth::default().build();
    let token_info = get_token(&mut spotify_oauth).expect("auth failed");
    let client_credential = SpotifyClientCredentials::default()
        .token_info(token_info)
        .build();

    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();
    let playlist_id = "37i9dQZEVXbeihcByisIXZ";
    let playlist = spotify.playlist(playlist_id, None, None).unwrap();

    let mut vec = Vec::new();
    for pt in playlist.tracks.items {
        let playlist_track = pt.track.unwrap();
        let album_tracks = spotify
            .album_track(&playlist_track.album.id.as_ref().unwrap(), 30, 0)
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

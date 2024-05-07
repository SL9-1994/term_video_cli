use chrono::prelude::*;
use rusty_ytdl::{Video, VideoOptions, VideoQuality, VideoSearchOptions};
use std::path::PathBuf;

/// Downloads a YouTube video.
///
/// ## Arguments
///
/// * `url` - The URL of the YouTube video.
///
/// ## Returns
///
/// The path to the locally downloaded video.
pub async fn yt_video_downloader(url: String) -> PathBuf {
    let video_url = url;

    // TODO: Allow changing download settings with CLI options.
    let video_options = VideoOptions {
        quality: VideoQuality::Highest,
        filter: VideoSearchOptions::VideoAudio,
        ..Default::default()
    };

    let video = Video::new_with_options(video_url, video_options).unwrap();

    let stream = video.stream().await.unwrap();

    while let Some(_chunk) = stream.chunk().await.unwrap() {
        // println!("{:#?}", _chunk);
    }

    let timestamp = Local::now().format("%Y%m%d%H%M").to_string();

    let output_file_name = format!("{}_yt.mp4", timestamp);

    let video_path = std::path::Path::new(&output_file_name);
    video.download(video_path).await.unwrap();

    return video_path.to_path_buf();
}

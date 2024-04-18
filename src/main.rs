use rusty_ytdl::{Video, VideoOptions, VideoQuality, VideoSearchOptions};
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a video URL as a command line argument.");
        return;
    }

    let video_url = &args[1];

    let video_options = VideoOptions {
        quality: VideoQuality::Highest,
        filter: VideoSearchOptions::VideoAudio,
        ..Default::default()
    };

    // BadApple
    let video = Video::new_with_options(&*video_url, video_options).unwrap();

    let stream = video.stream().await.unwrap();

    while let Some(chunk) = stream.chunk().await.unwrap() {
        println!("{:#?}", chunk);
    }

    // let music_path = std::path::Path::new(r"badapple.mp3");
    // video.download(music_path).await.unwrap();
    let video_path = std::path::Path::new(r"badapple.mkv");
    video.download(video_path).await.unwrap();
}

use serde_json::json;
use youtube_dl::{YoutubeDl, YoutubeDlOutput};

fn main() {
    let output = YoutubeDl::new("https://www.youtube.com/watch?v=zUJiT9Agb1U")
        .youtube_dl_path("yt-dlp")
        .download(false)
        .socket_timeout("15")
        .run()
        .unwrap();
    let video = match output {
        YoutubeDlOutput::SingleVideo(video) => video,
        _ => panic!("single video should not be a playlist"),
    };
    println!("Video title: {}", json!(video));
}

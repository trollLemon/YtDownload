mod app;
mod yt_downloader;

use app::YoutubeVidDownloader;
use eframe::{egui::vec2, run_native, NativeOptions};
use tokio;

#[tokio::main]
async fn main() {
    let app = YoutubeVidDownloader::default();
    let mut window_options = NativeOptions::default();
    window_options.initial_window_size = Some(vec2(520.0, 220.0));
    window_options.resizable = true;
    run_native(Box::new(app), window_options);
}

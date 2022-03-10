pub mod yt_downloader {

    //TODO: indicate to front end if the video failed to download
    //TODO: also make a duplicate of

    use rustube::url::Url;
    use rustube::Result;
    use rustube::Video;
    use std::path::PathBuf;

    pub enum DownloadOption {
        Video,
        Audio,
        Standard,
    }

    pub struct VideoTarget {
        url: String,
        option: DownloadOption,
        path: String,
    }

    impl VideoTarget {
        pub fn new(link: String, dl_option: DownloadOption, desired_path: String) -> Self {
            Self {
                url: link,
                option: dl_option,
                path: desired_path,
            }
        }
    }

    pub async fn download(video_selection: VideoTarget) -> Result<PathBuf> {
        let url = Url::parse(&video_selection.url)?;
        let video: Video = Video::from_url(&url).await?;
        let dir = PathBuf::from(&video_selection.path);

        let stream = video
            .streams()
            .iter()
            .filter(|stream| stream.includes_video_track && stream.includes_audio_track)
            .max_by_key(|stream| stream.quality_label)
            .unwrap();

        let b = stream.download_to_dir(dir);
        b.await
    }
}

#[cfg(test)]
mod tests {

    use rustube::tokio;

    use crate::yt_downloader::yt_downloader::{download, DownloadOption, VideoTarget};

    #[tokio::test]
    async fn downloader() {
        let video = VideoTarget::new(
            "https://www.youtube.com/watch?v=2ZIpFytCSVc".to_string(),
            DownloadOption::Standard,
            "test_download_path".to_string(),
        );

        download(video).await;
    }

    #[tokio::test]
    async fn downloader_with_age_restricted_video() {
        let video = VideoTarget::new(
            "https://www.youtube.com/watch?v=3OIRHM6-M3I".to_string(),
            DownloadOption::Standard,
            "test_download_path".to_string(),
        );

        download(video).await;
    }
}

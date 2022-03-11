pub mod yt_downloader {

    //TODO: indicate to front end if the video failed to download
    //TODO: also make a duplicate of

    use rustube::url::Url;
    use rustube::Result;

    use rustube::Video;
    use std::path::PathBuf;

    pub struct VideoTarget {
        url: String,
        path: String,
    }

    impl VideoTarget {
        pub fn new(link: String, desired_path: String) -> Self {
            Self {
                url: link,
                path: desired_path,
            }
        }
    }

    pub async fn download_video(video_selection: VideoTarget) -> Result<PathBuf> {
        let url = Url::parse(&video_selection.url)?;
        let video: Video = Video::from_url(&url).await?;
        let dir = PathBuf::from(&video_selection.path);

        let stream = video
            .streams()
            .iter()
            .filter(|stream| stream.includes_video_track && stream.includes_audio_track)
            .max_by_key(|stream| stream.quality_label)
            .unwrap();

        let dl_video = stream.download_to_dir(dir);
        dl_video.await
    }

    pub async fn download_audio(video_selection: VideoTarget) -> Result<PathBuf> {
        let url = Url::parse(&video_selection.url)?;
        let video: Video = Video::from_url(&url).await?;
        let dir = PathBuf::from(&video_selection.path);

        let stream = video
            .streams()
            .iter()
            .filter(|stream| !stream.includes_video_track && stream.includes_audio_track)
            .max_by_key(|stream| stream.quality_label)
            .unwrap();

        let dl_video = stream.download_to_dir(dir);
        dl_video.await
    }
}

#[cfg(test)]
mod tests {

    use crate::yt_downloader::yt_downloader::{download_audio, download_video, VideoTarget};
    use rustube::tokio;

    #[tokio::test]
    async fn downloader() {
        let video = VideoTarget::new(
            "https://www.youtube.com/watch?v=2ZIpFytCSVc".to_string(),
            "test_download_path".to_string(),
        );

        download_video(video).await;
    }

    #[tokio::test]
    async fn downloader_audio() {
        let video = VideoTarget::new(
            "https://www.youtube.com/watch?v=2ZIpFytCSVc".to_string(),
            "test_download_path".to_string(),
        );

        download_audio(video).await;
    }

    #[tokio::test]
    //Should not attempt to download the vid since its age restricted and do nothing without killing the app
    async fn downloader_with_age_restricted_video() {
        let video = VideoTarget::new(
            "https://www.youtube.com/watch?v=3OIRHM6-M3I".to_string(),
            "test_download_path".to_string(),
        );

        download_video(video).await;
    }
}

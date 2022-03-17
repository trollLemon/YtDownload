/*

figure out the operating system and adjust the default install location
*/
mod get_os {
    pub fn determine_default_path() -> &'static str {
        if cfg!(windows) {
            "C:\\Users\\*user name*\\Downloads"
        } else {
            "/home/*user name*/Downloads/"
        }
    }
}

/*
    Renders all of the GUI stuff
*/

use eframe::{
    egui::{self, FontData},
    egui::{Context, FontDefinitions},
    epaint::FontFamily,
    epi::App,
};

use crate::yt_downloader::yt_downloader::{self, VideoTarget};
use poll_promise::Promise;
use rustube::Result;
use std::path::PathBuf;

pub struct YoutubeVidDownloader {
    input: String,
    download_to: String,
    task: Option<poll_promise::Promise<Result<PathBuf>>>,
    download_video: bool,
}

impl Default for YoutubeVidDownloader {
    fn default() -> Self {
        Self {
            input: "url".to_owned(),
            download_to: get_os::determine_default_path().to_owned(),
            task: None::<Promise<Result<PathBuf>>>,
            download_video: true,
        }
    }
}

impl App for YoutubeVidDownloader {
    fn update(&mut self, ctx: &egui::Context, _frame: &eframe::epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(30.0);
            ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);
            ui.visuals_mut().override_text_color = Some(egui::Color32::DARK_BLUE);
            ui.label("URL");
            ui.add_space(10.0);

            ui.text_edit_singleline(&mut self.input);

            ui.add_space(10.0);
            ui.label("Download Location");
            ui.text_edit_singleline(&mut self.download_to);

            match &self.task {
                Some(promise) => match promise.poll() {
                    std::task::Poll::Ready(_) => {
                        ui.colored_label(egui::Color32::DARK_BLUE, "downloaded!");
                    }
                    std::task::Poll::Pending => {
                        ui.add(egui::Spinner::new());
                    }
                },
                None => {
                    ui.colored_label(egui::Color32::DARK_BLUE, "download something");
                }
            }
        });

        egui::TopBottomPanel::top("top_pannel").show(ctx, |ui| {
            ui.heading("Youtube Video Downloader");
        });

        egui::TopBottomPanel::bottom("bottom_pannel").show(ctx, |ui| {
            ui.hyperlink_to(
                "<-- Github repo -->",
                "https://github.com/trollLemon/YtDownload",
            );
        });

        egui::SidePanel::right("side_panel").show(ctx, |ui| {
            ui.add_space(50.0);
            ui.radio_value(&mut self.download_video, true, "Download Video");
            ui.radio_value(&mut self.download_video, false, "Download audio");
            if ui.button("Download").clicked() {
                let url = &mut self.input;
                let path = &mut self.download_to;
                let video = VideoTarget::new(url.to_string(), path.to_string());

                if self.download_video {
                    self.task = Some(Promise::spawn_async(async move {
                        yt_downloader::download_video(video).await
                    }));
                } else {
                    self.task = Some(Promise::spawn_async(async move {
                        yt_downloader::download_audio(video).await
                    }));
                }
            }

            ui.add_space(5.0);

            if ui.button("Choose Download Location").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.download_to = path.display().to_string();
                }
            }
        });
    }

    fn name(&self) -> &str {
        "Youtube Video Downloader"
    }

    fn setup(
        &mut self,
        ctx: &eframe::egui::Context,
        _frame: &eframe::epi::Frame,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        self.load_fonts(ctx);
    }
}

impl YoutubeVidDownloader {
    fn load_fonts(&self, ctx: &Context) {
        let mut font = FontDefinitions::default();
        font.font_data.insert(
            "Saira".to_owned(),
            FontData::from_static(include_bytes!(
                "../assets/fonts/Saira-VariableFont_wdth,wght.ttf"
            )),
        );
        font.families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "Saira".to_owned());
        ctx.set_fonts(font);
    }
}

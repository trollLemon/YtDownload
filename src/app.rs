/*
    Renders all of the GUI stuff
*/

use eframe::{
    egui::{self, FontData},
    egui::{Context, FontDefinitions},
    epaint::FontFamily,
    epi::App,
};

pub struct YoutubeVidDownloader {
    input: String,
    download_to: String,
}

impl Default for YoutubeVidDownloader {
    fn default() -> Self {
        Self {
            input: "url".to_owned(),
            download_to: "~/Downloads".to_owned(),
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
        });

        egui::TopBottomPanel::top("top_pannel").show(ctx, |ui| {
            ui.heading("Youtube Video Downloader");
        });

        egui::TopBottomPanel::bottom("bottom_pannel").show(ctx, |ui| {
            ui.hyperlink_to(
                "<-- Github repo -->",
                "https://github.com/trollLemon/YtDownload",
            );
            //TODO: add status box and progress bar
        });

        egui::SidePanel::right("side_panel").show(ctx, |ui| {
            ui.add_space(50.0);
            if ui.button("Download").clicked() {
                println!("IMPLEMENT ME");
            }

            ui.add_space(30.0);

            if ui.button("Choose Download Location").clicked() {
                println!("IMPLEMENT ME");
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

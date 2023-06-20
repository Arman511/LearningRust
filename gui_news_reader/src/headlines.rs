use eframe::egui::{Color32, FontDefinitions, CtxRef, FontFamily, Label, TextStyle, Hyperlink, Separator, Layout};
use std::borrow::Cow;

pub struct Headlines {
    articles: Vec<NewsCardData>,
}
pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

impl Headlines {
    pub fn new() -> Headlines {
        let iter = (0..10).map(|a| NewsCardData {
            title: format!("title{}", a),
            description: format!("desc{}", a),
            link: format!("link{}", a),
        });
        Headlines {
            articles: Vec::from_iter(iter),
        }
    }

    pub fn configure_font(&self, ctx: &CtxRef) {
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "MesloLGS".to_string(),
            Cow::Borrowed(include_bytes!("../MesloLGS_NF_Regular.ttf")),
        );
        font_def
            .family_and_size
            .insert(TextStyle::Heading, (FontFamily::Proportional, 35.));
        font_def
            .family_and_size
            .insert(TextStyle::Body, (FontFamily::Proportional, 20.));
        font_def
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "MesloLGS".to_string());

        ctx.set_fonts(font_def);
    }

    pub fn render_news_card(&self, ui: &mut eframe::egui::Ui) {
        for a in &self.articles {
            // render title
            ui.add_space(PADDING);
            let title = format!("> {}", a.title);
            ui.colored_label(WHITE, title);
            // render desc
            ui.add_space(PADDING);
            let desc = Label::new(&a.description).text_style(TextStyle::Button);
            ui.add(desc);
            // render link
            ui.style_mut().visuals.hyperlink_color = CYAN;
            ui.add_space(PADDING);
            ui.with_layout(Layout::right_to_left(), |ui| {
                ui.add(Hyperlink::new(&a.link).text("Read More"))
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }
}

struct NewsCardData {
    title: String,
    description: String,
    link: String,
}

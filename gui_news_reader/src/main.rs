mod headlines;
use headlines::{Headlines, PADDING};

use eframe::{
    egui::{CentralPanel, ScrollArea, Ui, Vec2, Separator, TopBottomPanel, CtxRef, Label, Hyperlink},
    epi::App,
    run_native, NativeOptions,
};

impl App for Headlines {
    fn setup(
        &mut self,
        ctx: &eframe::egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        self.configure_font(ctx);
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {
            render_header(ui);
            ScrollArea::auto_sized().show(ui, |ui| {
                self.render_news_card(ui);
            });
            render_footer(ctx);
        });
    }

    fn name(&self) -> &str {
        "Headlines"
    }
}

fn render_footer( ctx: &CtxRef) {
TopBottomPanel::bottom("footer").show(ctx, |ui| {
    ui.vertical_centered(|ui|{
        ui.add_space(10.);
        ui.add(Label::new("API Source: newsdata.io").monospace());
        ui.add(Hyperlink::new("https.github.com/emilk/equi").text("Made with equi").text_style(eframe::egui::TextStyle::Monospace));
        ui.add(Hyperlink::new(""))
        ui.add_space(10.);
    
    })
});
}
fn render_header(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Headlines");
    });
    ui.add_space(PADDING);
    let sep = Separator::default().spacing(20.);
    ui.add(sep);
}

fn main() {
    let app = Headlines::new();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(360., 640.));

    run_native(Box::new(app), win_option);
}

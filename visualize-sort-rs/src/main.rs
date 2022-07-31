use notan::draw::*;
use notan::egui::{self, *};
use notan::prelude::*;

#[derive(AppState)]
struct State {
    font: Font,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .add_config(EguiConfig)
        .add_config(DrawConfig)
        .draw(draw)
        .build()
}

fn setup(gfx: &mut Graphics) {}

fn draw(gfx: &mut Graphics, plugins: &mut Plugins) {
    let mut output = plugins.egui(|ctx| {
        egui::Window::new("Statistics").show(ctx, |ctx| egui::RichText::new("text"));
    });

    gfx.render(&output);
}

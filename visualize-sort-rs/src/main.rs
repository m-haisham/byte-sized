mod algorithm;
mod algorithms;
mod event;
mod report;
mod sync;

use crate::algorithms::algorithms;
use notan::draw::*;
use notan::egui::{self, *};
use notan::prelude::*;
use sync::SyncVec;

#[derive(AppState)]
struct State {
    sync: SyncVec,
    update_duration: f32,
    update_timer: f32,
}

#[notan_main]
fn main() -> Result<(), String> {
    let window_config = WindowConfig::new()
        .title("Visualize sort")
        .size(1280, 720)
        .vsync()
        .resizable();

    notan::init_with(setup)
        .add_config(window_config)
        .add_config(EguiConfig)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn setup() -> State {
    State {
        sync: SyncVec::new(100, algorithms()[0].clone()),
        update_duration: 0.1,
        update_timer: 0.0,
    }
}

fn update(app: &mut App, state: &mut State) {
    state.update_timer += app.timer.delta_f32();
    if state.update_timer >= state.update_duration {
        state.update_timer = 0.0;
        state.sync.try_apply();
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let mut draw = gfx.create_draw();

    draw.clear(Color::BLACK);

    {
        let parent_size = gfx.size();
        let parent_height = parent_size.1 as f32;

        let lookup = state.sync.lookup();

        let bar_width = parent_size.0 as f32 / state.sync.values().len() as f32;
        for (offset, value) in state.sync.values().iter().enumerate() {
            let bar_size = (bar_width, value * parent_height);

            let bar_y = parent_height - bar_size.1;
            let bar_x = bar_size.0 * offset as f32;

            let mut bar = draw.rect((bar_x, bar_y), bar_size);

            if lookup.writes_contains(&offset) {
                bar.color(Color::RED);
            } else if lookup.accesses_contains(&offset) {
                bar.color(Color::GREEN);
            } else {
                bar.color(Color::WHITE);
            }
        }
    }

    let output = plugins.egui(|ctx| {
        egui::Window::new("Stats").show(ctx, |ui| draw_egui_ui(ui, app));
    });

    gfx.render(&draw);
    gfx.render(&output);
}

fn draw_egui_ui(ui: &mut egui::Ui, app: &mut App) {
    egui::Grid::new("stat_grid")
        .num_columns(2)
        .spacing([40.0, 6.0])
        .show(ui, |ui| {
            ui.label("Fps");
            ui.label(format!("{:.2}", app.timer.fps()));
            ui.end_row();

            // TODO: add fullscreen logic
            ui.label("Fullscreen");
            ui.checkbox(&mut false, "");
            ui.end_row();
        });
}

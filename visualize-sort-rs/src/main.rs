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
    update: Update,
}

struct Update {
    paused: bool,
    duration: f32,
    timer: f32,
}

impl Default for Update {
    fn default() -> Self {
        Self {
            paused: false,
            duration: 0.1,
            timer: 0.0,
        }
    }
}

impl Update {
    fn should_update(&mut self, delta: f32) -> bool {
        if self.paused {
            return false;
        }

        if self.timer >= self.duration {
            self.timer = 0.0;
            true
        } else {
            self.timer += delta;
            false
        }
    }
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
        update: Update::default(),
    }
}

fn update(app: &mut App, state: &mut State) {
    if state.update.should_update(app.timer.delta_f32()) {
        state.sync.next();
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let mut draw = gfx.create_draw();

    draw.clear(Color::BLACK);
    draw_bars(&mut draw, gfx, state);

    let output = plugins.egui(|ctx| {
        egui::Window::new("Stats").show(ctx, |ui| draw_egui_ui(ui, app, state));
    });

    gfx.render(&draw);
    gfx.render(&output);
}

fn draw_bars(draw: &mut Draw, gfx: &mut Graphics, state: &mut State) {
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
            bar.color(Color::RED);
        } else {
            bar.color(Color::WHITE);
        }
    }
}

fn draw_egui_ui(ui: &mut egui::Ui, app: &mut App, state: &mut State) {
    egui::Grid::new("stat_grid")
        .num_columns(2)
        .spacing([40.0, 6.0])
        .show(ui, |ui| {
            ui.label("Fps");
            ui.label(format!("{:.2}", app.timer.fps()));
            ui.end_row();

            ui.label("Fullscreen");
            {
                let mut is_fullscreen = app.window().is_fullscreen();
                if ui.checkbox(&mut is_fullscreen, "").clicked() {
                    app.window().set_fullscreen(is_fullscreen);
                };
            }
            ui.end_row();

            ui.heading(state.sync.name());
            ui.end_row();

            ui.label("Accesses");
            ui.colored_label(Color32::GREEN, format!("{}", state.sync.accesses()));
            ui.end_row();

            ui.label("Writes");
            ui.colored_label(Color32::RED, format!("{}", state.sync.writes()));
            ui.end_row();

            ui.label("Paused");
            ui.checkbox(&mut state.update.paused, "");
            ui.end_row();

            ui.label("Speed");
            ui.add(
                DragValue::new(&mut state.update.duration)
                    .speed(0.005)
                    .max_decimals(3)
                    .clamp_range(0.0..=1.0),
            );
            ui.end_row();
        });
}

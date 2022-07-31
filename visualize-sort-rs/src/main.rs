mod algorithm;
mod algorithms;
mod event;
mod report;

use notan::draw::*;
use notan::egui::{self, *};
use notan::prelude::*;
use rand::prelude::SliceRandom;

#[derive(AppState)]
struct State {
    values: Vec<f32>,
    current: usize,

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

macro_rules! vec_uniform {
    ($t:ty, $c:expr) => {{
        let mut output = Vec::<$t>::new();
        let segment_value = 1.0 / $c as $t;
        for offset in 0..$c {
            output.push(segment_value * offset as $t);
        }
        output
    }};
}

fn setup(gfx: &mut Graphics) -> State {
    let mut values = vec_uniform!(f32, gfx.size().0);
    values.shuffle(&mut rand::thread_rng());

    State {
        values,
        current: 0,

        update_duration: 0.01,
        update_timer: 0.0,
    }
}

fn update(app: &mut App, state: &mut State) {
    state.update_timer += app.timer.delta_f32();
    if state.update_timer >= state.update_duration {
        state.update_timer = 0.0;

        // if state.current < state.scripts.history.len() - 1 {
        //     state.current += 1;
        // }
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let mut draw = gfx.create_draw();

    draw.clear(Color::BLACK);

    // {
    //     let event = &state.scripts.history[state.current];

    //     let parent_size = gfx.size();
    //     let parent_height = parent_size.1 as f32;

    //     let bar_width = parent_size.0 as f32 / event.snapshot.len() as f32;
    //     for (offset, value) in event.snapshot.iter().enumerate() {
    //         let bar_size = (bar_width, value * parent_height);

    //         let bar_y = parent_height - bar_size.1;
    //         let bar_x = bar_size.0 * offset as f32;

    //         let mut bar = draw.rect((bar_x, bar_y), bar_size);

    //         if event.writes.contains(&offset) {
    //             bar.color(Color::RED);
    //         } else if event.accesses.contains(&offset) {
    //             bar.color(Color::GREEN);
    //         } else {
    //             bar.color(Color::WHITE);
    //         }
    //     }
    // }

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

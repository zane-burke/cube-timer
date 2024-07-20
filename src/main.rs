#![recursion_limit = "1024"]

mod history;
mod seq_gen;
mod stats;
mod timer;
mod utils;
mod shuffle;
mod preferences;
mod app;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<app::App>::new().render();
}

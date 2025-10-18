// src/main.rs
//
use leptos::mount::mount_to_body;

mod app;
mod components;
mod pages;
pub mod settings_store;

use app::App;

fn main() {
    mount_to_body(App);
}

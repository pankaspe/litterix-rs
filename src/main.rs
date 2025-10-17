use leptos::mount::mount_to_body;

mod app;
mod components;
mod pages;

use app::App;

fn main() {
    mount_to_body(App);
}

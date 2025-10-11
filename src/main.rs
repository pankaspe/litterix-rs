use leptos::mount::mount_to_body;
use leptos::prelude::*;

mod components;

use components::Bio;
use components::Gallery;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="main">
            <div class="left-panel">
                <Bio />
            </div>
            <div class="right-panel">
                <Gallery />
            </div>
        </div>
    }
}

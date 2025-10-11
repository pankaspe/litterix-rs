use leptos::mount::mount_to_body;
use leptos::prelude::*;

mod components;

use components::Counter;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div>
            <Counter />
        </div>
    }
}

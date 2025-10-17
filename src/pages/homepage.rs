// src/pages/homepage.rs
//
use crate::components::ZenMode;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <main class="container home">
            <div class="home__intro">
                <div class="home__badge">
                    <span>"🦀"</span>
                    <span>"rust + wasm"</span>
                </div>

                <h1 class="home__title">
                    <span class="home__title-highlight">"Litterix"</span>
                </h1>

                <p class="home__subtitle">
                    "allenati, migliora la tua velocità, batti i tuoi record"
                </p>
            </div>

            <div class="home__game">
                <ZenMode />
            </div>
        </main>
    }
}

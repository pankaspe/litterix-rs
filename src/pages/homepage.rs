// src/pages/homepage.rs
//
use crate::components::Game;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <main class="container home">

        // Il componente Game ora è un figlio diretto del layout 'home'
            <Game />

        </main>
    }
}

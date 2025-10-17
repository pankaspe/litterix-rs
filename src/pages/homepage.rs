// src/pages/homepage.rs
//
use crate::components::Game;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <main class="container home">

            // Sostituiamo l'intero blocco 'home__intro' con una singola frase
            <p class="home__tagline">
                "Ritmo. Precisione. Velocità."
            </p>

            // Il componente Game ora è un figlio diretto del layout 'home'
            <Game />

        </main>
    }
}

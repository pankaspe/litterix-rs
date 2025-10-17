use crate::components::{RushMode, ZenMode};
use leptos::prelude::*;
use web_sys::window;

// Un enum per rappresentare lo stato in modo pulito e sicuro
#[derive(Clone, Copy, PartialEq)]
enum GameMode {
    Zen,
    Rush,
}

#[component]
pub fn Game() -> impl IntoView {
    // Leggiamo il frammento (#) dall'URL per impostare lo stato iniziale.
    // Questo permette di condividere link diretti a una modalità specifica.
    let get_initial_mode = || {
        if let Some(win) = window() {
            if let Ok(hash) = win.location().hash() {
                return match hash.as_str() {
                    "#rush" => GameMode::Rush,
                    _ => GameMode::Zen, // Default su Zen se l'hash è #zen o sconosciuto
                };
            }
        }
        GameMode::Zen // Fallback se non siamo in un browser
    };

    // Creiamo un signal per tenere traccia della modalità attiva
    let (active_mode, set_active_mode) = signal(get_initial_mode());

    view! {
        <div class="game-container">
            // --- Selettore a Tab ---
            <div class="game-tabs">
                <a
                    href="#zen"
                    class="game-tab"
                    // Applica la classe 'active' dinamicamente
                    class:active=move || active_mode.get() == GameMode::Zen
                    // Al click, aggiorna il nostro signal
                    on:click=move |_| set_active_mode.set(GameMode::Zen)
                >
                    "Zen Mode"
                </a>
                <a
                    href="#rush"
                    class="game-tab"
                    class:active=move || active_mode.get() == GameMode::Rush
                    on:click=move |_| set_active_mode.set(GameMode::Rush)
                >
                    "Rush Mode"
                </a>
            </div>

            // --- Area di Gioco ---
            // Renderizza il componente corretto in base allo stato del signal
            <div class="game-content">
                {move || match active_mode.get() {
                    GameMode::Zen => view! { <ZenMode /> }.into_any(),
                    GameMode::Rush => view! { <RushMode /> }.into_any(),
                }}
            </div>
        </div>
    }
}

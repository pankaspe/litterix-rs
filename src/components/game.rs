// src/components/game.rs
//
use crate::components::{MarathonMode, RushMode, ZenMode};
use leptos::prelude::*;
use web_sys::window;

// Un enum per rappresentare lo stato in modo pulito e sicuro
#[derive(Clone, Copy, PartialEq)]
enum GameMode {
    Zen,
    Rush,
    Marathon,
}

#[component]
pub fn Game() -> impl IntoView {
    // Leggiamo il frammento (#) dall'URL per impostare lo stato iniziale
    let get_initial_mode = || {
        if let Some(win) = window() {
            if let Ok(hash) = win.location().hash() {
                return match hash.as_str() {
                    "#rush" => GameMode::Rush,
                    "#marathon" => GameMode::Marathon,
                    _ => GameMode::Zen,
                };
            }
        }
        GameMode::Zen
    };

    let (active_mode, set_active_mode) = signal(get_initial_mode());

    view! {
        <div class="game-container">
            <div class="game-tabs">
                <div class="game-tabs__inner-wrapper">
                    <a
                        href="#zen"
                        class="game-tab"
                        class:active=move || active_mode.get() == GameMode::Zen
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
                    <a
                        href="#marathon"
                        class="game-tab"
                        class:active=move || active_mode.get() == GameMode::Marathon
                        on:click=move |_| set_active_mode.set(GameMode::Marathon)
                    >
                        "Marathon"
                    </a>
                </div>
            </div>

            <div class="game-content">
                {move || match active_mode.get() {
                    GameMode::Zen => view! { <ZenMode /> }.into_any(),
                    GameMode::Rush => view! { <RushMode /> }.into_any(),
                    GameMode::Marathon => view! { <MarathonMode /> }.into_any(),
                }}
            </div>
        </div>
    }
}

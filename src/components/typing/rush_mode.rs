use crate::components::typing::{MetricsBar, TypingEngine};
use leptos::prelude::*;
use std::time::Duration;

// Costanti per configurare facilmente la modalità
const RUSH_PHRASES: &[&str] = &[
    "il tempo scorre come sabbia tra le dita",
    "la pressione rivela il vero talento",
    "ogni secondo è prezioso non sprecarlo",
    "la precisione è la chiave della velocità",
    "mantieni la calma e continua a digitare",
    "supera i tuoi limiti una frase alla volta",
    "la tastiera attende il tuo comando rapido",
    "senti il ritmo aumenta la cadenza",
];
const INITIAL_TIME: f64 = 20.0;
const ACCURACY_BONUS_PERFECT: f64 = 5.0; // 100%
const ACCURACY_BONUS_HIGH: f64 = 3.0; // > 75%
const ACCURACY_BONUS_MEDIUM: f64 = 2.0; // > 50%
const ACCURACY_BONUS_LOW: f64 = 1.0; // > 25%

#[derive(Clone, PartialEq, Debug)]
enum GameState {
    Pending,  // In attesa che l'utente inizi a digitare
    Running,  // Gioco in corso
    Finished, // Il tempo è scaduto
}

#[component]
pub fn RushMode() -> impl IntoView {
    // --- State Signals ---
    let (game_state, set_game_state) = signal(GameState::Pending);
    let (phrase_index, set_phrase_index) = signal(0_usize);
    let (time_remaining, set_time_remaining) = signal(INITIAL_TIME);

    // Statistiche cumulative della sessione
    let (total_words_typed, set_total_words_typed) = signal(0_usize);
    let (total_chars_typed, set_total_chars_typed) = signal(0_usize);

    // Statistiche dell'ultima frase per la UI
    let (last_wpm, set_last_wpm) = signal(0.0);
    let (last_accuracy, set_last_accuracy) = signal(100.0);

    // --- Timer Logic ---
    Effect::new(move |_| {
        if game_state.get() == GameState::Running {
            // Avvia l'intervallo solo quando il gioco è in esecuzione
            let handle = set_interval_with_handle(
                move || {
                    set_time_remaining.update(|t| *t -= 0.1);
                    if time_remaining.get() <= 0.0 {
                        set_time_remaining.set(0.0);
                        set_game_state.set(GameState::Finished);
                    }
                },
                Duration::from_millis(100),
            )
            .unwrap();

            // Pulisci l'intervallo quando l'effetto viene rieseguito o il componente smontato
            on_cleanup(move || handle.clear());
        }
    });

    // --- Callback Handlers ---
    let on_char_typed = Callback::new(move |_: ()| {
        // Avvia il gioco al primo carattere digitato
        if game_state.get() == GameState::Pending {
            set_game_state.set(GameState::Running);
        }
        set_total_chars_typed.update(|c| *c += 1);
    });

    let on_word_typed = Callback::new(move |_: ()| {
        set_total_words_typed.update(|w| *w += 1);
    });

    let on_word_deleted = Callback::new(move |_: ()| {
        set_total_words_typed.update(|w| {
            if *w > 0 {
                *w -= 1;
            }
        });
    });

    let on_complete = Callback::new(move |(wpm, accuracy): (f64, f64)| {
        set_last_wpm.set(wpm);
        set_last_accuracy.set(accuracy);

        // Calcola e aggiungi il bonus di tempo in base all'accuratezza
        let time_bonus = match accuracy {
            a if a == 100.0 => ACCURACY_BONUS_PERFECT,
            a if a > 75.0 => ACCURACY_BONUS_HIGH,
            a if a > 50.0 => ACCURACY_BONUS_MEDIUM,
            a if a > 25.0 => ACCURACY_BONUS_LOW,
            _ => 0.0,
        };
        set_time_remaining.update(|t| *t += time_bonus);

        // Passa alla frase successiva
        // --- CORREZIONE 1 QUI ---
        set_phrase_index.update(|i| *i += 1);
    });

    let restart_game = move |_| {
        set_game_state.set(GameState::Pending);
        set_time_remaining.set(INITIAL_TIME);
        set_phrase_index.set(0);
        set_total_words_typed.set(0);
        set_total_chars_typed.set(0);
        set_last_wpm.set(0.0);
        set_last_accuracy.set(100.0);
    };

    // --- Render Logic ---
    view! {
        <div class="rush-mode">

            <div class="rush-header">
                <span class="rush-icon">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-gauge-icon lucide-gauge"><path d="m12 14 4-4"/><path d="M3.34 19a10 10 0 1 1 17.32 0"/></svg>
                </span>
                <h2 class="rush-title">"Rush"</h2>
                <p class="rush-description">
                    "Una corsa contro il tempo. L'accuratezza è la chiave per guadagnare secondi preziosi."
                </p>
            </div>


            <MetricsBar
                wpm=Signal::derive(move || last_wpm.get())
                accuracy=Signal::derive(move || last_accuracy.get())
                chars_typed=Signal::derive(move || total_chars_typed.get())
                words_typed=Signal::derive(move || total_words_typed.get())
                timer=Signal::derive(move || time_remaining.get())
            />

            <div class="rush-typing-area">
                // --- NUOVA LOGICA CON <Show> ---

                // Mostra la schermata di Game Over solo quando lo stato è Finished
                <Show
                    when=move || game_state.get() == GameState::Finished
                    fallback=|| () // Non fare nulla quando non è "Finished"
                >
                    <div class="rush-game-over">
                        <h3>"Tempo Scaduto!"</h3>
                        <div class="rush-final-stats">
                            <p><strong>"Parole Totali:"</strong> {total_words_typed.get()}</p>
                            <p><strong>"Frasi Completate:"</strong> {phrase_index.get()}</p>
                        </div>
                        <button class="rush-play-again-button" on:click=restart_game>
                            "Gioca Ancora"
                        </button>
                    </div>
                </Show>

                // Mostra il TypingEngine finché lo stato NON è Finished.
                // In questo modo, il componente non viene ricreato passando da Pending a Running.
                <Show
                    when=move || game_state.get() != GameState::Finished
                    fallback=|| () // Non fare nulla quando è "Finished"
                >
                    {
                        let current_text = RUSH_PHRASES[phrase_index.get() % RUSH_PHRASES.len()].to_string();
                        view! {
                            <TypingEngine
                                text=current_text
                                on_complete=on_complete
                                on_char_typed=on_char_typed
                                on_word_typed=on_word_typed
                                on_word_deleted=on_word_deleted
                            />
                        }
                    }
                </Show>
            </div>
        </div>
    }
}

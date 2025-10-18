use crate::components::typing::{MetricsBar, TypingEngine};
use leptos::prelude::*;
use rand::rngs::OsRng;
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::time::Duration;

// Struttura semplificata per deserializzare il JSON
#[derive(Deserialize)]
struct PhrasesData {
    phrases: Vec<String>,
}

// Carica le frasi dal file JSON
fn load_phrases() -> Vec<String> {
    let json_data = include_str!("../../../assets/datasets/base-dataset.json");
    let data: PhrasesData =
        serde_json::from_str(json_data).expect("Errore nel parsing del file base-dataset.json");
    data.phrases
}

// Mescola le frasi in ordine casuale
fn shuffle_phrases(phrases: &[String]) -> Vec<String> {
    let mut shuffled = phrases.to_vec();
    shuffled.shuffle(&mut OsRng);
    shuffled
}

// Costanti per configurare facilmente la modalità
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
    // Carica le frasi base all'avvio del componente
    let base_phrases = StoredValue::new(load_phrases());

    // Signal per le frasi mescolate che cambierà ad ogni restart
    let (shuffled_phrases, set_shuffled_phrases) =
        signal(shuffle_phrases(&base_phrases.get_value()));

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

    // Traccia le accuracy e wpm per calcolare le medie
    let (accuracy_sum, set_accuracy_sum) = signal(0.0);
    let (wpm_sum, set_wpm_sum) = signal(0.0);
    let (phrases_completed, set_phrases_completed) = signal(0_usize);

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

        // Aggiungi l'accuracy e il wpm alla somma e incrementa il contatore
        set_accuracy_sum.update(|sum| *sum += accuracy);
        set_wpm_sum.update(|sum| *sum += wpm);
        set_phrases_completed.update(|count| *count += 1);

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
        set_accuracy_sum.set(0.0);
        set_wpm_sum.set(0.0);
        set_phrases_completed.set(0);

        // Rimescola le frasi ad ogni restart
        set_shuffled_phrases.set(shuffle_phrases(&base_phrases.get_value()));
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
                <Show
                    when=move || game_state.get() == GameState::Finished
                    fallback=|| ()
                >
                    <div class="rush-game-over">
                        <h3 class="rush-game-over-title">"Tempo Scaduto!"</h3>
                        <div class="rush-final-stats">
                            <div class="rush-stat-item">
                                <span class="rush-stat-label">"Parole Totali"</span>
                                <span class="rush-stat-value">{total_words_typed.get()}</span>
                            </div>
                            <div class="rush-stat-item">
                                <span class="rush-stat-label">"Frasi Completate"</span>
                                <span class="rush-stat-value">{phrase_index.get()}</span>
                            </div>
                            <div class="rush-stat-item">
                                <span class="rush-stat-label">"WPM Medio"</span>
                                <span class="rush-stat-value">
                                    {move || {
                                        let avg = if phrases_completed.get() > 0 {
                                            wpm_sum.get() / phrases_completed.get() as f64
                                        } else {
                                            0.0
                                        };
                                        format!("{:.0}", avg)
                                    }}
                                </span>
                            </div>
                            <div class="rush-stat-item">
                                <span class="rush-stat-label">"Accuracy Media"</span>
                                <span class="rush-stat-value">
                                    {move || {
                                        let avg = if phrases_completed.get() > 0 {
                                            accuracy_sum.get() / phrases_completed.get() as f64
                                        } else {
                                            0.0
                                        };
                                        format!("{:.1}%", avg)
                                    }}
                                </span>
                            </div>
                        </div>
                        <button class="rush-play-again-button" on:click=restart_game>
                            "Gioca Ancora"
                        </button>
                    </div>
                </Show>

                <Show
                    when=move || game_state.get() != GameState::Finished
                    fallback=|| ()
                >
                    {
                        let phrases_list = shuffled_phrases.get();
                        let current_text = phrases_list[phrase_index.get() % phrases_list.len()].clone();
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

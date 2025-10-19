// src/components/typing/zen_mode.rs (AGGIORNATO con tracking stats)
//
use crate::components::typing::{ComboPopup, MetricsBar, TypingEngine, combo_popup::ComboType};
use crate::settings_store::use_settings;
use crate::stats_store::{GameMode as StatsGameMode, use_stats};
use leptos::prelude::*;
use rand::rngs::OsRng;
use rand::seq::SliceRandom;
use serde::Deserialize;
use web_sys::window;

#[derive(Deserialize)]
struct PhrasesData {
    phrases: Vec<String>,
}

fn load_phrases_from_content(json_content: &str) -> Vec<String> {
    let data: PhrasesData =
        serde_json::from_str(json_content).expect("Errore nel parsing del dataset JSON");
    data.phrases
}

fn shuffle_phrases(phrases: &[String]) -> Vec<String> {
    let mut shuffled = phrases.to_vec();
    shuffled.shuffle(&mut OsRng);
    shuffled
}

#[component]
pub fn ZenMode() -> impl IntoView {
    let settings_ctx = use_settings();
    let stats_ctx = use_stats();

    let base_phrases = Memo::new(move |_| {
        let difficulty = settings_ctx.get_difficulty();
        let json_content = difficulty.get_dataset_content();
        load_phrases_from_content(json_content)
    });

    let (shuffled_phrases, set_shuffled_phrases) = signal(Vec::<String>::new());
    let (phrase_index, set_phrase_index) = signal(0_usize);
    let (last_wpm, set_last_wpm) = signal(0.0);
    let (last_accuracy, set_last_accuracy) = signal(100.0);
    let (chars_typed, set_chars_typed) = signal(0_usize);
    let (words_typed, set_words_typed) = signal(0_usize);
    let (is_transitioning, set_is_transitioning) = signal(false);

    // Sistema combo aggiornato
    let (consecutive_correct_words, set_consecutive_correct_words) = signal(0_usize);
    let (combo_trigger, set_combo_trigger) = signal::<Option<ComboType>>(None);
    let (phrase_has_errors, set_phrase_has_errors) = signal(false);
    let (last_combo_milestone, set_last_combo_milestone) = signal(0_usize);
    let (highest_combo, set_highest_combo) = signal(0_usize);

    // Tracking sessione per stats
    let (session_started, set_session_started) = signal(false);
    let (session_start_time, set_session_start_time) = signal(0.0);
    let (total_session_words, set_total_session_words) = signal(0_u32);
    let (total_session_chars, set_total_session_chars) = signal(0_u32);
    let (accuracy_sum, set_accuracy_sum) = signal(0.0);
    let (wpm_sum, set_wpm_sum) = signal(0.0);
    let (phrases_completed, set_phrases_completed) = signal(0_u32);

    Effect::new(move |_| {
        let phrases = base_phrases.get();
        set_shuffled_phrases.set(shuffle_phrases(&phrases));
        set_phrase_index.set(0);
        set_chars_typed.set(0);
        set_words_typed.set(0);
        set_consecutive_correct_words.set(0);
        set_phrase_has_errors.set(false);
        set_last_combo_milestone.set(0);
        set_highest_combo.set(0);
        set_session_started.set(false);
        set_total_session_words.set(0);
        set_total_session_chars.set(0);
        set_accuracy_sum.set(0.0);
        set_wpm_sum.set(0.0);
        set_phrases_completed.set(0);
    });

    let current_phrase = Memo::new(move |_| {
        let phrases = shuffled_phrases.get();
        if phrases.is_empty() {
            String::new()
        } else {
            phrases[phrase_index.get() % phrases.len()].clone()
        }
    });

    // Callback per carattere errato
    let on_char_error = Callback::new(move |_: ()| {
        let current_combo = consecutive_correct_words.get();
        if current_combo >= 5 {
            set_combo_trigger.set(Some(ComboType::ComboBroken));
        }
        set_consecutive_correct_words.set(0);
        set_phrase_has_errors.set(true);
        set_last_combo_milestone.set(0);
    });

    let on_complete = Callback::new(move |(wpm, accuracy): (f64, f64)| {
        set_last_wpm.set(wpm);
        set_last_accuracy.set(accuracy);
        set_accuracy_sum.update(|sum| *sum += accuracy);
        set_wpm_sum.update(|sum| *sum += wpm);
        set_phrases_completed.update(|count| *count += 1);

        // Se la frase è stata completata senza errori
        if !phrase_has_errors.get() {
            set_combo_trigger.set(Some(ComboType::PerfectPhrase));
        }

        set_is_transitioning.set(true);

        set_timeout(
            move || {
                set_phrase_index.update(|i| {
                    *i += 1;
                    if *i >= shuffled_phrases.get().len() {
                        let phrases = base_phrases.get();
                        set_shuffled_phrases.set(shuffle_phrases(&phrases));
                        *i = 0;
                    }
                });
                set_chars_typed.set(0);
                set_words_typed.set(0);
                set_phrase_has_errors.set(false);
                set_is_transitioning.set(false);
            },
            std::time::Duration::from_millis(400),
        );
    });

    let on_char_typed = Callback::new(move |_: ()| {
        // Inizia la sessione al primo carattere
        if !session_started.get() {
            set_session_started.set(true);
            if let Some(win) = window() {
                if let Some(perf) = win.performance() {
                    set_session_start_time.set(perf.now());
                }
            }
        }

        set_chars_typed.update(|c| *c += 1);
        set_total_session_chars.update(|c| *c += 1);
    });

    let on_word_typed = Callback::new(move |_: ()| {
        set_words_typed.update(|w| *w += 1);
        set_total_session_words.update(|w| *w += 1);

        // Incrementa il contatore di parole consecutive corrette
        set_consecutive_correct_words.update(|count| {
            *count += 1;
            let current = *count;

            if current > highest_combo.get() {
                set_highest_combo.set(current);
            }

            let last_milestone = last_combo_milestone.get();

            // Logica milestone: trigger solo quando si supera un nuovo traguardo
            let should_trigger = match current {
                5 => last_milestone < 5,
                10 => last_milestone < 10,
                15 => last_milestone < 15,
                20 => last_milestone < 20,
                40 => last_milestone < 40,
                80 => last_milestone < 80,
                160 => last_milestone < 160,
                320 => last_milestone < 320,
                640 => last_milestone < 640,
                1000 => last_milestone < 1000,
                _ => false,
            };

            if should_trigger {
                set_last_combo_milestone.set(current);

                match current {
                    5 => set_combo_trigger.set(Some(ComboType::Streak5)),
                    10 => set_combo_trigger.set(Some(ComboType::Streak10)),
                    15 => set_combo_trigger.set(Some(ComboType::Streak15)),
                    20 => set_combo_trigger.set(Some(ComboType::Streak20)),
                    40 => set_combo_trigger.set(Some(ComboType::Streak40)),
                    80 => set_combo_trigger.set(Some(ComboType::Streak80)),
                    160 => set_combo_trigger.set(Some(ComboType::Streak160)),
                    320 => set_combo_trigger.set(Some(ComboType::Streak320)),
                    640 => set_combo_trigger.set(Some(ComboType::Streak640)),
                    1000 => set_combo_trigger.set(Some(ComboType::Streak1000)),
                    _ => {}
                }
            }
        });
    });

    let on_word_deleted = Callback::new(move |_: ()| {
        set_words_typed.update(|w| {
            if *w > 0 {
                *w -= 1;
            }
        });
        set_total_session_words.update(|w| {
            if *w > 0 {
                *w -= 1;
            }
        });
        // Cancellare conta come errore
        let current_combo = consecutive_correct_words.get();
        if current_combo >= 5 {
            set_combo_trigger.set(Some(ComboType::ComboBroken));
        }
        set_consecutive_correct_words.set(0);
        set_phrase_has_errors.set(true);
        set_last_combo_milestone.set(0);
    });

    // Funzione per terminare la sessione e salvare le stats
    let end_session = move |_| {
        if !session_started.get() || phrases_completed.get() == 0 {
            return;
        }

        let session_time = if let Some(win) = window() {
            if let Some(perf) = win.performance() {
                (perf.now() - session_start_time.get()) / 1000.0 // converti in secondi
            } else {
                0.0
            }
        } else {
            0.0
        };

        let avg_wpm = if phrases_completed.get() > 0 {
            wpm_sum.get() / phrases_completed.get() as f64
        } else {
            0.0
        };

        let avg_accuracy = if phrases_completed.get() > 0 {
            accuracy_sum.get() / phrases_completed.get() as f64
        } else {
            100.0
        };

        stats_ctx.record_game(
            StatsGameMode::Zen,
            total_session_words.get(),
            total_session_chars.get(),
            session_time,
            avg_wpm,
            avg_accuracy,
            highest_combo.get(),
            None,
        );

        // Reset sessione
        set_session_started.set(false);
        set_total_session_words.set(0);
        set_total_session_chars.set(0);
        set_accuracy_sum.set(0.0);
        set_wpm_sum.set(0.0);
        set_phrases_completed.set(0);
        set_highest_combo.set(0);
        set_consecutive_correct_words.set(0);
        set_last_combo_milestone.set(0);

        // Ricarica le frasi
        let phrases = base_phrases.get();
        set_shuffled_phrases.set(shuffle_phrases(&phrases));
        set_phrase_index.set(0);
    };

    view! {
        <div class="zen-mode">
            <ComboPopup trigger=Signal::derive(move || combo_trigger.get()) />

            <div class="zen-header">
                <span class="zen-icon">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-loader-pinwheel-icon lucide-loader-pinwheel"><path d="M22 12a1 1 0 0 1-10 0 1 1 0 0 0-10 0"/><path d="M7 20.7a1 1 0 1 1 5-8.7 1 1 0 1 0 5-8.6"/><path d="M7 3.3a1 1 0 1 1 5 8.6 1 1 0 1 0 5 8.6"/><circle cx="12" cy="12" r="10"/></svg>
                </span>
                <h2 class="zen-title">"Zen"</h2>
                <p class="zen-description">
                    "Trova il tuo flow. Una frase dopo l'altra, al tuo passo. L'unica sfida è la tua concentrazione."
                </p>
            </div>

            <MetricsBar
                wpm=Signal::derive(move || last_wpm.get())
                accuracy=Signal::derive(move || last_accuracy.get())
                chars_typed=Signal::derive(move || chars_typed.get())
                words_typed=Signal::derive(move || words_typed.get())
                current_phrase=Signal::derive(move || phrase_index.get() + 1)
            />

            {move || {
                if is_transitioning.get() {
                    view! {
                        <div class="zen-transition zen-transition--fading">
                            <div class="typing-display" style="min-height: 200px;">
                            </div>
                        </div>
                    }.into_any()
                } else {
                    let phrase = current_phrase.get();
                    if phrase.is_empty() {
                        view! {
                            <div class="typing-display" style="min-height: 200px;">
                                <p>"Caricamento frasi..."</p>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="zen-transition">
                                <TypingEngine
                                    text=phrase
                                    on_complete=on_complete
                                    on_char_typed=on_char_typed
                                    on_char_error=on_char_error
                                    on_word_typed=on_word_typed
                                    on_word_deleted=on_word_deleted
                                />
                            </div>
                        }.into_any()
                    }
                }
            }}


            // Pulsante per terminare la sessione
            <Show when=move || session_started.get()>
                <div class="zen-session-controls">
                    <button class="zen-end-session-button" on:click=end_session>
                        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M9 9h6v6H9z"/>
                            <path d="M3 12a9 9 0 1 0 18 0 9 9 0 1 0-18 0"/>
                        </svg>
                        "Termina Sessione e Salva"
                    </button>
                    <div class="zen-session-info">
                        <span class="zen-session-stat">
                            "Frasi completate: "
                            <strong>{move || phrases_completed.get()}</strong>
                        </span>
                        <span class="zen-session-stat">
                            "Parole totali: "
                            <strong>{move || total_session_words.get()}</strong>
                        </span>
                    </div>
                </div>
            </Show>
        </div>
    }
}

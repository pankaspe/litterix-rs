// src/components/typing/marathon_mode.rs
//
use crate::components::typing::{ComboPopup, ComboType, MetricsBar, TypingEngine};
use crate::settings_store::use_settings;
use leptos::prelude::*;
use rand::rngs::OsRng;
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::time::Duration;

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

const MARATHON_TIME: f64 = 120.0; // 2 minuti

#[derive(Clone, PartialEq, Debug)]
enum GameState {
    Pending,
    Running,
    Finished,
}

// Helper per ottenere il badge combo
fn get_combo_badge(combo: usize) -> (&'static str, &'static str) {
    match combo {
        0..=4 => ("", "Nessun Combo"),
        5..=9 => ("🔥", "Combo"),
        10..=14 => ("⚡", "Combo"),
        15..=19 => ("💫", "Combo"),
        20..=39 => ("🌟", "Grande Combo"),
        40..=79 => ("💥", "Mega Combo"),
        80..=159 => ("🚀", "Mega Combo"),
        160..=319 => ("⭐", "Ultra Combo"),
        320..=639 => ("👑", "Legendary"),
        640..=999 => ("🔱", "Godlike"),
        _ => ("🏆", "Unstoppable"),
    }
}

#[component]
pub fn MarathonMode() -> impl IntoView {
    let settings_ctx = use_settings();

    let base_phrases = Memo::new(move |_| {
        let difficulty = settings_ctx.get_difficulty();
        let json_content = difficulty.get_dataset_content();
        load_phrases_from_content(json_content)
    });

    let (shuffled_phrases, set_shuffled_phrases) = signal(Vec::<String>::new());
    let (game_state, set_game_state) = signal(GameState::Pending);
    let (phrase_index, set_phrase_index) = signal(0_usize);
    let (time_remaining, set_time_remaining) = signal(MARATHON_TIME);
    let (total_words_typed, set_total_words_typed) = signal(0_usize);
    let (total_chars_typed, set_total_chars_typed) = signal(0_usize);
    let (last_wpm, set_last_wpm) = signal(0.0);
    let (last_accuracy, set_last_accuracy) = signal(100.0);
    let (accuracy_sum, set_accuracy_sum) = signal(0.0);
    let (wpm_sum, set_wpm_sum) = signal(0.0);
    let (phrases_completed, set_phrases_completed) = signal(0_usize);

    // Sistema combo
    let (consecutive_correct_words, set_consecutive_correct_words) = signal(0_usize);
    let (combo_trigger, set_combo_trigger) = signal::<Option<ComboType>>(None);
    let (phrase_has_errors, set_phrase_has_errors) = signal(false);
    let (last_combo_milestone, set_last_combo_milestone) = signal(0_usize);
    let (highest_combo, set_highest_combo) = signal(0_usize);

    Effect::new(move |_| {
        let phrases = base_phrases.get();
        set_shuffled_phrases.set(shuffle_phrases(&phrases));
    });

    // Timer countdown
    Effect::new(move |_| {
        if game_state.get() == GameState::Running {
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

            on_cleanup(move || handle.clear());
        }
    });

    let on_char_typed = Callback::new(move |_: ()| {
        if game_state.get() == GameState::Pending {
            set_game_state.set(GameState::Running);
        }
        set_total_chars_typed.update(|c| *c += 1);
    });

    let on_char_error = Callback::new(move |_: ()| {
        let current_combo = consecutive_correct_words.get();
        if current_combo >= 5 {
            set_combo_trigger.set(Some(ComboType::ComboBroken));
        }
        set_consecutive_correct_words.set(0);
        set_phrase_has_errors.set(true);
        set_last_combo_milestone.set(0);
    });

    let on_word_typed = Callback::new(move |_: ()| {
        set_total_words_typed.update(|w| *w += 1);

        set_consecutive_correct_words.update(|count| {
            *count += 1;
            let current = *count;

            if current > highest_combo.get() {
                set_highest_combo.set(current);
            }

            let last_milestone = last_combo_milestone.get();

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
        set_total_words_typed.update(|w| {
            if *w > 0 {
                *w -= 1;
            }
        });
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

        if !phrase_has_errors.get() {
            set_combo_trigger.set(Some(ComboType::PerfectPhrase));
        }

        set_phrase_index.update(|i| *i += 1);
        set_phrase_has_errors.set(false);
    });

    let restart_game = move |_| {
        set_game_state.set(GameState::Pending);
        set_time_remaining.set(MARATHON_TIME);
        set_phrase_index.set(0);
        set_total_words_typed.set(0);
        set_total_chars_typed.set(0);
        set_last_wpm.set(0.0);
        set_last_accuracy.set(100.0);
        set_accuracy_sum.set(0.0);
        set_wpm_sum.set(0.0);
        set_phrases_completed.set(0);
        set_consecutive_correct_words.set(0);
        set_phrase_has_errors.set(false);
        set_last_combo_milestone.set(0);
        set_highest_combo.set(0);

        let phrases = base_phrases.get();
        set_shuffled_phrases.set(shuffle_phrases(&phrases));
    };

    view! {
        <div class="marathon-mode">
            <ComboPopup trigger=Signal::derive(move || combo_trigger.get()) />

            <div class="marathon-header">
                <span class="marathon-icon">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 2v4"/><path d="m16 6-4 4-4-4"/><rect width="20" height="8" x="2" y="14" rx="2"/><path d="M6 18h.01"/><path d="M10 18h.01"/></svg>
                </span>
                <h2 class="marathon-title">"Marathon"</h2>
                <p class="marathon-description">
                    "2 minuti per massimizzare parole e combo. Ogni frase perfetta conta!"
                </p>
            </div>

            <MetricsBar
                wpm=Signal::derive(move || last_wpm.get())
                accuracy=Signal::derive(move || last_accuracy.get())
                chars_typed=Signal::derive(move || total_chars_typed.get())
                words_typed=Signal::derive(move || total_words_typed.get())
                timer=Signal::derive(move || time_remaining.get())
            />

            <div class="marathon-typing-area">
                <Show
                    when=move || game_state.get() == GameState::Finished
                    fallback=|| ()
                >
                    <div class="marathon-game-over">
                        <h3 class="marathon-game-over-title">"Marathon Completata!"</h3>
                        <div class="marathon-final-stats">
                            <div class="marathon-stat-item marathon-stat-item--highlight">
                                <span class="marathon-stat-label">"Punteggio Totale"</span>
                                <span class="marathon-stat-value marathon-stat-value--score">
                                    {move || {
                                        let words = total_words_typed.get();
                                        let combo_bonus = highest_combo.get() / 5;
                                        let score = words + combo_bonus;
                                        format!("{}", score)
                                    }}
                                </span>
                            </div>
                            <div class="marathon-stat-item">
                                <span class="marathon-stat-label">"Parole Totali"</span>
                                <span class="marathon-stat-value">{total_words_typed.get()}</span>
                            </div>
                            <div class="marathon-stat-item">
                                <span class="marathon-stat-label">"Frasi Completate"</span>
                                <span class="marathon-stat-value">{phrases_completed.get()}</span>
                            </div>
                            <div class="marathon-stat-item marathon-stat-item--combo">
                                <span class="marathon-stat-label">"Combo Massima"</span>
                                <span class="marathon-stat-value marathon-stat-value--combo">
                                    {move || {
                                        let combo = highest_combo.get();
                                        let (emoji, _label) = get_combo_badge(combo);
                                        format!("{} {}", emoji, combo)
                                    }}
                                </span>
                            </div>
                            <div class="marathon-stat-item">
                                <span class="marathon-stat-label">"WPM Medio"</span>
                                <span class="marathon-stat-value">
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
                            <div class="marathon-stat-item">
                                <span class="marathon-stat-label">"Accuracy Media"</span>
                                <span class="marathon-stat-value">
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
                        <button class="marathon-play-again-button" on:click=restart_game>
                            "Gioca Ancora"
                        </button>
                    </div>
                </Show>

                <Show
                    when=move || game_state.get() != GameState::Finished
                    fallback=|| ()
                >
                    {move || {
                        let phrases_list = shuffled_phrases.get();
                        if phrases_list.is_empty() {
                            view! {
                                <div class="typing-display" style="min-height: 200px;">
                                    <p>"Caricamento frasi..."</p>
                                </div>
                            }.into_any()
                        } else {
                            let current_text = phrases_list[phrase_index.get() % phrases_list.len()].clone();
                            view! {
                                <TypingEngine
                                    text=current_text
                                    on_complete=on_complete
                                    on_char_typed=on_char_typed
                                    on_char_error=on_char_error
                                    on_word_typed=on_word_typed
                                    on_word_deleted=on_word_deleted
                                />
                            }.into_any()
                        }
                    }}
                </Show>
            </div>
        </div>
    }
}

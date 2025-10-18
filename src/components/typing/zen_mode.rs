// src/components/typing/zen_mode.rs
//
use crate::components::typing::{MetricsBar, TypingEngine};
use leptos::prelude::*;
use rand::rngs::OsRng;
use rand::seq::SliceRandom;
use serde::Deserialize;

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

#[component]
pub fn ZenMode() -> impl IntoView {
    // Carica le frasi base all'avvio del componente
    let base_phrases = StoredValue::new(load_phrases());

    // Signal per le frasi mescolate
    let (shuffled_phrases, set_shuffled_phrases) =
        signal(shuffle_phrases(&base_phrases.get_value()));

    let (phrase_index, set_phrase_index) = signal(0_usize);
    let (last_wpm, set_last_wpm) = signal(0.0);
    let (last_accuracy, set_last_accuracy) = signal(100.0);
    let (chars_typed, set_chars_typed) = signal(0_usize);
    let (words_typed, set_words_typed) = signal(0_usize);
    let (is_transitioning, set_is_transitioning) = signal(false);

    // Genera la frase corrente in anticipo
    let current_phrase = Memo::new(move |_| {
        let phrases = shuffled_phrases.get();
        phrases[phrase_index.get() % phrases.len()].clone()
    });

    let on_complete = Callback::new(move |(wpm, accuracy): (f64, f64)| {
        set_last_wpm.set(wpm);
        set_last_accuracy.set(accuracy);
        set_is_transitioning.set(true);

        // Transizione più fluida
        set_timeout(
            move || {
                set_phrase_index.update(|i| {
                    *i += 1;
                    // Quando finiscono tutte le frasi, rimescola
                    if *i >= shuffled_phrases.get().len() {
                        set_shuffled_phrases.set(shuffle_phrases(&base_phrases.get_value()));
                        *i = 0;
                    }
                });
                set_chars_typed.set(0); // Reset contatore caratteri
                set_words_typed.set(0); // Reset contatore parole
                set_is_transitioning.set(false);
            },
            std::time::Duration::from_millis(400),
        );
    });

    let on_char_typed = Callback::new(move |_: ()| {
        set_chars_typed.update(|c| *c += 1);
    });

    let on_word_typed = Callback::new(move |_: ()| {
        set_words_typed.update(|w| *w += 1);
    });

    let on_word_deleted = Callback::new(move |_: ()| {
        set_words_typed.update(|w| {
            // Assicurati di non scendere sotto lo zero
            if *w > 0 {
                *w -= 1;
            }
        });
    });

    view! {
        <div class="zen-mode">

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
                    view! {
                        <div class="zen-transition">
                            <TypingEngine
                                text=current_phrase.get()
                                on_complete=on_complete
                                on_char_typed=on_char_typed
                                on_word_typed=on_word_typed
                                on_word_deleted=on_word_deleted
                            />
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}

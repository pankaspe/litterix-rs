// src/components/typing/zen_mode.rs
//
use crate::components::typing::{MetricsBar, TypingEngine};
use leptos::prelude::*;

const ZEN_PHRASES: &[&str] = &[
    "la velocità è nulla senza il controllo",
    "ogni errore è un maestro che insegna pazienza",
    "il ritmo della tastiera diventa musica per le dita",
    "rust e webassembly creano magia nel browser",
    "la precisione batte sempre la fretta",
    "digitare senza guardare è come volare senza ali",
    "il codice pulito nasce da dita sicure",
    "la tastiera è uno strumento musicale per programmatori",
];

#[component]
pub fn ZenMode() -> impl IntoView {
    let (phrase_index, set_phrase_index) = signal(0_usize);
    let (last_wpm, set_last_wpm) = signal(0.0);
    let (last_accuracy, set_last_accuracy) = signal(100.0);
    let (chars_typed, set_chars_typed) = signal(0_usize);
    let (words_typed, set_words_typed) = signal(0_usize);
    let (is_transitioning, set_is_transitioning) = signal(false);

    // Genera la frase corrente in anticipo
    let current_phrase = Memo::new(move |_| ZEN_PHRASES[phrase_index.get()].to_string());

    let on_complete = Callback::new(move |(wpm, accuracy): (f64, f64)| {
        set_last_wpm.set(wpm);
        set_last_accuracy.set(accuracy);
        set_is_transitioning.set(true);

        // Transizione più fluida
        set_timeout(
            move || {
                set_phrase_index.update(|i| {
                    *i = (*i + 1) % ZEN_PHRASES.len();
                });
                set_chars_typed.set(0); // Reset contatore caratteri
                set_words_typed.set(0); // Reset contatore parole
                set_is_transitioning.set(false);
            },
            std::time::Duration::from_millis(800),
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
                <h2 class="zen-title">"Zen Mode"</h2>
                <span class="zen-badge">
                    {move || format!("Frase {}/{}", phrase_index.get() + 1, ZEN_PHRASES.len())}
                </span>
            </div>

            <MetricsBar
                wpm=Signal::derive(move || last_wpm.get())
                accuracy=Signal::derive(move || last_accuracy.get())
                chars_typed=Signal::derive(move || chars_typed.get())
                words_typed=Signal::derive(move || words_typed.get())
                current_phrase=Signal::derive(move || phrase_index.get() + 1)
                total_phrases=ZEN_PHRASES.len()
            />

            {move || {
                // Non mostrare nulla durante la transizione
                if is_transitioning.get() {
                    view! {
                        <div class="typing-display" style="min-height: 200px; opacity: 0.3;">
                            <div class="typing-text">"Caricamento prossima frase..."</div>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <TypingEngine
                            text=current_phrase.get()
                            on_complete=on_complete
                            on_char_typed=on_char_typed
                            on_word_typed=on_word_typed
                            on_word_deleted=on_word_deleted
                        />
                    }.into_any()
                }
            }}
        </div>
    }
}

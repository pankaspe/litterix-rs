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

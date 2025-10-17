use crate::components::typing::TypingEngine;
use leptos::prelude::*;

// Frasi hardcoded per testing
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
    let (key, set_key) = signal(0); // Chiave per forzare re-render

    let current_phrase = move || ZEN_PHRASES[phrase_index.get()].to_string();

    let on_complete = Callback::new(move |(wpm, accuracy): (f64, f64)| {
        leptos::logging::log!("Completato! WPM: {:.0}, Accuracy: {:.1}%", wpm, accuracy);

        // Passa alla frase successiva dopo un breve delay
        set_timeout(
            move || {
                set_phrase_index.update(|i| {
                    *i = (*i + 1) % ZEN_PHRASES.len();
                });
                set_key.update(|k| *k += 1);
            },
            std::time::Duration::from_millis(1500),
        );
    });

    view! {
        <style>
            {r#"
.zen-mode {
    display: flex;
    flex-direction: column;
    gap: 2rem;
    max-width: 1000px;
    margin: 0 auto;
    padding: 2rem 0;
}

.zen-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-bottom: 1rem;
    border-bottom: 1px solid var(--color-surface);
}

.zen-title {
    font-size: 1.5rem;
    font-family: var(--font-family-mono);
    color: var(--color-text);
    font-weight: 500;
}

.zen-badge {
    background: rgba(247, 76, 0, 0.1);
    color: var(--color-primary);
    padding: 0.5rem 1rem;
    border-radius: 8px;
    font-family: var(--font-family-mono);
    font-size: 0.85rem;
}
            "#}
        </style>

        <div class="zen-mode">
            <div class="zen-header">
                <h2 class="zen-title">"Zen mode"</h2>
                <span class="zen-badge">
                    {move || format!("Frase {}", phrase_index.get() + 1)}
                </span>
            </div>

            {move || {
                let _ = key.get(); // Trigger re-render
                view! {
                    <TypingEngine
                        text=current_phrase()
                        on_complete=on_complete
                    />
                }
            }}
        </div>
    }
}

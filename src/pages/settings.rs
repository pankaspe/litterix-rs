// src/pages/settings.rs
//
use crate::settings_store::{DatasetDifficulty, use_settings};
use leptos::prelude::*;

#[component]
pub fn Settings() -> impl IntoView {
    let settings_ctx = use_settings();

    // State locale per la selezione corrente (non salvata finché non si preme Salva)
    let (selected_difficulty, set_selected_difficulty) = signal(settings_ctx.get_difficulty());

    // State per il messaggio di feedback
    let (feedback_message, set_feedback_message) = signal(String::new());
    let (show_feedback, set_show_feedback) = signal(false);

    let handle_difficulty_change = move |difficulty: DatasetDifficulty| {
        set_selected_difficulty.set(difficulty);
    };

    let handle_save = move |_| {
        // Aggiorna il context con la selezione corrente
        settings_ctx.set_difficulty(selected_difficulty.get());

        // Salva in localStorage
        match settings_ctx.save() {
            Ok(_) => {
                set_feedback_message.set("✓ Impostazioni salvate con successo!".to_string());
                set_show_feedback.set(true);

                // Nascondi il messaggio dopo 3 secondi
                set_timeout(
                    move || set_show_feedback.set(false),
                    std::time::Duration::from_secs(3),
                );
            }
            Err(_) => {
                set_feedback_message
                    .set("✗ Errore nel salvataggio delle impostazioni.".to_string());
                set_show_feedback.set(true);
            }
        }
    };

    let handle_reset = move |_| {
        // Reset alle impostazioni di default
        match settings_ctx.reset() {
            Ok(_) => {
                set_selected_difficulty.set(DatasetDifficulty::Base);
                set_feedback_message
                    .set("✓ Impostazioni ripristinate ai valori predefiniti.".to_string());
                set_show_feedback.set(true);

                set_timeout(
                    move || set_show_feedback.set(false),
                    std::time::Duration::from_secs(3),
                );
            }
            Err(_) => {
                set_feedback_message.set("✗ Errore nel ripristino delle impostazioni.".to_string());
                set_show_feedback.set(true);
            }
        }
    };

    view! {
        <main class="container settings">
            <h1 class="settings__title">
                "Impostazioni "
                <span class="settings__highlight">"personalizza l'esperienza"</span>
            </h1>

            <p class="settings__description">
                "Configura Litterix secondo le tue preferenze. Le impostazioni vengono salvate localmente nel tuo browser."
            </p>

            // Messaggio Privacy
            <div class="settings__privacy-notice">
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="settings__privacy-icon">
                    <circle cx="12" cy="12" r="10"/>
                    <path d="M12 16v-4"/>
                    <path d="M12 8h.01"/>
                </svg>
                <p class="settings__privacy-text">
                    "Le tue preferenze sono salvate solo nel tuo browser (localStorage) e non vengono inviate a server esterni. "
                    "Puoi eliminarle in qualsiasi momento premendo il pulsante Reset."
                </p>
            </div>

            // Messaggio di Feedback
            <Show when=move || show_feedback.get()>
                <div class="settings__feedback">
                    {move || feedback_message.get()}
                </div>
            </Show>

            <div class="settings__section">
                <h2 class="settings__section-title">"Difficoltà Dataset"</h2>
                <p style="color: var(--color-text-muted); margin-bottom: 1rem;">
                    "Scegli il livello di complessità delle frasi che desideri esercitare."
                </p>

                <div class="settings__options">
                    // Opzione Base
                    <label
                        class=move || {
                            if selected_difficulty.get() == DatasetDifficulty::Base {
                                "settings__option settings__option--active"
                            } else {
                                "settings__option"
                            }
                        }
                    >
                        <input
                            type="radio"
                            name="difficulty"
                            class="settings__option-radio"
                            checked=move || selected_difficulty.get() == DatasetDifficulty::Base
                            on:change=move |_| handle_difficulty_change(DatasetDifficulty::Base)
                        />
                        <div class="settings__option-content">
                            <div class="settings__option-title">"Base"</div>
                            <div class="settings__option-description">
                                "Frasi semplici e dirette, ideali per iniziare a prendere confidenza con la tastiera."
                            </div>
                        </div>
                    </label>

                    // Opzione Intermediate
                    <label
                        class=move || {
                            if selected_difficulty.get() == DatasetDifficulty::Intermediate {
                                "settings__option settings__option--active"
                            } else {
                                "settings__option"
                            }
                        }
                    >
                        <input
                            type="radio"
                            name="difficulty"
                            class="settings__option-radio"
                            checked=move || selected_difficulty.get() == DatasetDifficulty::Intermediate
                            on:change=move |_| handle_difficulty_change(DatasetDifficulty::Intermediate)
                        />
                        <div class="settings__option-content">
                            <div class="settings__option-title">"Intermedio"</div>
                            <div class="settings__option-description">
                                "Frasi di media complessità con vocabolario più ricco, per migliorare fluidità e velocità."
                            </div>
                        </div>
                    </label>

                    // Opzione Advanced
                    <label
                        class=move || {
                            if selected_difficulty.get() == DatasetDifficulty::Advanced {
                                "settings__option settings__option--active"
                            } else {
                                "settings__option"
                            }
                        }
                    >
                        <input
                            type="radio"
                            name="difficulty"
                            class="settings__option-radio"
                            checked=move || selected_difficulty.get() == DatasetDifficulty::Advanced
                            on:change=move |_| handle_difficulty_change(DatasetDifficulty::Advanced)
                        />
                        <div class="settings__option-content">
                            <div class="settings__option-title">"Avanzato"</div>
                            <div class="settings__option-description">
                                "Frasi lunghe e articolate per affinare la precisione e testare la vera padronanza."
                            </div>
                        </div>
                    </label>
                </div>
            </div>

            // Pulsanti di azione
            <div class="settings__actions">
                <button class="settings__button settings__button--primary" on:click=handle_save>
                    "Salva Impostazioni"
                </button>
                <button class="settings__button settings__button--secondary" on:click=handle_reset>
                    "Reset"
                </button>
            </div>
        </main>
    }
}

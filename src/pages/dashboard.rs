// src/pages/dashboard.rs
//
use crate::stats_store::use_stats;
use leptos::prelude::*;

#[component]
pub fn Dashboard() -> impl IntoView {
    let stats_ctx = use_stats();
    let stats = Signal::derive(move || stats_ctx.get_stats());

    let (show_confirm_dialog, set_show_confirm_dialog) = signal(false);
    let (feedback_message, set_feedback_message) = signal(String::new());
    let (show_feedback, set_show_feedback) = signal(false);

    let handle_clear = move |_| {
        set_show_confirm_dialog.set(true);
    };

    let confirm_clear = move |_| match stats_ctx.clear() {
        Ok(_) => {
            set_feedback_message.set("✓ Tutti i dati sono stati eliminati.".to_string());
            set_show_feedback.set(true);
            set_show_confirm_dialog.set(false);

            set_timeout(
                move || set_show_feedback.set(false),
                std::time::Duration::from_secs(3),
            );
        }
        Err(_) => {
            set_feedback_message.set("✗ Errore durante l'eliminazione.".to_string());
            set_show_feedback.set(true);
        }
    };

    let cancel_clear = move |_| {
        set_show_confirm_dialog.set(false);
    };

    view! {
        <main class="container dashboard">
            <h1 class="dashboard__title">
                "Dashboard "
                <span class="dashboard__highlight">"il tuo progresso"</span>
            </h1>

            <Show when=move || show_feedback.get()>
                <div class="dashboard__feedback">
                    {move || feedback_message.get()}
                </div>
            </Show>

            <Show
                when=move || stats.get().has_played()
                fallback=|| view! {
                    <div class="dashboard__empty">
                        <div class="dashboard__empty-icon">
                            <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M3 3v18h18"/>
                                <path d="m19 9-5 5-4-4-3 3"/>
                            </svg>
                        </div>
                        <h2 class="dashboard__empty-title">"Nessun Dato Disponibile"</h2>
                        <p class="dashboard__empty-text">
                            "Inizia a giocare per vedere le tue statistiche e il tuo progresso nel tempo!"
                        </p>
                    </div>
                }
            >
                <div class="dashboard__content">
                    // Sezione Record Personali
                    <section class="dashboard__section">
                        <h2 class="dashboard__section-title">
                            <span class="dashboard__section-icon">"🏆"</span>
                            "Record Personali"
                        </h2>
                        <div class="dashboard__cards">
                            <div class="dashboard__card dashboard__card--primary">
                                <div class="dashboard__card-label">"WPM Massimo"</div>
                                <div class="dashboard__card-value">{move || format!("{:.0}", stats.get().best_wpm)}</div>
                            </div>
                            <div class="dashboard__card dashboard__card--success">
                                <div class="dashboard__card-label">"Accuracy Massima"</div>
                                <div class="dashboard__card-value">{move || format!("{:.1}%", stats.get().best_accuracy)}</div>
                            </div>
                            <div class="dashboard__card dashboard__card--warning">
                                <div class="dashboard__card-label">"Combo Massima"</div>
                                <div class="dashboard__card-value">{move || stats.get().highest_combo.to_string()}</div>
                            </div>
                            <div class="dashboard__card dashboard__card--info">
                                <div class="dashboard__card-label">"Punteggio Marathon"</div>
                                <div class="dashboard__card-value">{move || stats.get().marathon_best_score.to_string()}</div>
                            </div>
                        </div>
                    </section>

                    // Sezione Statistiche Generali
                    <section class="dashboard__section">
                        <h2 class="dashboard__section-title">
                            <span class="dashboard__section-icon">"📊"</span>
                            "Statistiche Generali"
                        </h2>
                        <div class="dashboard__stats-grid">
                            <div class="dashboard__stat">
                                <div class="dashboard__stat-label">"Partite Giocate"</div>
                                <div class="dashboard__stat-value">{move || stats.get().total_games_played.to_string()}</div>
                            </div>
                            <div class="dashboard__stat">
                                <div class="dashboard__stat-label">"Parole Totali"</div>
                                <div class="dashboard__stat-value">{move || stats.get().total_words_typed.to_string()}</div>
                            </div>
                            <div class="dashboard__stat">
                                <div class="dashboard__stat-label">"Caratteri Totali"</div>
                                <div class="dashboard__stat-value">{move || stats.get().total_chars_typed.to_string()}</div>
                            </div>
                            <div class="dashboard__stat">
                                <div class="dashboard__stat-label">"Tempo Totale"</div>
                                <div class="dashboard__stat-value">
                                    {move || {
                                        let minutes = (stats.get().total_time_played / 60.0).floor() as u32;
                                        format!("{}m", minutes)
                                    }}
                                </div>
                            </div>
                            <div class="dashboard__stat">
                                <div class="dashboard__stat-label">"WPM Medio"</div>
                                <div class="dashboard__stat-value">{move || format!("{:.0}", stats.get().average_wpm)}</div>
                            </div>
                            <div class="dashboard__stat">
                                <div class="dashboard__stat-label">"Accuracy Media"</div>
                                <div class="dashboard__stat-value">{move || format!("{:.1}%", stats.get().average_accuracy)}</div>
                            </div>
                        </div>
                    </section>

                    // Sezione Per Modalità
                    <section class="dashboard__section">
                        <h2 class="dashboard__section-title">
                            <span class="dashboard__section-icon">"🎮"</span>
                            "Per Modalità"
                        </h2>
                        <div class="dashboard__modes">
                            <div class="dashboard__mode">
                                <div class="dashboard__mode-icon">"🧘"</div>
                                <div class="dashboard__mode-name">"Zen Mode"</div>
                                <div class="dashboard__mode-count">{move || format!("{} partite", stats.get().zen_games)}</div>
                            </div>
                            <div class="dashboard__mode">
                                <div class="dashboard__mode-icon">"⚡"</div>
                                <div class="dashboard__mode-name">"Rush Mode"</div>
                                <div class="dashboard__mode-count">{move || format!("{} partite", stats.get().rush_games)}</div>
                            </div>
                            <div class="dashboard__mode">
                                <div class="dashboard__mode-icon">"🏃"</div>
                                <div class="dashboard__mode-name">"Marathon"</div>
                                <div class="dashboard__mode-count">{move || format!("{} partite", stats.get().marathon_games)}</div>
                            </div>
                        </div>
                    </section>

                    // Pulsante Elimina Dati
                    <div class="dashboard__actions">
                        <button class="dashboard__clear-button" on:click=handle_clear>
                            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/>
                            </svg>
                            "Elimina Tutti i Dati"
                        </button>
                    </div>
                </div>
            </Show>

            // Dialog di conferma
            <Show when=move || show_confirm_dialog.get()>
                <div class="dashboard__overlay" on:click=cancel_clear>
                    <div class="dashboard__dialog" on:click=|e| e.stop_propagation()>
                        <h3 class="dashboard__dialog-title">"Conferma Eliminazione"</h3>
                        <p class="dashboard__dialog-text">
                            "Sei sicuro di voler eliminare tutti i tuoi dati? Questa azione non può essere annullata."
                        </p>
                        <div class="dashboard__dialog-actions">
                            <button class="dashboard__dialog-button dashboard__dialog-button--cancel" on:click=cancel_clear>
                                "Annulla"
                            </button>
                            <button class="dashboard__dialog-button dashboard__dialog-button--confirm" on:click=confirm_clear>
                                "Elimina"
                            </button>
                        </div>
                    </div>
                </div>
            </Show>
        </main>
    }
}

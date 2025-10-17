use leptos::prelude::*;

#[component]
pub fn MetricsBar(
    wpm: Signal<f64>,
    accuracy: Signal<f64>,
    chars_typed: Signal<usize>,
    words_typed: Signal<usize>,
    #[prop(optional)] current_phrase: Option<Signal<usize>>,
    #[prop(optional)] total_phrases: Option<usize>,
    #[prop(optional)] timer: Option<Signal<f64>>,
) -> impl IntoView {
    view! {
        <div class="metrics-bar">
            // --- Gruppo Sinistro: Metriche di Performance ---
            <div class="metrics-bar__group">
                {move || timer.map(|t| view! {
                    <div class="metrics-bar__stat metrics-bar__stat--timer">
                        <div class="metrics-bar__label">"tempo"</div>
                        <div class="metrics-bar__value">{move || format!("{:.1}", t.get())}</div>
                    </div>
                })}
                <div class="metrics-bar__stat metrics-bar__stat--wpm">
                    <div class="metrics-bar__label">"wpm"</div>
                    <div class="metrics-bar__value">
                        {move || if wpm.get() > 0.0 { format!("{:.0}", wpm.get()) } else { "-".to_string() }}
                    </div>
                </div>
                <div class="metrics-bar__stat">
                    <div class="metrics-bar__label">"acc"</div>
                    <div class="metrics-bar__value">{move || format!("{:.1}%", accuracy.get())}</div>
                </div>
            </div>

            // --- Gruppo Destro: Metriche di Progresso ---
            <div class="metrics-bar__group">
                <div class="metrics-bar__stat">
                    <div class="metrics-bar__label">"caratteri"</div>
                    <div class="metrics-bar__value">{move || chars_typed.get().to_string()}</div>
                </div>
                <div class="metrics-bar__stat">
                    <div class="metrics-bar__label">"parole"</div>
                    <div class="metrics-bar__value">{move || words_typed.get().to_string()}</div>
                </div>
                {move || {
                    // Logica per mostrare la frase:
                    // 1. Deve esistere un 'current_phrase'
                    // 2. Se esiste anche 'total_phrases', mostra "X/Y"
                    // 3. Altrimenti, mostra solo "X"
                    current_phrase.map(|current| view! {
                        <div class="metrics-bar__stat">
                            <div class="metrics-bar__label">"frase"</div>
                            <div class="metrics-bar__value">
                                {move || if let Some(total) = total_phrases {
                                    format!("{}/{}", current.get(), total)
                                } else {
                                    current.get().to_string()
                                }}
                            </div>
                        </div>
                    })
                }}
            </div>
        </div>
    }
}

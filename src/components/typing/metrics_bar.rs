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
            // Metrica Timer (mostrata solo se presente)
            {move || {
                timer.map(|t| {
                    view! {
                        <div class="metrics-bar__stat metrics-bar__stat--timer">
                            <div class="metrics-bar__label">"Tempo"</div>
                            <div class="metrics-bar__value">
                                {move || format!("{:.1}s", t.get())}
                            </div>
                        </div>
                    }
                })
            }}

            <div class="metrics-bar__stat">
                <div class="metrics-bar__label">"WPM"</div>
                <div class="metrics-bar__value">
                    {move || {
                        let w = wpm.get();
                        if w > 0.0 {
                            format!("{:.0}", w)
                        } else {
                            "-".to_string()
                        }
                    }}
                </div>
            </div>

            <div class="metrics-bar__stat">
                <div class="metrics-bar__label">"Accuracy"</div>
                <div class="metrics-bar__value">
                    {move || format!("{:.1}%", accuracy.get())}
                </div>
            </div>

            // --- REINSERITO IL BLOCCO MANCANTE ---
            <div class="metrics-bar__stat">
                <div class="metrics-bar__label">"Caratteri"</div>
                <div class="metrics-bar__value">
                    {move || chars_typed.get().to_string()}
                </div>
            </div>
            // --- FINE BLOCCO REINSERITO ---

            <div class="metrics-bar__stat">
                <div class="metrics-bar__label">"Parole"</div>
                <div class="metrics-bar__value">
                    {move || words_typed.get().to_string()}
                </div>
            </div>

            // Metrica Frase (mostrata solo se presente)
            {move || {
                if let (Some(current), Some(total)) = (current_phrase, total_phrases) {
                    Some(view! {
                        <div class="metrics-bar__stat">
                            <div class="metrics-bar__label">"Frase"</div>
                            <div class="metrics-bar__value">
                                {move || format!("{}/{}", current.get(), total)}
                            </div>
                        </div>
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}

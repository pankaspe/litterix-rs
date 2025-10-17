// src/components/typing/metrics_bar.rs
//
use leptos::prelude::*;

#[component]
pub fn MetricsBar(
    wpm: Signal<f64>,
    accuracy: Signal<f64>,
    current_phrase: Signal<usize>,
    total_phrases: usize,
) -> impl IntoView {
    view! {
        <div class="metrics-bar">
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

            <div class="metrics-bar__stat">
                <div class="metrics-bar__label">"Frase"</div>
                <div class="metrics-bar__value">
                    {move || format!("{}/{}", current_phrase.get(), total_phrases)}
                </div>
            </div>
        </div>
    }
}

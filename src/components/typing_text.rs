use leptos::prelude::*;
use std::time::Duration;

#[component]
pub fn TypingText(
    text: &'static str,
    #[prop(default = 3)] speed_ms: u64,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] delay_ms: Option<u64>,
) -> impl IntoView {
    let (displayed_text, set_displayed_text) = signal(String::new());
    let (show_cursor, set_show_cursor) = signal(true);

    Effect::new(move || {
        let chars: Vec<char> = text.chars().collect();
        let total = chars.len();
        let initial_delay = delay_ms.unwrap_or(0);

        for i in 0..=total {
            let text_slice: String = chars[..i].iter().collect();
            set_timeout(
                move || {
                    set_displayed_text.set(text_slice.clone());
                    if i == total {
                        set_show_cursor.set(false);
                    }
                },
                Duration::from_millis(initial_delay + (speed_ms * i as u64)),
            );
        }
    });

    view! {
        <span class=class style="white-space: pre-wrap;">
            {move || displayed_text.get()}
            {move || if show_cursor.get() {
                view! { <span class="typing-cursor">"|"</span> }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}
        </span>
    }
}

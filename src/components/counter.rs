use leptos::prelude::*;

#[component]
pub fn Counter() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <button
            style="background: blue; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer;"
            on:click=move |_| set_count.update(|n| *n += 1)
        >
            "Click me: "
            {count}
        </button>
        <p>
            "Double count: "
            {move || count.get() * 2}
        </p>
    }
}

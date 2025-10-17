use crate::components::ZenMode;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <style>
            {r#"
.home {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 3rem 0;
    gap: 3rem;
}

.home__intro {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    gap: 1rem;
}

.home__badge {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    background: rgba(247, 76, 0, 0.1);
    color: var(--color-primary);
    padding: 0.5rem 1rem;
    border-radius: 8px;
    font-size: 0.9rem;
    font-weight: 500;
    font-family: var(--font-family-mono);
}

.home__title {
    font-size: 3rem;
    font-weight: 300;
    color: var(--color-text);
    letter-spacing: -0.02em;
}

.home__title-highlight {
    color: var(--color-primary);
    font-weight: 500;
}

.home__subtitle {
    max-width: 600px;
    color: var(--color-text-muted);
    line-height: 1.8;
    font-size: 1rem;
    font-weight: 400;
}

.home__game {
    width: 100%;
    max-width: 1200px;
}

@media (max-width: 768px) {
    .home__title {
        font-size: 2rem;
    }

    .home__subtitle {
        font-size: 0.95rem;
    }
}
            "#}
        </style>

        <main class="container home">
            <div class="home__intro">
                <div class="home__badge">
                    <span>"🦀"</span>
                    <span>"rust + wasm"</span>
                </div>

                <h1 class="home__title">
                    <span class="home__title-highlight">"litterix"</span>
                </h1>

                <p class="home__subtitle">
                    "allenati, migliora la tua velocità, batti i tuoi record"
                </p>
            </div>

            <div class="home__game">
                <ZenMode />
            </div>
        </main>
    }
}

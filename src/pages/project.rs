use leptos::prelude::*;

#[component]
pub fn Project() -> impl IntoView {
    view! {
        <style>
            {r#"
.project {
    padding: 3rem 0;
    max-width: 900px;
    margin: 0 auto;
}

.hero {
    display: flex;
    flex-direction: column;
    gap: 2rem;
}

.hero__badge {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    background: rgba(247, 76, 0, 0.1);
    color: var(--color-primary);
    padding: 0.5rem 1rem;
    border-radius: 8px;
    font-weight: 500;
    font-size: 0.9rem;
    width: fit-content;
    font-family: var(--font-family-mono);
}

.hero__title {
    font-size: 3rem;
    font-weight: 300;
    color: var(--color-text);
    letter-spacing: -0.02em;
}

.hero__title-highlight {
    color: var(--color-primary);
    font-weight: 500;
}

.hero__description {
    color: var(--color-text-muted);
    line-height: 1.8;
    font-size: 1.1rem;
}

.hero__description strong {
    color: var(--color-text);
    font-weight: 500;
}

.hero__description em {
    color: var(--color-primary);
    font-style: normal;
}

.about-section {
    margin-top: 3rem;
    padding-top: 2rem;
    border-top: 1px solid var(--color-surface);
}

.section-title {
    font-size: 1.5rem;
    font-weight: 500;
    color: var(--color-text);
    margin-bottom: 1.5rem;
    letter-spacing: -0.01em;
}

.about-section p {
    color: var(--color-text-muted);
    line-height: 1.8;
    margin-bottom: 1.5rem;
}

.about-section p strong {
    color: var(--color-text);
    font-weight: 500;
}

.stack-list {
    list-style: none;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

.stack-list li {
    display: flex;
    align-items: flex-start;
    gap: 1rem;
    color: var(--color-text-muted);
    line-height: 1.6;
}

.stack-list li::before {
    content: "→";
    color: var(--color-primary);
    font-weight: 700;
    flex-shrink: 0;
}

.stack-list strong {
    color: var(--color-text);
    font-weight: 500;
}

@media (max-width: 768px) {
    .hero__title {
        font-size: 2rem;
    }

    .hero__description {
        font-size: 1rem;
    }
}
            "#}
        </style>

        <main class="container project">
            <section class="hero">
                <div class="hero__content">
                    <div class="hero__badge">
                        <span>"💡"</span>
                        <span>"il progetto"</span>
                    </div>

                    <h1 class="hero__title">
                        "benvenuto in "
                        <span class="hero__title-highlight">"litterix"</span>
                    </h1>

                    <p class="hero__description">
                        "litterix nasce come esperimento rust + webassembly per creare un typing game reattivo, preciso e fluido. "
                        "progettato per offrire "
                        <strong>"due modalità di gioco uniche"</strong>
                        ": una classica sfida da tastiera e una modalità "
                        <em>"tap-based"</em>" per dispositivi touch."
                    </p>

                    <div class="about-section">
                        <h2 class="section-title">"un motore, due sfide"</h2>
                        <p>
                            "al cuore del gioco c'è il "
                            <code>"TypingEngine"</code>
                            ", pensato per essere "
                            <strong>"dual-mode"</strong>"."
                        </p>
                        <ul class="stack-list">
                            <li>
                                <span>
                                    <strong>"desktop"</strong>
                                    " — dattilografia classica e rigorosa. ogni errore blocca il flusso, ideale per allenare la precisione."
                                </span>
                            </li>
                            <li>
                                <span>
                                    <strong>"touch"</strong>
                                    " — una sfida di rapidità e ricerca. tocca le parole corrette in sequenza tra quelle mischiate."
                                </span>
                            </li>
                        </ul>
                        <p>
                            "due esperienze, due classifiche: una per la tastiera, una per il tocco."
                        </p>
                    </div>

                    <div class="about-section">
                        <h2 class="section-title">"le frasi"</h2>
                        <p>
                            "ogni partita è unica: un algoritmo procedurale genera testi dinamici combinando nuclei semantici (s-v-o) e strutture via via più complesse."
                        </p>
                    </div>

                    <div class="about-section">
                        <h2 class="section-title">"lo stack"</h2>
                        <ul class="stack-list">
                            <li>
                                <span>
                                    <strong>"rust"</strong>
                                    " — per velocità e sicurezza."
                                </span>
                            </li>
                            <li>
                                <span>
                                    <strong>"leptos"</strong>
                                    " — framework reattivo per il web in rust."
                                </span>
                            </li>
                            <li>
                                <span>
                                    <strong>"webassembly"</strong>
                                    " — esecuzione nativa nel browser."
                                </span>
                            </li>
                        </ul>
                    </div>
                </div>
            </section>
        </main>
    }
}

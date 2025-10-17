// src/pages/project.rs
//
use leptos::prelude::*;

#[component]
pub fn Project() -> impl IntoView {
    view! {
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

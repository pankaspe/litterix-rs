use crate::components::TypingText;
use leptos::prelude::*;

struct BioData {
    name: &'static str,
    title: &'static str,
    description: &'static str,
    skills: &'static [&'static str],
    interests: &'static [&'static str],
}

const BIO: BioData = BioData {
    name: "$ Andrea B.",
    title: "> Rust Developer",
    description: "Appassionato di Rust e web development. Creo applicazioni performanti e scalabili.\nMi piace fotografare. Ma non solo catturare immagini — catturare dati.\n\nQuesta applicazione, scritta in Rust e Leptos, è la fusione delle mie due passioni: la programmazione e la fotografia.\nNon è un semplice portfolio. È un esperimento.\nOgni foto vive su due livelli: quello che l'occhio umano percepisce… e quello che la macchina interpreta.\n\nCliccando sul pulsante \"Rust\" puoi scendere sotto la superficie, vedere i byte, ascoltare come il computer vede la realtà.\nLa luce si trasforma in codice.\nE il codice, in una visione diversa del mondo.",
    skills: &["Rust", "Leptos", "WebAssembly", "Backend"],
    interests: &["Performance", "Open Source", "System Programming"],
};

#[component]
pub fn Bio() -> impl IntoView {
    // Calcola i delay
    let name_chars = BIO.name.len();
    let title_chars = BIO.title.len();
    let description_chars = BIO.description.len();

    let title_delay = (name_chars * 15) as u64;
    let description_delay = title_delay + (title_chars * 15) as u64;
    let skills_delay = description_delay + (description_chars * 15) as u64;

    // Signal per mostrare le card
    let (show_cards, set_show_cards) = signal(false);

    // Mostra le card dopo che finisce la description
    Effect::new(move || {
        set_timeout(
            move || set_show_cards.set(true),
            std::time::Duration::from_millis(skills_delay),
        );
    });

    view! {
        <style>
            {r#"
                .bio {
                    font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
                    font-size: 14px;
                    line-height: 1.8;
                    padding: 20px;
                }

                .bio h1 {
                    color: #4ec9b0;
                    font-size: 24px;
                    margin-bottom: 8px;
                }

                .bio h2 {
                    color: #569cd6;
                    font-size: 16px;
                    margin-bottom: 16px;
                }

                .bio h3 {
                    color: #c586c0;
                    font-size: 16px;
                    margin-bottom: 12px;
                    display: block;
                }

                .bio h3::before {
                    content: '# ';
                    color: var(--accent);
                }

                .bio p {
                    color: var(--text-primary);
                    margin-bottom: 20px;
                    white-space: pre-wrap;
                    word-wrap: break-word;
                    line-height: 1.6;
                }

                .cards-container {
                    display: grid;
                    grid-template-columns: 1fr 1fr;
                    gap: 16px;
                    margin-top: 20px;
                    opacity: 0;
                    transform: translateY(20px);
                    transition: opacity 0.8s ease-out, transform 0.8s ease-out;
                }

                .cards-container.show {
                    opacity: 1;
                    transform: translateY(0);
                }

                @media (max-width: 768px) {
                    .cards-container {
                        grid-template-columns: 1fr;
                    }
                }

                .bio .card {
                    background: transparent;
                    border: 1px solid var(--border);
                    border-left: 3px solid var(--accent);
                    padding: 16px;
                }

                .bio ul {
                    list-style: none;
                    padding-left: 0;
                    margin-top: 8px;
                }

                .bio li {
                    color: var(--text-primary);
                    padding: 4px 0;
                    padding-left: 20px;
                    position: relative;
                }

                .bio li::before {
                    content: "→";
                    color: var(--accent);
                    position: absolute;
                    left: 0;
                }

                .terminal-prompt::before {
                    color: #4ec9b0;
                    font-weight: bold;
                }

                .typing-cursor {
                    color: var(--accent);
                }

                @keyframes blink-caret {
                    from, to {
                        opacity: 1;
                    }
                    50% {
                        opacity: 0;
                    }
                }

            "#}
        </style>

        <div class="bio">
            <h1>
                <TypingText text=BIO.name speed_ms=15 />
            </h1>

            <h2>
                <TypingText text=BIO.title speed_ms=15 delay_ms=title_delay />
            </h2>

            <p class="terminal-prompt">
                <TypingText text=BIO.description speed_ms=15 delay_ms=description_delay />
            </p>

            <div class=move || if show_cards.get() { "cards-container show" } else { "cards-container" }>
                <div class="card">
                    <h3>
                        <TypingText text="Skills" speed_ms=15 delay_ms=skills_delay />
                    </h3>
                    <ul>
                        {BIO.skills.iter().enumerate()
                            .map(|(i, skill)| {
                                let item_delay = skills_delay + 100 + (i as u64 * 200);
                                view! {
                                    <li>
                                        <TypingText text=skill speed_ms=15 delay_ms=item_delay />
                                    </li>
                                }
                            })
                            .collect::<Vec<_>>()
                        }
                    </ul>
                </div>

                <div class="card">
                    <h3>
                        <TypingText text="Interessi" speed_ms=15 delay_ms=(skills_delay + 200) />
                    </h3>
                    <ul>
                        {BIO.interests.iter().enumerate()
                            .map(|(i, interest)| {
                                let item_delay = skills_delay + 300 + (i as u64 * 200);
                                view! {
                                    <li>
                                        <TypingText text=interest speed_ms=15 delay_ms=item_delay />
                                    </li>
                                }
                            })
                            .collect::<Vec<_>>()
                        }
                    </ul>
                </div>
            </div>

        </div>
    }
}

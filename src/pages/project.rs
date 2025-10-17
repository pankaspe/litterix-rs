use leptos::prelude::*;

#[component]
pub fn Project() -> impl IntoView {
    view! {
        <main class="container project">

            <h1 class="project__title">
                "Un esperimento in "
                <span class="project__highlight">"Rust e WebAssembly"</span>
            </h1>

            <p class="project__description">
                "Litterix nasce con un obiettivo: esplorare il potenziale di Rust per creare un'esperienza web reattiva, precisa e nativamente performante. È un'applicazione per l'allenamento della battitura costruita interamente con Leptos, un framework reattivo che compila in WebAssembly."
            </p>

            <div class="project__section">
                <h2 class="project__section-title">"Le Modalità di Gioco"</h2>
                <p>
                    "L'applicazione offre due sfide distinte, alimentate dallo stesso motore di battitura."
                </p>
                <ul class="project__list">
                    <li>
                        <strong>"Zen Mode"</strong>
                        " — Un flusso continuo di frasi per allenare il ritmo e la precisione senza la pressione del tempo. Ideale per entrare nello stato di flow e migliorare la memoria muscolare."
                    </li>
                    <li>
                        <strong>"Rush Mode"</strong>
                        " — Una corsa contro il tempo. Completa le frasi per guadagnare secondi preziosi, dove ogni errore conta e la velocità è tutto."
                    </li>
                </ul>
            </div>

            <div class="project__section">
                <h2 class="project__section-title">"Lo Stack Tecnologico"</h2>
                <ul class="project__list">
                    <li>
                        <strong>"Rust"</strong>
                        " — Per la sua garanzia di sicurezza e le sue performance eccezionali."
                    </li>
                    <li>
                        <strong>"Leptos"</strong>
                        " — Un framework reattivo moderno per costruire interfacce web interamente in Rust."
                    </li>
                    <li>
                        <strong>"WebAssembly (WASM)"</strong>
                        " — Per eseguire il codice Rust nel browser a velocità quasi nativa, garantendo una fluidità impeccabile."
                    </li>
                </ul>
            </div>

        </main>
    }
}

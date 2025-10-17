# Litterix

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)![Leptos](https://img.shields.io/badge/Framework-Leptos-F74C00?style=for-the-badge&logo=rust)![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=webassembly&logoColor=white)![Status: In Sviluppo](https://img.shields.io/badge/Status-In%20Sviluppo-orange?style=for-the-badge)

Un'esperienza di battitura minimale e performante, costruita con Rust e WebAssembly.

<!-- Inserisci qui uno screenshot o una GIF dell'applicazione! -->
<!-- ![Screenshot di Litterix](https://user-images.githubusercontent.com/path/to/your/screenshot.png) -->

---

> **Litterix** è un typing game nato come esperimento per esplorare il potenziale del framework **Leptos** e dell'ecosistema Rust per la creazione di esperienze web reattive, precise e nativamente performanti. L'obiettivo è offrire un'interfaccia pulita e minimale dove l'utente possa allenare la propria velocità di battitura, concentrandosi su ritmo e accuratezza.

## Le Modalità di Gioco

L'applicazione offre due sfide distinte, alimentate dallo stesso motore di battitura reattivo.

-   **🧘 Zen Mode**: Un flusso continuo e infinito di frasi per allenare il ritmo e la precisione senza la pressione del tempo. Ideale per entrare nello stato di *flow* e migliorare la memoria muscolare.
-   **⏱️ Rush Mode**: Una corsa contro il tempo. Completa le frasi correttamente per guadagnare secondi preziosi, dove ogni errore conta e la velocità è fondamentale per stabilire un nuovo record.

## Caratteristiche Tecniche

-   **Motore di Battitura Reattivo**: Scritto interamente in Rust, il `TypingEngine` gestisce lo stato carattere per carattere, supportando input Unicode, accenti e gestione avanzata del backspace.
-   **State Management con Signals**: Lo stato dell'applicazione è gestito in modo granulare e reattivo tramite i `signal` di Leptos, garantendo aggiornamenti del DOM efficienti e senza ridisegni inutili.
-   **Performance Native con WASM**: Compilato in WebAssembly, il codice Rust viene eseguito nel browser a velocità quasi nativa, assicurando un feedback istantaneo a ogni pressione dei tasti e un'esperienza utente fluida.
-   **Architettura a Componenti Modulare**: L'interfaccia è suddivisa in componenti riutilizzabili e disaccoppiati (`TypingEngine`, `MetricsBar`, `Game`), rendendo il codice pulito e facilmente estensibile.
-   **UI Minimale e Intenzionale**: Ogni elemento dell'interfaccia è stato progettato per ridurre il disordine visivo e fornire un feedback chiaro e immediato, come gli indicatori di focus dinamici e le metriche pulite.

## Stack Tecnologico

-   🦀 **Rust**: Per la sua garanzia di sicurezza, performance eccezionali e un ecosistema moderno.
-   🚀 **Leptos**: Un framework reattivo all'avanguardia per costruire interfacce web interamente in Rust.
-   🕸️ **WebAssembly (WASM)**: Per eseguire il codice Rust nel browser, eliminando la necessità di JavaScript e garantendo una fluidità impeccabile.

## Roadmap

Litterix è un progetto in continua evoluzione. Le prossime funzionalità includono:

-   [ ] Aggiunta autenticazione.
-   [ ] Implementazione di classifiche e statistiche utente.
-   [ ] Nuove modalità di gioco (es. "Quote Mode" con citazioni famose).
-   [ ] Miglioramento dell'accessibilità.

## Licenza

Questo progetto è rilasciato sotto la [Licenza MIT](LICENSE).

# 🦀 Litterix

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Leptos](https://img.shields.io/badge/Framework-Leptos-F74C00?style=for-the-badge&logo=rust)
![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=webassembly&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)

> Un'esperienza di battitura minimale e performante, costruita interamente in Rust e WebAssembly.

**Litterix** è un typing game nato come esperimento per esplorare il potenziale del framework **Leptos** e dell'ecosistema Rust per la creazione di esperienze web reattive, precise e nativamente performanti. L'obiettivo è offrire un'interfaccia pulita e minimale dove l'utente possa allenare la propria velocità di battitura, concentrandosi su ritmo e accuratezza.

---

## ✨ Caratteristiche

### 🎮 Due Modalità di Gioco

- **🧘 Zen Mode**: Un flusso continuo e infinito di frasi per allenare il ritmo e la precisione senza la pressione del tempo. Ideale per entrare nello stato di *flow* e migliorare la memoria muscolare.

- **⏱️ Rush Mode**: Una corsa contro il tempo dove ogni secondo conta. Completa le frasi correttamente per guadagnare tempo prezioso. L'accuratezza ti premia con bonus temporali:
  - >100% accuracy: +5 secondi
  - >75% accuracy: +3 secondi
  - >50% accuracy: +2 secondi
  - >25% accuracy: +1 secondo

### 📊 Metriche in Tempo Reale

- **WPM (Words Per Minute)**: Misura la tua velocità di battitura
- **Accuracy**: Percentuale di precisione carattere per carattere
- **Caratteri e Parole**: Contatori progressivi della sessione
- **Timer**: Tempo rimanente in Rush Mode

### ⚙️ Impostazioni Personalizzabili

Scegli tra tre livelli di difficoltà:
- **Base**: Frasi semplici e dirette per principianti
- **Intermedio**: Vocabolario più ricco per utenti con esperienza
- **Avanzato**: Frasi lunghe e articolate per veri maestri della tastiera

Le impostazioni vengono salvate localmente nel browser (localStorage) per preservare la tua esperienza tra le sessioni.

---

## 🛠️ Stack Tecnologico

- **🦀 Rust**: Per sicurezza garantita, performance eccezionali e un ecosistema moderno
- **🚀 Leptos 0.8**: Framework reattivo all'avanguardia per costruire interfacce web interamente in Rust
- **🕸️ WebAssembly (WASM)**: Esegue il codice Rust nel browser a velocità quasi nativa
- **📦 Trunk**: Build tool e dev server per applicazioni WASM
- **🎨 CSS Modules**: Stili componibilizzati e modulari

---

## 🏗️ Architettura

### Motore di Battitura Reattivo

Il cuore di Litterix è il `TypingEngine`, scritto interamente in Rust:
- Gestione dello stato carattere per carattere
- Supporto completo per Unicode e caratteri accentati
- Gestione avanzata del backspace con correzione intelligente
- Calcolo real-time di WPM e accuracy
- Callbacks personalizzabili per eventi (carattere digitato, parola completata, frase completata)

### State Management Granulare

Lo stato dell'applicazione è gestito con i **signals** di Leptos:
- Aggiornamenti del DOM efficienti e mirati
- Nessun re-render inutile
- Reattività fine-grained per performance ottimali

### Componenti Modulari

Architettura pulita e disaccoppiata:
- `TypingEngine`: Motore di battitura riutilizzabile
- `MetricsBar`: Visualizzazione metriche configurabile
- `ZenMode` / `RushMode`: Modalità di gioco indipendenti
- `SettingsContext`: Gestione globale delle impostazioni

---

## 🎯 Roadmap

- [ ] **Temi personalizzabili**: Dark mode, light mode, e temi della community
- [ ] **Statistiche avanzate**: Grafici di progresso, storia delle sessioni
- [ ] **Modalità Practice**: Allenamento su caratteri specifici o combinazioni difficili
- [ ] **Leaderboard locale**: Traccia i tuoi migliori record
- [ ] **Supporto multilingua**: Dataset in inglese, spagnolo, francese
- [ ] **Suoni e feedback audio**: Feedback sonoro opzionale per digitazione
- [ ] **Modalità Multiplayer**: Sfida amici in tempo reale (con WebSockets)

---

## 📝 Licenza

Questo progetto è rilasciato sotto la [Licenza MIT](LICENSE). Vedi il file `LICENSE` per i dettagli completi.

---

## 🙏 Ringraziamenti

- Il team di [Leptos](https://github.com/leptos-rs/leptos) per il framework eccezionale
- La community Rust per il supporto e le risorse
- [Monkeytype](https://monkeytype.com/) per l'ispirazione sul design minimale

---

## 📚 Risorse Utili

- [Documentazione Leptos](https://leptos.dev/)
- [The Rust Book](https://doc.rust-lang.org/book/)
- [WebAssembly Concepts](https://developer.mozilla.org/en-US/docs/WebAssembly/Concepts)

---

<p align="center">
  Fatto con ❤️ e 🦀 da Andrea B.
</p>

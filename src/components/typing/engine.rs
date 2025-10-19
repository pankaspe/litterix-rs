// src/components/typing/engine.rs (AGGIORNATO)
//
use leptos::ev::KeyboardEvent;
use leptos::html::Input;
use leptos::prelude::*;
use web_sys::window;

#[derive(Clone, Debug, PartialEq)]
pub enum CharStatus {
    Pending,
    Correct,
    Incorrect,
}

#[derive(Clone, Debug)]
pub struct TypingState {
    pub text: String,
    pub current_index: usize,
    pub char_statuses: Vec<CharStatus>,
    pub is_complete: bool,
    pub started: bool,
    pub start_time: Option<f64>,
    pub end_time: Option<f64>,
}

impl TypingState {
    pub fn new(text: String) -> Self {
        let char_count = text.chars().count();
        Self {
            text,
            current_index: 0,
            char_statuses: vec![CharStatus::Pending; char_count],
            is_complete: false,
            started: false,
            start_time: None,
            end_time: None,
        }
    }

    // Ritorna true se il carattere è corretto, false se è sbagliato
    pub fn handle_key(&mut self, key: &str) -> bool {
        if self.is_complete {
            return true;
        }

        if !self.started {
            self.started = true;
            if let Some(win) = window() {
                if let Some(perf) = win.performance() {
                    self.start_time = Some(perf.now());
                }
            }
        }

        let chars: Vec<char> = self.text.chars().collect();

        if self.current_index >= chars.len() {
            return true;
        }

        let expected_char = chars[self.current_index];
        let input_char = key.chars().next().unwrap_or('\0');

        let is_correct = input_char == expected_char;

        if is_correct {
            self.char_statuses[self.current_index] = CharStatus::Correct;
        } else {
            self.char_statuses[self.current_index] = CharStatus::Incorrect;
        }

        self.current_index += 1;

        if self.current_index >= chars.len() {
            self.is_complete = true;
            if let Some(win) = window() {
                if let Some(perf) = win.performance() {
                    self.end_time = Some(perf.now());
                }
            }
        }

        is_correct
    }

    pub fn handle_backspace(&mut self) -> bool {
        if self.current_index > 0 {
            self.current_index -= 1;
            self.char_statuses[self.current_index] = CharStatus::Pending;
            true
        } else {
            false
        }
    }

    pub fn get_wpm(&self) -> Option<f64> {
        if let (Some(start), Some(end)) = (self.start_time, self.end_time) {
            let duration_ms = end - start;
            let duration_min = duration_ms / 60000.0;
            let word_count = self.text.split_whitespace().count() as f64;
            Some(word_count / duration_min)
        } else {
            None
        }
    }

    pub fn get_accuracy(&self) -> f64 {
        let total = self.char_statuses.len() as f64;
        if total == 0.0 {
            return 100.0;
        }
        let correct = self
            .char_statuses
            .iter()
            .filter(|s| **s == CharStatus::Correct)
            .count() as f64;
        (correct / total) * 100.0
    }
}

#[component]
pub fn TypingEngine(
    text: String,
    #[prop(optional)] on_complete: Option<Callback<(f64, f64)>>,
    #[prop(optional)] on_char_typed: Option<Callback<()>>,
    #[prop(optional)] on_char_error: Option<Callback<()>>,
    #[prop(optional)] on_word_typed: Option<Callback<()>>,
    #[prop(optional)] on_word_deleted: Option<Callback<()>>,
) -> impl IntoView {
    let (state, set_state) = signal(TypingState::new(text.clone()));
    let input_ref = NodeRef::<Input>::new();

    Effect::new({
        let input_ref = input_ref.clone();
        move || {
            if let Some(input) = input_ref.get() {
                let _ = input.focus();
            }
        }
    });

    let handle_input = move |_| {
        if let Some(input) = input_ref.get() {
            let value = input.value();
            if value.is_empty() {
                return;
            }

            let last_char = value.chars().last().unwrap();

            set_state.update(|s| {
                if s.current_index == 0 && last_char == ' ' {
                    return;
                }

                let is_correct = s.handle_key(&last_char.to_string());

                if let Some(callback) = on_char_typed {
                    callback.run(());
                }

                // Nuovo: notifica se il carattere è sbagliato
                if !is_correct {
                    if let Some(callback) = on_char_error {
                        callback.run(());
                    }
                }

                if s.current_index > 0 {
                    let processed_char_index = s.current_index - 1;
                    let text_char = s.text.chars().nth(processed_char_index).unwrap();
                    let char_status = &s.char_statuses[processed_char_index];

                    if text_char == ' ' && *char_status == CharStatus::Correct {
                        if let Some(callback) = on_word_typed {
                            callback.run(());
                        }
                    }
                }

                if s.is_complete {
                    if !s.text.ends_with(' ') {
                        if let Some(callback) = on_word_typed {
                            callback.run(());
                        }
                    }

                    if let Some(callback) = on_complete {
                        if let Some(wpm) = s.get_wpm() {
                            let accuracy = s.get_accuracy();
                            callback.run((wpm, accuracy));
                        }
                    }
                }
            });

            input.set_value("");
        }
    };

    let handle_keydown = move |ev: KeyboardEvent| {
        if ev.key() == "Backspace" {
            ev.prevent_default();
            set_state.update(|s| {
                if s.current_index > 0 {
                    let index_to_delete = s.current_index - 1;
                    let char_to_delete = s.text.chars().nth(index_to_delete).unwrap();
                    let status_of_char = &s.char_statuses[index_to_delete];

                    if char_to_delete == ' ' && *status_of_char == CharStatus::Correct {
                        if let Some(callback) = on_word_deleted {
                            callback.run(());
                        }
                    }
                }

                s.handle_backspace();
            });
        }

        if ev.repeat() {
            ev.prevent_default();
            return;
        }
    };

    let handle_click = move |_| {
        if let Some(input) = input_ref.get() {
            let _ = input.focus();
        }
    };

    view! {
        <div class="typing-engine">
            <div class="typing-display" on:click=handle_click>
                <input
                    node_ref=input_ref
                    type="text"
                    class="typing-input"
                    on:input=handle_input
                    on:keydown=handle_keydown
                    autocomplete="off"
                    autocapitalize="off"
                    spellcheck="false"
                />
                <div class="typing-text">
                    {move || {
                        let s = state.get();
                        let chars: Vec<char> = s.text.chars().collect();

                        let mut words = Vec::new();
                        let mut current_word_chars = Vec::new();
                        let mut char_index = 0;

                        for ch in chars.iter() {
                            if *ch == ' ' {
                                if !current_word_chars.is_empty() {
                                    words.push((current_word_chars.clone(), false));
                                    current_word_chars.clear();
                                }
                                words.push((vec![(char_index, ' ')], true));
                                char_index += 1;
                            } else {
                                current_word_chars.push((char_index, *ch));
                                char_index += 1;
                            }
                        }

                        if !current_word_chars.is_empty() {
                            words.push((current_word_chars, false));
                        }

                        words.into_iter().map(|(word_chars, _)| {
                            view! {
                                <span class="typing-word">
                                    {word_chars.into_iter().map(|(i, ch)| {
                                        let status = &s.char_statuses[i];
                                        let is_current = i == s.current_index;

                                        let mut class = match status {
                                            CharStatus::Pending => "typing-char typing-char--pending",
                                            CharStatus::Correct => "typing-char typing-char--correct",
                                            CharStatus::Incorrect => "typing-char typing-char--incorrect",
                                        }
                                        .to_string();

                                        if is_current {
                                            class.push_str(" typing-char--current");
                                        }

                                        let ch_display = if ch == ' ' { '\u{00A0}' } else { ch };

                                        view! { <span class=class>{ch_display}</span> }
                                    }).collect_view()}
                                </span>
                            }
                        }).collect_view()
                    }}
                </div>
            </div>
        </div>
    }
}

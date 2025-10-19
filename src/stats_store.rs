// src/stats_store.rs
//
// Sistema di tracking delle statistiche dell'utente

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use web_sys::window;

const STATS_STORAGE_KEY: &str = "litterix_stats";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameStats {
    // Statistiche generali
    pub total_games_played: u32,
    pub total_words_typed: u32,
    pub total_chars_typed: u32,
    pub total_time_played: f64, // in secondi

    // Record personali
    pub best_wpm: f64,
    pub best_accuracy: f64,
    pub highest_combo: usize,

    // Medie
    pub average_wpm: f64,
    pub average_accuracy: f64,

    // Per modalità
    pub zen_games: u32,
    pub rush_games: u32,
    pub marathon_games: u32,
    pub marathon_best_score: u32,
}

impl Default for GameStats {
    fn default() -> Self {
        Self {
            total_games_played: 0,
            total_words_typed: 0,
            total_chars_typed: 0,
            total_time_played: 0.0,
            best_wpm: 0.0,
            best_accuracy: 0.0,
            highest_combo: 0,
            average_wpm: 0.0,
            average_accuracy: 0.0,
            zen_games: 0,
            rush_games: 0,
            marathon_games: 0,
            marathon_best_score: 0,
        }
    }
}

impl GameStats {
    pub fn load_from_storage() -> Self {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(json)) = storage.get_item(STATS_STORAGE_KEY) {
                    if let Ok(stats) = serde_json::from_str::<GameStats>(&json) {
                        return stats;
                    }
                }
            }
        }
        Self::default()
    }

    pub fn save_to_storage(&self) -> Result<(), JsValue> {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(json) = serde_json::to_string(self) {
                    return storage.set_item(STATS_STORAGE_KEY, &json);
                }
            }
        }
        Ok(())
    }

    pub fn clear_storage() -> Result<(), JsValue> {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                return storage.remove_item(STATS_STORAGE_KEY);
            }
        }
        Ok(())
    }

    pub fn has_played(&self) -> bool {
        self.total_games_played > 0
    }
}

#[derive(Clone, Copy)]
pub struct StatsContext {
    stats: RwSignal<GameStats>,
}

impl StatsContext {
    pub fn new() -> Self {
        let saved_stats = GameStats::load_from_storage();
        Self {
            stats: RwSignal::new(saved_stats),
        }
    }

    pub fn get_stats(&self) -> GameStats {
        self.stats.get()
    }

    // Aggiorna le statistiche dopo una partita
    pub fn record_game(
        &self,
        mode: GameMode,
        words: u32,
        chars: u32,
        time: f64,
        wpm: f64,
        accuracy: f64,
        combo: usize,
        marathon_score: Option<u32>,
    ) {
        self.stats.update(|s| {
            s.total_games_played += 1;
            s.total_words_typed += words;
            s.total_chars_typed += chars;
            s.total_time_played += time;

            // Aggiorna record
            if wpm > s.best_wpm {
                s.best_wpm = wpm;
            }
            if accuracy > s.best_accuracy {
                s.best_accuracy = accuracy;
            }
            if combo > s.highest_combo {
                s.highest_combo = combo;
            }

            // Aggiorna medie
            let total_games = s.total_games_played as f64;
            s.average_wpm = (s.average_wpm * (total_games - 1.0) + wpm) / total_games;
            s.average_accuracy = (s.average_accuracy * (total_games - 1.0) + accuracy) / total_games;

            // Aggiorna contatori per modalità
            match mode {
                GameMode::Zen => s.zen_games += 1,
                GameMode::Rush => s.rush_games += 1,
                GameMode::Marathon => {
                    s.marathon_games += 1;
                    if let Some(score) = marathon_score {
                        if score > s.marathon_best_score {
                            s.marathon_best_score = score;
                        }
                    }
                }
            }
        });

        let _ = self.stats.get().save_to_storage();
    }

    pub fn clear(&self) -> Result<(), JsValue> {
        GameStats::clear_storage()?;
        self.stats.set(GameStats::default());
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GameMode {
    Zen,
    Rush,
    Marathon,
}

pub fn use_stats() -> StatsContext {
    use_context::<StatsContext>().expect("StatsContext deve essere fornito a livello di App")
}

// src/settings_store.rs
//
// Gestisce le impostazioni dell'applicazione usando localStorage del browser

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use web_sys::window;

const STORAGE_KEY: &str = "litterix_settings";

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum DatasetDifficulty {
    Base,
    Intermediate,
    Advanced,
}

impl DatasetDifficulty {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Base => "base",
            Self::Intermediate => "intermediate",
            Self::Advanced => "advanced",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "intermediate" => Self::Intermediate,
            "advanced" => Self::Advanced,
            _ => Self::Base,
        }
    }

    pub fn get_dataset_content(&self) -> &'static str {
        match self {
            Self::Base => include_str!("../assets/datasets/base-dataset.json"),
            Self::Intermediate => include_str!("../assets/datasets/intermedie-dataset.json"),
            Self::Advanced => include_str!("../assets/datasets/advanced-dataset.json"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppSettings {
    pub difficulty: DatasetDifficulty,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            difficulty: DatasetDifficulty::Base,
        }
    }
}

impl AppSettings {
    // Carica le impostazioni da localStorage
    pub fn load_from_storage() -> Self {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(json)) = storage.get_item(STORAGE_KEY) {
                    if let Ok(settings) = serde_json::from_str::<AppSettings>(&json) {
                        return settings;
                    }
                }
            }
        }
        Self::default()
    }

    // Salva le impostazioni in localStorage
    pub fn save_to_storage(&self) -> Result<(), JsValue> {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(json) = serde_json::to_string(self) {
                    return storage.set_item(STORAGE_KEY, &json);
                }
            }
        }
        Ok(())
    }

    // Elimina le impostazioni da localStorage
    pub fn clear_storage() -> Result<(), JsValue> {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                return storage.remove_item(STORAGE_KEY);
            }
        }
        Ok(())
    }
}

// Context globale per le impostazioni
#[derive(Clone, Copy)]
pub struct SettingsContext {
    settings: RwSignal<AppSettings>,
}

impl SettingsContext {
    pub fn new() -> Self {
        // Carica le impostazioni salvate all'avvio
        let saved_settings = AppSettings::load_from_storage();
        Self {
            settings: RwSignal::new(saved_settings),
        }
    }

    pub fn get_difficulty(&self) -> DatasetDifficulty {
        self.settings.get().difficulty
    }

    pub fn set_difficulty(&self, difficulty: DatasetDifficulty) {
        self.settings.update(|s| s.difficulty = difficulty);
    }

    pub fn get_settings(&self) -> AppSettings {
        self.settings.get()
    }

    // Salva le impostazioni correnti in localStorage
    pub fn save(&self) -> Result<(), JsValue> {
        let settings = self.settings.get();
        settings.save_to_storage()
    }

    // Reset alle impostazioni di default e cancella localStorage
    pub fn reset(&self) -> Result<(), JsValue> {
        AppSettings::clear_storage()?;
        self.settings.set(AppSettings::default());
        Ok(())
    }
}

// Hook per usare il context
pub fn use_settings() -> SettingsContext {
    use_context::<SettingsContext>().expect("SettingsContext deve essere fornito a livello di App")
}

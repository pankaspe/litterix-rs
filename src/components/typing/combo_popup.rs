// src/components/typing/combo_popup.rs
//
use leptos::prelude::*;
use std::time::Duration;

#[derive(Clone, Debug, PartialEq)]
pub enum ComboType {
    Streak5,
    Streak10,
    Streak15,
    Streak20,
    Streak40,
    Streak80,
    Streak160,
    Streak320,
    Streak640,
    Streak1000,
    PerfectPhrase,
    ComboBroken,
}

impl ComboType {
    pub fn message(&self) -> &'static str {
        match self {
            Self::Streak5 => "🔥 Combo +5!",
            Self::Streak10 => "⚡ Combo +10!",
            Self::Streak15 => "💫 Combo +15!",
            Self::Streak20 => "🌟 Combo +20!",
            Self::Streak40 => "💥 COMBO +40!",
            Self::Streak80 => "🚀 MEGA COMBO +80!",
            Self::Streak160 => "⭐ ULTRA COMBO +160!",
            Self::Streak320 => "👑 LEGENDARY +320!",
            Self::Streak640 => "🔱 GODLIKE +640!",
            Self::Streak1000 => "🏆 UNSTOPPABLE +1000!",
            Self::PerfectPhrase => "✨ Frase Perfetta!",
            Self::ComboBroken => "💔 Combo Interrotta!",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            Self::Streak5 => "#ffc107",    // Giallo
            Self::Streak10 => "#ff9800",   // Arancione
            Self::Streak15 => "#ff5722",   // Rosso-arancio
            Self::Streak20 => "#e91e63",   // Rosa
            Self::Streak40 => "#9c27b0",   // Viola
            Self::Streak80 => "#673ab7",   // Viola scuro
            Self::Streak160 => "#3f51b5",  // Indaco
            Self::Streak320 => "#2196f3",  // Blu
            Self::Streak640 => "#00bcd4",  // Ciano
            Self::Streak1000 => "#ffd700", // Oro brillante
            Self::PerfectPhrase => "#f74c00",
            Self::ComboBroken => "#666666",
        }
    }
}

#[component]
pub fn ComboPopup(#[prop(into)] trigger: Signal<Option<ComboType>>) -> impl IntoView {
    let (visible, set_visible) = signal(false);
    let (current_combo, set_current_combo) = signal::<Option<ComboType>>(None);

    Effect::new(move |_| {
        if let Some(combo) = trigger.get() {
            set_current_combo.set(Some(combo));
            set_visible.set(true);

            set_timeout(
                move || {
                    set_visible.set(false);
                    set_timeout(
                        move || set_current_combo.set(None),
                        Duration::from_millis(300),
                    );
                },
                Duration::from_millis(2000),
            );
        }
    });

    view! {
        <Show when=move || current_combo.get().is_some() fallback=|| ()>
            <div
                class="combo-popup"
                class:combo-popup--visible=move || visible.get()
                style=move || {
                    current_combo.get()
                        .map(|c| format!("--combo-color: {}", c.color()))
                        .unwrap_or_default()
                }
            >
                <div class="combo-popup__content">
                    {move || current_combo.get().map(|c| c.message())}
                </div>
            </div>
        </Show>
    }
}

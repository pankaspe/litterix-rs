// src/app.rs
//
use crate::components::Navbar;
use crate::pages::{Home, Project, Settings};
use crate::settings_store::SettingsContext;
use leptos::context::Provider;
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

#[component]
pub fn App() -> impl IntoView {
    // Crea il context delle impostazioni
    let settings_ctx = SettingsContext::new();

    view! {
        <Router>
            <Provider value=settings_ctx>
                <Navbar />
                <main>
                    <Routes fallback=|| view! { <p>"Pagina non trovata."</p> }>
                        <Route path=path!("/") view=|| view! { <Home /> } />
                        <Route path=path!("/project") view=|| view! { <Project /> } />
                        <Route path=path!("/settings") view=|| view! { <Settings /> } />
                    </Routes>
                </main>
            </Provider>
        </Router>
    }
}

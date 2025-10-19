// src/app.rs
//
use crate::components::Navbar;
use crate::pages::{Dashboard, Home, Project, Settings};
use crate::settings_store::SettingsContext;
use crate::stats_store::StatsContext;
use leptos::context::Provider;
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

#[component]
pub fn App() -> impl IntoView {
    // Crea i context delle impostazioni e delle statistiche
    let settings_ctx = SettingsContext::new();
    let stats_ctx = StatsContext::new();

    view! {
        <Router>
            // Provider per le impostazioni
            <Provider value=settings_ctx>
                // Provider per le statistiche
                <Provider value=stats_ctx>
                    <Navbar />
                    <main>
                        <Routes fallback=|| view! { <p>"Pagina non trovata."</p> }>
                            <Route path=path!("/") view=|| view! { <Home /> } />
                            <Route path=path!("/project") view=|| view! { <Project /> } />
                            <Route path=path!("/dashboard") view=|| view! { <Dashboard /> } />
                            <Route path=path!("/settings") view=|| view! { <Settings /> } />
                        </Routes>
                    </main>
                </Provider>
            </Provider>
        </Router>
    }
}

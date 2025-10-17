// src/app.rs
//
use crate::components::Navbar;
use crate::pages::{Home, Project};
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Navbar />
            <main>
                <Routes fallback=|| view! { <p>"Pagina non trovata."</p> }>
                    <Route path=path!("/") view=|| view! { <Home /> } />
                    <Route path=path!("/project") view=|| view! { <Project /> } />
                </Routes>
            </main>
        </Router>
    }
}

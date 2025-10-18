// src/components/navbar.rs
//
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;

#[component]
pub fn Navbar() -> impl IntoView {
    let location = use_location();

    view! {
        <nav class="navbar">
            <div class="navbar__container">
                // --- Left: Navigation links ---
                <div class="navbar__left">
                    <A
                        href="/"
                        attr:class="navbar__link"
                        class:navbar__link--active=move || location.pathname.get() == "/"
                    >
                        "Home"
                    </A>

                    <A
                        href="/project"
                        attr:class="navbar__link"
                        class:navbar__link--active=move || location.pathname.get() == "/project"
                    >
                        "Info"
                    </A>
                </div>

                // --- Center: Logo ---
                <div class="navbar__center">
                    <A href="/" attr:class="navbar__logo">
                        <span class="navbar__logo-icon">"🦀"</span>
                        <span class="navbar__logo-text">"litterix"</span>
                    </A>
                </div>

                // --- Right: Login button ---
                <div class="navbar__right">
                    <A
                        href="/settings"
                        attr:class="navbar__login-btn"
                        class:navbar__link--active=move || location.pathname.get() == "/settings"
                    >
                        <span class="navbar__login-icon">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-settings-icon lucide-settings"><path d="M9.671 4.136a2.34 2.34 0 0 1 4.659 0 2.34 2.34 0 0 0 3.319 1.915 2.34 2.34 0 0 1 2.33 4.033 2.34 2.34 0 0 0 0 3.831 2.34 2.34 0 0 1-2.33 4.033 2.34 2.34 0 0 0-3.319 1.915 2.34 2.34 0 0 1-4.659 0 2.34 2.34 0 0 0-3.32-1.915 2.34 2.34 0 0 1-2.33-4.033 2.34 2.34 0 0 0 0-3.831A2.34 2.34 0 0 1 6.35 6.051a2.34 2.34 0 0 0 3.319-1.915"/><circle cx="12" cy="12" r="3"/></svg>                        </span>
                    </A>
                </div>
            </div>
        </nav>
    }
}

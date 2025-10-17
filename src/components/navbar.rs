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
                        "Bio"
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
                    <A href="/login" attr:class="navbar__login-btn">
                        <span class="navbar__login-icon">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-log-in-icon lucide-log-in"><path d="m10 17 5-5-5-5"/><path d="M15 12H3"/><path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"/></svg>
                        </span>
                    </A>
                </div>
            </div>
        </nav>
    }
}

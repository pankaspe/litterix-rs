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
                            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" fill="currentColor" viewBox="0 0 24 24">
                                <path fill-rule="evenodd" d="M12 4a4 4 0 1 0 0 8 4 4 0 0 0 0-8Zm-2 9a4 4 0 0 0-4 4v1a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2v-1a4 4 0 0 0-4-4h-4Z" clip-rule="evenodd"/>
                            </svg>
                        </span>
                        <span>"login"</span>
                    </A>
                </div>
            </div>
        </nav>
    }
}

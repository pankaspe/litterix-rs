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
                        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor" viewBox="0 0 24 24">
                            <path fill-rule="evenodd" d="M11.293 3.293a1 1 0 0 1 1.414 0l6 6 2 2a1 1 0 0 1-1.414 1.414L19 12.414V19a2 2 0 0 1-2 2h-3a1 1 0 0 1-1-1v-3h-2v3a1 1 0 0 1-1 1H7a2 2 0 0 1-2-2v-6.586l-.293.293a1 1 0 0 1-1.414-1.414l2-2 6-6Z" clip-rule="evenodd"/>
                        </svg>
                    </A>

                    <A
                        href="/project"
                        attr:class="navbar__link"
                        class:navbar__link--active=move || location.pathname.get() == "/project"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor" viewBox="0 0 24 24">
                            <path fill-rule="evenodd" d="M2 12C2 6.477 6.477 2 12 2s10 4.477 10 10-4.477 10-10 10S2 17.523 2 12Zm9.408-5.5a1 1 0 1 0 0 2h.01a1 1 0 1 0 0-2h-.01ZM10 10a1 1 0 1 0 0 2h1v3h-1a1 1 0 1 0 0 2h4a1 1 0 1 0 0-2h-1v-4a1 1 0 0 0-1-1h-2Z" clip-rule="evenodd"/>
                        </svg>
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

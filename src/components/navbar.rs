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

                // --- Right: Settings button ---
                <div class="navbar__right">
                    <A
                        href="/dashboard"
                        attr:class="navbar__link"
                        class:navbar__link--active=move || location.pathname.get() == "/dashboard"
                    >
                    <span class="navbar__login-icon">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-user-round-cog-icon lucide-user-round-cog"><path d="m14.305 19.53.923-.382"/><path d="m15.228 16.852-.923-.383"/><path d="m16.852 15.228-.383-.923"/><path d="m16.852 20.772-.383.924"/><path d="m19.148 15.228.383-.923"/><path d="m19.53 21.696-.382-.924"/><path d="M2 21a8 8 0 0 1 10.434-7.62"/><path d="m20.772 16.852.924-.383"/><path d="m20.772 19.148.924.383"/><circle cx="10" cy="8" r="5"/><circle cx="18" cy="18" r="3"/></svg>
                    </span>
                    </A>

                    <A
                        href="/settings"
                        attr:class="navbar__login-btn"
                        class:navbar__link--active=move || location.pathname.get() == "/settings"
                    >
                        <span class="navbar__login-icon">
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M9.671 4.136a2.34 2.34 0 0 1 4.659 0 2.34 2.34 0 0 0 3.319 1.915 2.34 2.34 0 0 1 2.33 4.033 2.34 2.34 0 0 0 0 3.831 2.34 2.34 0 0 1-2.33 4.033 2.34 2.34 0 0 0-3.319 1.915 2.34 2.34 0 0 1-4.659 0 2.34 2.34 0 0 0-3.32-1.915 2.34 2.34 0 0 1-2.33-4.033 2.34 2.34 0 0 0 0-3.831A2.34 2.34 0 0 1 6.35 6.051a2.34 2.34 0 0 0 3.319-1.915"/>
                                <circle cx="12" cy="12" r="3"/>
                            </svg>
                        </span>
                    </A>
                </div>
            </div>
        </nav>
    }
}

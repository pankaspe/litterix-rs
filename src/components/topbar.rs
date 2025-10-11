use leptos::prelude::*;

#[component]
pub fn Topbar() -> impl IntoView {
    let contacts = vec![
        ("GitHub", "https://github.com/pankaspe"),
        ("Instagram", "https://instagram.com/pankaspe"),
    ];

    view! {
        <style>
            {r#"
                .logo {
                    font-weight: 600;
                    font-size: 14px;
                    color: var(--text-primary);
                }
                .contact {
                    display: flex;
                    align-items: center;
                }
                .contact ul {
                    list-style: none;
                    display: flex;
                    gap: 16px;
                    margin: 0;
                    padding: 0;
                }
                .contact a {
                    color: var(--text-secondary);
                    transition: color 0.2s;
                    display: flex;
                    align-items: center;
                    font-size: 14px;
                }
                .contact a:hover {
                    color: var(--accent);
                    text-decoration: none;
                }
            "#}
        </style>

        <div class="topbar">
            <div class="logo">
                "Andrea B."
            </div>
            <div class="contact">
                <ul>
                    {contacts.into_iter()
                        .map(|(name, url)| {
                            view! {
                                <li>
                                    <a
                                        href={url}
                                        target="_blank"
                                        rel="noopener noreferrer"
                                    >
                                        {name}
                                    </a>
                                </li>
                            }
                        })
                        .collect::<Vec<_>>()
                    }
                </ul>
            </div>
        </div>
    }
}

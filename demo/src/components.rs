use dioxus::prelude::*;
use crate::Route;

fn scroll_to_section(section_id: &str) {
    #[cfg(feature = "web")]
    {
        dioxus::document::eval(&format!(
            "document.getElementById('{}')?.scrollIntoView({{ behavior: 'smooth' }})", 
            section_id
        ));
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct CodeBlockProps {
    #[props(optional, default = "rust".to_string())]
    language: String,
    code: String,
}

#[component]
pub fn CodeBlock(props: CodeBlockProps) -> Element {
    rsx! {
        pre {
            class: format!("language-{} p-3 rounded", props.language),
            code {
                class: format!("language-{}", props.language),
                "{props.code}"
            }
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct ExampleSectionProps {
    title: String,
    description: Option<String>,
    code: String,
    children: Element,
}

#[component]
pub fn ExampleSection(props: ExampleSectionProps) -> Element {
    rsx! {
        div {
            class: "mb-5",
            h3 {
                class: "h4 mb-3",
                "{props.title}"
            }
            if let Some(desc) = props.description {
                p {
                    class: "text-muted mb-3",
                    "{desc}"
                }
            }
            div {
                class: "border rounded p-3 mb-3",
                {props.children}
            }
            details {
                class: "mt-3",
                summary {
                    class: "btn btn-outline-secondary btn-sm",
                    "Show Code"
                }
                div {
                    class: "mt-2",
                    CodeBlock {
                        code: props.code
                    }
                }
            }
        }
    }
}

#[component]
pub fn ComponentNav() -> Element {
    rsx! {
        nav {
            class: "navbar navbar-expand-lg navbar-dark bg-primary sticky-top",
            div {
                class: "container",
                Link {
                    to: Route::Home {},
                    class: "navbar-brand",
                    "Dioxus Bootstrap"
                }
                button {
                    class: "navbar-toggler",
                    r#type: "button",
                    "data-bs-toggle": "collapse",
                    "data-bs-target": "#navbarNav",
                    span {
                        class: "navbar-toggler-icon"
                    }
                }
                div {
                    class: "collapse navbar-collapse",
                    id: "navbarNav",
                    ul {
                        class: "navbar-nav me-auto",
                        li {
                            class: "nav-item",
                            Link {
                                to: Route::Showcase {},
                                class: "nav-link",
                                "Showcase"
                            }
                        }
                        li {
                            class: "nav-item",
                            Link {
                                to: Route::LayoutPage {},
                                class: "nav-link",
                                "Layout"
                            }
                        }
                        li {
                            class: "nav-item",
                            Link {
                                to: Route::FormsPage {},
                                class: "nav-link",
                                "Forms"
                            }
                        }
                        li {
                            class: "nav-item",
                            Link {
                                to: Route::UIComponentsPage {},
                                class: "nav-link",
                                "Components"
                            }
                        }
                        li {
                            class: "nav-item",
                            Link {
                                to: Route::UtilitiesPage {},
                                class: "nav-link",
                                "Utilities"
                            }
                        }
                    }
                    div {
                        class: "navbar-nav",
                        ThemeToggle {}
                    }
                }
            }
        }
    }
}

#[component]
pub fn ThemeToggle() -> Element {
    let mut theme_mode = use_signal(|| "auto");
    
    let mut set_theme = move |theme: &'static str| {
        theme_mode.set(theme);
        // Use eval to execute JavaScript to set theme
        #[cfg(feature = "web")]
        {
            if theme == "auto" {
                // For auto mode, detect system preference
                dioxus::document::eval(
                    "
                    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
                    document.documentElement.setAttribute('data-bs-theme', prefersDark ? 'dark' : 'light');
                    "
                );
            } else {
                dioxus::document::eval(&format!(
                    "document.documentElement.setAttribute('data-bs-theme', '{}')", 
                    theme
                ));
            }
        }
    };
    
    // Set initial theme on component mount
    use_effect(move || {
        set_theme("auto");
    });
    
    rsx! {
        div {
            class: "dropdown",
            button {
                class: "btn btn-outline-light dropdown-toggle",
                r#type: "button",
                "data-bs-toggle": "dropdown",
                "Theme: {theme_mode()}"
            }
            ul {
                class: "dropdown-menu",
                li {
                    button {
                        class: "dropdown-item",
                        r#type: "button",
                        onclick: move |_| set_theme("auto"),
                        "Auto"
                    }
                }
                li {
                    button {
                        class: "dropdown-item",
                        r#type: "button",
                        onclick: move |_| set_theme("light"),
                        "Light"
                    }
                }
                li {
                    button {
                        class: "dropdown-item",
                        r#type: "button",
                        onclick: move |_| set_theme("dark"),
                        "Dark"
                    }
                }
            }
        }
    }
}

#[component]
pub fn TableOfContents() -> Element {
    rsx! {
        nav {
            class: "toc-nav position-sticky",
            style: "top: 100px;",
            h6 { "Table of Contents" }
            ul {
                class: "nav nav-pills flex-column",
                li {
                    class: "nav-item",
                    Link {
                        to: Route::Showcase {},
                        class: "nav-link",
                        "📋 All Components"
                    }
                }
                li {
                    class: "nav-item",
                    Link {
                        to: Route::LayoutPage {},
                        class: "nav-link",
                        "Layout Components"
                    }
                    ul {
                        class: "nav nav-pills flex-column ms-3",
                        li { 
                            a { 
                                class: "nav-link text-muted", 
                                href: "/layout#containers",
                                onclick: move |_| scroll_to_section("containers"),
                                "Containers" 
                            } 
                        }
                        li { 
                            a { 
                                class: "nav-link text-muted", 
                                href: "/layout#grid",
                                onclick: move |_| scroll_to_section("grid"),
                                "Grid System" 
                            } 
                        }
                        li { 
                            a { 
                                class: "nav-link text-muted", 
                                href: "/layout#cards",
                                onclick: move |_| scroll_to_section("cards"),
                                "Cards" 
                            } 
                        }
                        li { 
                            a { 
                                class: "nav-link text-muted", 
                                href: "/layout#modals",
                                onclick: move |_| scroll_to_section("modals"),
                                "Modals" 
                            } 
                        }
                        li { 
                            a { 
                                class: "nav-link text-muted", 
                                href: "/layout#accordions",
                                onclick: move |_| scroll_to_section("accordions"),
                                "Accordions" 
                            } 
                        }
                    }
                }
                li {
                    class: "nav-item",
                    Link {
                        to: Route::FormsPage {},
                        class: "nav-link",
                        "Form Components"
                    }
                    ul {
                        class: "nav nav-pills flex-column ms-3",
                        li { 
                            a { 
                                class: "nav-link text-muted", 
                                href: "/forms#inputs",
                                onclick: move |_| scroll_to_section("inputs"),
                                "Inputs" 
                            } 
                        }
                        li { 
                            a { 
                                class: "nav-link text-muted", 
                                href: "/forms#buttons",
                                onclick: move |_| scroll_to_section("buttons"),
                                "Buttons" 
                            } 
                        }
                        li { 
                            a { 
                                class: "nav-link text-muted", 
                                href: "/forms#checkboxes",
                                onclick: move |_| scroll_to_section("checkboxes"),
                                "Checkboxes & Radios" 
                            } 
                        }
                        li { 
                            a { 
                                class: "nav-link text-muted", 
                                href: "/forms#selects",
                                onclick: move |_| scroll_to_section("selects"),
                                "Selects" 
                            } 
                        }
                    }
                }
                li {
                    class: "nav-item",
                    Link {
                        to: Route::UIComponentsPage {},
                        class: "nav-link",
                        "UI Components"
                    }
                    ul {
                        class: "nav nav-pills flex-column ms-3",
                        li { 
                            a { 
                                class: "nav-link text-muted", 
                                href: "/components#alerts",
                                onclick: move |_| scroll_to_section("alerts"),
                                "Alerts" 
                            } 
                        }
                        li { 
                            a { 
                                class: "nav-link text-muted", 
                                href: "/components#badges",
                                onclick: move |_| scroll_to_section("badges"),
                                "Badges" 
                            } 
                        }
                        li { 
                            a { 
                                class: "nav-link text-muted", 
                                href: "/components#breadcrumbs",
                                onclick: move |_| scroll_to_section("breadcrumbs"),
                                "Breadcrumbs" 
                            } 
                        }
                        li { 
                            a { 
                                class: "nav-link text-muted", 
                                href: "/components#dropdowns",
                                onclick: move |_| scroll_to_section("dropdowns"),
                                "Dropdowns" 
                            } 
                        }
                        li { 
                            a { 
                                class: "nav-link text-muted", 
                                href: "/components#listgroups",
                                onclick: move |_| scroll_to_section("listgroups"),
                                "List Groups" 
                            } 
                        }
                        li { 
                            a { 
                                class: "nav-link text-muted", 
                                href: "/components#pagination",
                                onclick: move |_| scroll_to_section("pagination"),
                                "Pagination" 
                            } 
                        }
                        li { 
                            a { 
                                class: "nav-link text-muted", 
                                href: "/components#toasts",
                                onclick: move |_| scroll_to_section("toasts"),
                                "Toasts" 
                            } 
                        }
                    }
                }
                li {
                    class: "nav-item",
                    Link {
                        to: Route::UtilitiesPage {},
                        class: "nav-link",
                        "Utilities"
                    }
                }
            }
        }
    }
}
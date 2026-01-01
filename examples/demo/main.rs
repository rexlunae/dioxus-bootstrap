use dioxus::prelude::*;
use dioxus_bootstrap::*;
use dioxus_router::{Routable, Router};
use dioxus_router::components::{Link, Outlet};

mod components;
mod pages;
mod views;

use components::*;

// const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // document::Link { rel: "icon", href: FAVICON }
        // Prism.js for syntax highlighting with theme-aware styling
        document::Script {
            src: "https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-core.min.js"
        }
        document::Script {
            src: "https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/plugins/autoloader/prism-autoloader.min.js"
        }
        document::Style {
            r##"
            /* Custom Prism theme that respects Bootstrap dark/light mode */
            pre[class*="language-"] {{
                background: var(--bs-secondary-bg) !important;
                color: var(--bs-body-color) !important;
                border: 1px solid var(--bs-border-color) !important;
            }}

            code[class*="language-"] {{
                color: var(--bs-body-color) !important;
            }}

            .token.comment,
            .token.prolog,
            .token.doctype,
            .token.cdata {{
                color: var(--bs-secondary-color) !important;
                font-style: italic;
            }}

            .token.punctuation {{
                color: var(--bs-body-color) !important;
            }}

            .token.property,
            .token.tag,
            .token.constant,
            .token.symbol,
            .token.deleted {{
                color: var(--bs-danger) !important;
            }}

            .token.boolean,
            .token.number {{
                color: var(--bs-warning) !important;
            }}

            .token.selector,
            .token.attr-name,
            .token.string,
            .token.char,
            .token.builtin,
            .token.inserted {{
                color: var(--bs-success) !important;
            }}

            .token.operator,
            .token.entity,
            .token.url,
            .language-css .token.string,
            .style .token.string,
            .token.variable {{
                color: var(--bs-info) !important;
            }}

            .token.atrule,
            .token.attr-value,
            .token.function,
            .token.class-name {{
                color: var(--bs-primary) !important;
            }}

            .token.keyword {{
                color: var(--bs-purple, #6f42c1) !important;
                font-weight: bold;
            }}

            .token.regex,
            .token.important {{
                color: var(--bs-warning) !important;
            }}

            .token.important,
            .token.bold {{
                font-weight: bold;
            }}

            .token.italic {{
                font-style: italic;
            }}
            "##
        }
        GlobalTheme {}
        Router::<Route> {}
    }
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/showcase")]
    Showcase {},
    #[route("/layout")]
    LayoutPage {},
    #[route("/forms")]
    FormsPage {},
    #[route("/components")]
    UIComponentsPage {},
    #[route("/utilities")]
    UtilitiesPage {},
    #[route("/legacy/:id")]
    Legacy { id: i32 },
}

#[component]
fn Layout() -> Element {
    rsx! {
        ComponentNav {}

        Container {
            container_type: ContainerType::ContainerFluid,
            Row {
                Col {
                    md: Some(3),
                    lg: Some(2),
                    class: "d-none d-md-block".to_string(),
                    div {
                        class: "sticky-top pt-3",
                        style: "top: 80px; height: calc(100vh - 80px); overflow-y: auto;",
                        TableOfContents {}
                    }
                }
                Col {
                    md: Some(9),
                    lg: Some(10),
                    main {
                        class: "py-4",
                        Outlet::<Route> {}
                    }
                }
            }
        }

        footer {
            class: "bg-dark text-light py-4 mt-5",
            Container {
                Row {
                    Col {
                        md: Some(6),
                        h5 { "Dioxus Bootstrap 0.7" }
                        p { class: "mb-0", "A comprehensive Bootstrap 5.3 component library for Dioxus 0.7 applications." }
                    }
                    Col {
                        md: Some(6),
                        class: "text-md-end".to_string(),
                        p { class: "mb-0", "Built with ❤️ using Dioxus 0.7 & Bootstrap 5.3" }
                        p { class: "mb-0",
                            a {
                                href: "https://github.com/dioxuslabs/dioxus",
                                class: "text-light",
                                "Dioxus Framework"
                            }
                            " | "
                            a {
                                href: "https://getbootstrap.com",
                                class: "text-light",
                                "Bootstrap"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div {
            // Hero section
            div {
                class: "bg-primary text-white py-5 mb-5 rounded",
                Container {
                    Row {
                        justify_content: Some(JustifyContent::Center),
                        Col {
                            md: Some(8),
                            class: "text-center".to_string(),
                            h1 {
                                class: "display-4 fw-bold mb-3",
                                "🎨 Dioxus Bootstrap Components"
                            }
                            p {
                                class: "lead mb-4",
                                "A comprehensive, type-safe Bootstrap component library for Dioxus 0.7 applications. Build beautiful, responsive UIs with familiar Bootstrap styling and full Rust integration."
                            }
                            div {
                                Link {
                                    to: Route::Showcase {},
                                    class: "btn btn-light btn-lg me-3",
                                    "Get Started"
                                }
                                a {
                                    href: "https://github.com/dioxuslabs/dioxus",
                                    class: "btn btn-outline-light btn-lg",
                                    target: "_blank",
                                    "View on GitHub"
                                }
                            }
                        }
                    }
                }
            }

            // Features section
            Container {
                Row {
                    gutter: Some(4),
                    class: "mb-5".to_string(),
                    Col {
                        md: Some(4),
                        Card {
                            class: "h-100 text-center".to_string(),
                            CardBody {
                                div {
                                    class: "display-1 text-primary mb-3",
                                    "🚀"
                                }
                                CardTitle { "Type-Safe Components" }
                                CardText { "Full Rust type safety with comprehensive prop validation and IntelliSense support." }
                            }
                        }
                    }
                    Col {
                        md: Some(4),
                        Card {
                            class: "h-100 text-center".to_string(),
                            CardBody {
                                div {
                                    class: "display-1 text-success mb-3",
                                    "📱"
                                }
                                CardTitle { "Fully Responsive" }
                                CardText { "Complete Bootstrap 5.3 grid system with responsive breakpoints and utilities." }
                            }
                        }
                    }
                    Col {
                        md: Some(4),
                        Card {
                            class: "h-100 text-center".to_string(),
                            CardBody {
                                div {
                                    class: "display-1 text-warning mb-3",
                                    "⚡"
                                }
                                CardTitle { "High Performance" }
                                CardText { "Optimized for Dioxus with minimal overhead and fast rendering." }
                            }
                        }
                    }
                }

                // Quick start section
                Row {
                    class: "mb-5".to_string(),
                    Col {
                        Card {
                            CardHeader {
                                h4 { class: "mb-0", "Quick Start" }
                            }
                            CardBody {
                                p { "Get started with Dioxus Bootstrap (for Dioxus 0.7) in just a few steps:" }
                                ol {
                                    li {
                                        "Ensure you're using Dioxus 0.7: "
                                        code { "dioxus = \"0.7\"" }
                                    }
                                    li {
                                        "Add the dependency: "
                                        code { "cargo add dioxus-bootstrap" }
                                    }
                                    li {
                                        "Import and use: "
                                        code { "use dioxus_bootstrap::*;" }
                                    }
                                    li { "Add the GlobalTheme component to load Bootstrap assets automatically" }
                                }
                                Alert {
                                    variant: AlertVariant::Info,
                                    class: "mt-3".to_string(),
                                    "💡 The GlobalTheme component automatically loads Bootstrap 5.3 CSS and JS from CDN, handles dark/light mode, and provides theme switching functionality."
                                }
                            }
                        }
                    }
                }

                // Component overview
                h2 { class: "mb-4", "Component Categories" }
                Row {
                    gutter: Some(3),
                    Col {
                        md: Some(6),
                        class: "mb-4".to_string(),
                        Card {
                            CardHeader {
                                h5 { class: "mb-0", "📐 Layout Components" }
                            }
                            CardBody {
                                p { "Responsive layout building blocks:" }
                                ul {
                                    li { "Container (fluid & responsive)" }
                                    li { "Row & Col with full grid system" }
                                    li { "Card with header, body, footer" }
                                    li { "Modal with all variants" }
                                    li { "Accordion for collapsible content" }
                                }
                                Link {
                                    to: Route::LayoutPage {},
                                    class: "btn btn-primary btn-sm",
                                    "View Examples"
                                }
                            }
                        }
                    }
                    Col {
                        md: Some(6),
                        class: "mb-4".to_string(),
                        Card {
                            CardHeader {
                                h5 { class: "mb-0", "📝 Form Components" }
                            }
                            CardBody {
                                p { "Complete form controls:" }
                                ul {
                                    li { "Input (all HTML5 types)" }
                                    li { "Textarea & Select dropdowns" }
                                    li { "Checkbox, Radio & Switches" }
                                    li { "Button with all variants" }
                                    li { "Form validation states" }
                                }
                                Link {
                                    to: Route::FormsPage {},
                                    class: "btn btn-success btn-sm",
                                    "Try Interactive Forms"
                                }
                            }
                        }
                    }
                    Col {
                        md: Some(6),
                        Card {
                            CardHeader {
                                h5 { class: "mb-0", "🎨 UI Components" }
                            }
                            CardBody {
                                p { "Rich interactive elements:" }
                                ul {
                                    li { "Alert with dismissible variants" }
                                    li { "Badge & Breadcrumb navigation" }
                                    li { "Dropdown & List Groups" }
                                    li { "Pagination & Toast notifications" }
                                    li { "Tab system & Navigation" }
                                }
                                Link {
                                    to: Route::UIComponentsPage {},
                                    class: "btn btn-info btn-sm",
                                    "Explore Components"
                                }
                            }
                        }
                    }
                    Col {
                        md: Some(6),
                        Card {
                            CardHeader {
                                h5 { class: "mb-0", "🛠️ Utilities" }
                            }
                            CardBody {
                                p { "Comprehensive utility system:" }
                                ul {
                                    li { "Spacing (margin/padding)" }
                                    li { "Colors & Typography" }
                                    li { "Display & Flexbox" }
                                    li { "Borders & Shadows" }
                                    li { "Position utilities" }
                                }
                                Link {
                                    to: Route::UtilitiesPage {},
                                    class: "btn btn-warning btn-sm",
                                    "See Utilities"
                                }
                            }
                        }
                    }
                }

                // Stats section
                Row {
                    class: "text-center py-5 bg-body-secondary rounded mt-5".to_string(),
                    Col {
                        md: Some(3),
                        h3 { class: "text-primary", "50+" }
                        p { "Components" }
                    }
                    Col {
                        md: Some(3),
                        h3 { class: "text-success", "100%" }
                        p { "Bootstrap Compatible" }
                    }
                    Col {
                        md: Some(3),
                        h3 { class: "text-warning", "Type" }
                        p { "Safe" }
                    }
                    Col {
                        md: Some(3),
                        h3 { class: "text-info", "⚡" }
                        p { "Fast & Reliable" }
                    }
                }
            }
        }
    }
}

#[component]
fn Showcase() -> Element {
    rsx! {
        LayoutPage {}
        FormsPage {}
        UIComponentsPage {}
        UtilitiesPage {}
    }
}

#[component]
fn LayoutPage() -> Element {
    rsx! { pages::LayoutPage {} }
}

#[component]
fn FormsPage() -> Element {
    rsx! { pages::FormsPage {} }
}

#[component]
fn UIComponentsPage() -> Element {
    rsx! { pages::UIComponentsPage {} }
}

#[component]
fn UtilitiesPage() -> Element {
    rsx! { pages::UtilitiesPage {} }
}

#[component]
fn Legacy(id: i32) -> Element {
    let text = format!("Legacy page for item {}", id);
    rsx! {
        Container {
            Alert {
                variant: AlertVariant::Warning,
                "This is the legacy route structure. Use the new showcase for comprehensive examples."
            }
            h1 { "{text}" }
            Link {
                to: Route::Showcase {},
                class: "btn btn-primary",
                "Back to Showcase"
            }
        }
    }
}
use dioxus::prelude::*;

use dioxus_bootstrap::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css",
            integrity: "sha384-QWTKZyjpPEjISv5WaRU9OFeRpok6YctnYmDr5pNlyT2bRjXh0JMhjY6hW+ALEwIH",
            crossorigin: "anonymous"
        }
        document::Script {
            src: "https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js",
            integrity: "sha384-YvpcrYf0tY3lHB60NNkmXc5s9fDVZLESaAA55NDzOxhy9GkcIdslK1eN7N6jIeHz",
            crossorigin: "anonymous"
        }

        Button { variant: ButtonVariant::Secondary, outline: true, "Default" }
        Button { toggle: true, "Toggle Me!" }

        ButtonGroup {
            label: "A test button group.",
            Button { variant: ButtonVariant::Secondary, outline: true, "Default" }
            Button { toggle: true, "Toggle Me!" }
        }
    }
}

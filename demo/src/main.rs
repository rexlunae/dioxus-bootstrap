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
        GlobalTheme { mode: ThemeMode::Dark }
        Container {
            size: ExtendedSize::Fluid,

            Button { variant: ButtonVariant::Secondary, outline: true, "Default" }
            Button { toggle: true, "Toggle Me!" }

            ButtonGroup {
                label: "A test button group.",
                Button { variant: ButtonVariant::Primary, outline: false, "Left" }
                Button { variant: ButtonVariant::Primary, toggle: true, "Right" }
            }
            }
    }
}

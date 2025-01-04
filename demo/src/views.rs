use dioxus::prelude::*;
use dioxus_bootstrap::*;


#[component]
pub fn Page1() -> Element {
    rsx!{
        h1 {"Welcome to Page 1"}
        ButtonGroup {
            label: "A test button group.",
            Button { variant: ButtonVariant::Primary, outline: false, "Left" }
            Button { variant: ButtonVariant::Primary, toggle: true, "Right" }
        }
}
}

#[component]
pub fn Page2(id: i32) -> Element {
    let text = format!("Welcome to Page 2, item {}", id);
    rsx!{
        h1 {{text}}
        Button { variant: ButtonVariant::Secondary, outline: true, "Default" }
        Button { toggle: true, "Toggle Me!" }
}
}

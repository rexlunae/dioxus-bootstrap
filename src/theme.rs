use dioxus::prelude::*;

const DETECTOR: Asset = asset!("/assets/dark_mode_detect.js");

#[derive(Clone, PartialEq)]
pub enum ThemeMode {
    Auto,
    Dark,
    Light,
}

impl From<&str> for ThemeMode {
    fn from(value: &str) -> Self {
        match value {
            "dark" => Self::Dark,
            "light" => Self::Light,
            _ => Self::Auto,
        }
    }
}
impl Into<&str> for ThemeMode {
    fn into(self: Self) -> &'static str {
        match self {
            Self::Dark => "dark",
            Self::Light => "light",
            _ => "auto",
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct ThemeProps {
    #[props(optional, default = ThemeMode::Auto)]
    mode: ThemeMode,
    children: Element,
}

/// Sets dark/light mode based on system setting on the entire html tag.
#[component]
pub fn GlobalTheme() -> Element {
    rsx!{
        document::Script {
            src: DETECTOR
        }
    }
}

/// Overrides the larger mode for child nodes.
#[component]
pub fn LocalTheme(props: ThemeProps) -> Element {
    let theme_requested: &str = props.mode.into();

    rsx!{
        div {
            "data-bs-theme": theme_requested,
            {props.children}
        }
    }
}
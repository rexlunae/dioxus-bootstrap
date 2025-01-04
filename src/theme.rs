use dioxus::prelude::*;

const AUTO: Asset = asset!("/assets/auto_mode.js");
const DARK: Asset = asset!("/assets/dark_mode.js");
const LIGHT: Asset = asset!("/assets/light_mode.js");

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
    /// set to false if you don't want ot load the bootstrap assets from the CDN.
    /// You will need to load them manually.
    #[props(optional, default = true)]
    cdn_load_assets: bool,
}

/// Sets dark/light mode based on system setting on the entire html tag.
/// Also loads the required Bootstrap assets from the Bootstrap CDN. If
/// you want to load those assets some other way, you can disable this
/// with the cdn_load_assets prop.
#[component]
pub fn GlobalTheme(props: ThemeProps) -> Element {
    let mode = match props.mode {
        ThemeMode::Light => LIGHT,
        ThemeMode::Dark => DARK,
        _ => AUTO,
    };

    rsx!{
        if props.cdn_load_assets {
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
        }
        document::Script {
            src: mode
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
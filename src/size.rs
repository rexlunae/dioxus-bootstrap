//! No components here, just a generic size type used to modify components.

/// Standard sized used by Bootstrap.
#[derive(Clone, Copy, Default, PartialEq)]
pub enum Size {
    Small,
    Large,
    #[default]
    Normal,
}

impl Into<&'static str> for Size {
    fn into(self) -> &'static str {
        match self {
            Size::Large => "lg",
            Size::Small => "sm",
            _ => ""
        }
    }
}

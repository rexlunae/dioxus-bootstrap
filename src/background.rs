//! This module contains general class selectors defined by the Bootstrap theme. It does not contain any components.

#[derive(Clone, PartialEq)]
pub enum BackGroundColor {
    None,
    Primary,
    Secondary,
    Tertiary,
}

impl From<BackGroundColor> for &str {
    fn from(value: BackGroundColor) -> &'static str {
        match value {
            BackGroundColor::Primary => "bg-body-primary",
            BackGroundColor::Secondary => "bg-body-secondary",
            BackGroundColor::Tertiary => "bg-body-tertiary",
            _ => "",
        }
    }
}
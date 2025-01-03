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

#[derive(Clone, Copy, Default, PartialEq)]
pub enum ExtendedSize {
    XS,
    Small,
    Medium,
    Large,
    XL,
    XXL,
    #[default]
    Normal,
    Fluid,
}

impl Into<&'static str> for ExtendedSize {
    fn into(self) -> &'static str {
        match self {
            ExtendedSize::XS => "xs",
            ExtendedSize::Small => "sm",
            ExtendedSize::Medium => "md",
            ExtendedSize::Large => "lg",
            ExtendedSize::XL => "xl",
            ExtendedSize::XXL => "xxl",
            ExtendedSize::Fluid => "fluid",
            _ => ""
        }
    }
}

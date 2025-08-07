use dioxus::prelude::*;

// Text utilities
#[derive(Clone, Copy, PartialEq)]
pub enum TextAlign {
    Start,
    Center,
    End,
    Justify,
}

impl Into<&'static str> for TextAlign {
    fn into(self) -> &'static str {
        match self {
            TextAlign::Start => "text-start",
            TextAlign::Center => "text-center",
            TextAlign::End => "text-end",
            TextAlign::Justify => "text-justify",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum TextTransform {
    Lowercase,
    Uppercase,
    Capitalize,
}

impl Into<&'static str> for TextTransform {
    fn into(self) -> &'static str {
        match self {
            TextTransform::Lowercase => "text-lowercase",
            TextTransform::Uppercase => "text-uppercase",
            TextTransform::Capitalize => "text-capitalize",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum FontWeight {
    Bold,
    Bolder,
    SemiBold,
    Medium,
    Normal,
    Light,
    Lighter,
}

impl Into<&'static str> for FontWeight {
    fn into(self) -> &'static str {
        match self {
            FontWeight::Bold => "fw-bold",
            FontWeight::Bolder => "fw-bolder",
            FontWeight::SemiBold => "fw-semibold",
            FontWeight::Medium => "fw-medium",
            FontWeight::Normal => "fw-normal",
            FontWeight::Light => "fw-light",
            FontWeight::Lighter => "fw-lighter",
        }
    }
}

// Display utilities
#[derive(Clone, Copy, PartialEq)]
pub enum Display {
    None,
    Inline,
    InlineBlock,
    Block,
    Grid,
    Table,
    TableCell,
    TableRow,
    Flex,
    InlineFlex,
}

impl Into<&'static str> for Display {
    fn into(self) -> &'static str {
        match self {
            Display::None => "d-none",
            Display::Inline => "d-inline",
            Display::InlineBlock => "d-inline-block",
            Display::Block => "d-block",
            Display::Grid => "d-grid",
            Display::Table => "d-table",
            Display::TableCell => "d-table-cell",
            Display::TableRow => "d-table-row",
            Display::Flex => "d-flex",
            Display::InlineFlex => "d-inline-flex",
        }
    }
}

// Flex utilities
#[derive(Clone, Copy, PartialEq)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

impl Into<&'static str> for FlexDirection {
    fn into(self) -> &'static str {
        match self {
            FlexDirection::Row => "flex-row",
            FlexDirection::RowReverse => "flex-row-reverse",
            FlexDirection::Column => "flex-column",
            FlexDirection::ColumnReverse => "flex-column-reverse",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum FlexWrap {
    Nowrap,
    Wrap,
    WrapReverse,
}

impl Into<&'static str> for FlexWrap {
    fn into(self) -> &'static str {
        match self {
            FlexWrap::Nowrap => "flex-nowrap",
            FlexWrap::Wrap => "flex-wrap",
            FlexWrap::WrapReverse => "flex-wrap-reverse",
        }
    }
}

// Spacing utilities
#[derive(Clone, Copy, PartialEq)]
pub enum SpacingSize {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Auto,
}

impl Into<&'static str> for SpacingSize {
    fn into(self) -> &'static str {
        match self {
            SpacingSize::Zero => "0",
            SpacingSize::One => "1",
            SpacingSize::Two => "2",
            SpacingSize::Three => "3",
            SpacingSize::Four => "4",
            SpacingSize::Five => "5",
            SpacingSize::Auto => "auto",
        }
    }
}

pub fn margin_class(size: SpacingSize) -> String {
    let size_str: &str = size.into();
    format!("m-{}", size_str)
}

pub fn margin_x_class(size: SpacingSize) -> String {
    let size_str: &str = size.into();
    format!("mx-{}", size_str)
}

pub fn margin_y_class(size: SpacingSize) -> String {
    let size_str: &str = size.into();
    format!("my-{}", size_str)
}

pub fn padding_class(size: SpacingSize) -> String {
    let size_str: &str = size.into();
    format!("p-{}", size_str)
}

pub fn padding_x_class(size: SpacingSize) -> String {
    let size_str: &str = size.into();
    format!("px-{}", size_str)
}

pub fn padding_y_class(size: SpacingSize) -> String {
    let size_str: &str = size.into();
    format!("py-{}", size_str)
}

// Color utilities
#[derive(Clone, Copy, PartialEq)]
pub enum TextColor {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
    Body,
    Muted,
    White,
    Black50,
    White50,
}

impl Into<&'static str> for TextColor {
    fn into(self) -> &'static str {
        match self {
            TextColor::Primary => "text-primary",
            TextColor::Secondary => "text-secondary",
            TextColor::Success => "text-success",
            TextColor::Danger => "text-danger",
            TextColor::Warning => "text-warning",
            TextColor::Info => "text-info",
            TextColor::Light => "text-light",
            TextColor::Dark => "text-dark",
            TextColor::Body => "text-body",
            TextColor::Muted => "text-muted",
            TextColor::White => "text-white",
            TextColor::Black50 => "text-black-50",
            TextColor::White50 => "text-white-50",
        }
    }
}

// Border utilities
#[derive(Clone, Copy, PartialEq)]
pub enum BorderRadius {
    None,
    Small,
    Normal,
    Large,
    Circle,
    Pill,
}

impl Into<&'static str> for BorderRadius {
    fn into(self) -> &'static str {
        match self {
            BorderRadius::None => "rounded-0",
            BorderRadius::Small => "rounded-1",
            BorderRadius::Normal => "rounded",
            BorderRadius::Large => "rounded-3",
            BorderRadius::Circle => "rounded-circle",
            BorderRadius::Pill => "rounded-pill",
        }
    }
}

// Position utilities
#[derive(Clone, Copy, PartialEq)]
pub enum Position {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

impl Into<&'static str> for Position {
    fn into(self) -> &'static str {
        match self {
            Position::Static => "position-static",
            Position::Relative => "position-relative",
            Position::Absolute => "position-absolute",
            Position::Fixed => "position-fixed",
            Position::Sticky => "position-sticky",
        }
    }
}

// Shadow utilities
#[derive(Clone, Copy, PartialEq)]
pub enum Shadow {
    None,
    Small,
    Normal,
    Large,
}

impl Into<&'static str> for Shadow {
    fn into(self) -> &'static str {
        match self {
            Shadow::None => "shadow-none",
            Shadow::Small => "shadow-sm",
            Shadow::Normal => "shadow",
            Shadow::Large => "shadow-lg",
        }
    }
}
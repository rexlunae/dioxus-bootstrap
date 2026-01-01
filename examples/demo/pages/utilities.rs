use dioxus::prelude::*;
use dioxus_bootstrap::*;
use crate::components::*;

#[component]
pub fn UtilitiesPage() -> Element {
    rsx! {
        div {
            id: "utilities",
            h2 { class: "mb-4", "Utility Classes" }
            p {
                class: "lead",
                "Bootstrap utility classes for spacing, colors, display, flexbox, and more."
            }
            
            SpacingUtilities {}
            ColorUtilities {}
            DisplayUtilities {}
            FlexUtilities {}
            BorderUtilities {}
            PositionUtilities {}
        }
    }
}

#[component]
pub fn SpacingUtilities() -> Element {
    rsx! {
        ExampleSection {
            title: "Spacing Utilities".to_string(),
            description: Some("Use margin and padding utility functions for consistent spacing.".to_string()),
            code: r#"// Utility functions for spacing
margin_class(SpacingSize::Three)        // "m-3" 
padding_class(SpacingSize::Four)        // "p-4"
margin_x_class(SpacingSize::Auto)       // "mx-auto"
margin_y_class(SpacingSize::Two)        // "my-2"
padding_x_class(SpacingSize::Five)      // "px-5"
padding_y_class(SpacingSize::One)       // "py-1"

// Usage in components
div { 
    class: format!("{} {}", margin_class(SpacingSize::Three), padding_class(SpacingSize::Two)),
    "Content with spacing"
}"#.to_string(),
            
            div {
                div { class: "mb-4",
                    h6 { "Margin Examples" }
                    div { 
                        class: format!("{} bg-body-secondary border", margin_class(SpacingSize::Three)),
                        "m-3: margin on all sides"
                    }
                    div { 
                        class: format!("{} bg-body-secondary border mt-2", margin_x_class(SpacingSize::Auto)),
                        "mx-auto: centered horizontally"
                    }
                }
                
                div { class: "mb-4",
                    h6 { "Padding Examples" }
                    div { 
                        class: format!("{} bg-primary text-white", padding_class(SpacingSize::Three)),
                        "p-3: padding on all sides"
                    }
                    div { 
                        class: format!("{} bg-success text-white mt-2", padding_y_class(SpacingSize::Four)),
                        "py-4: vertical padding only"
                    }
                }
                
                div {
                    h6 { "Available Spacing Sizes" }
                    div { class: "d-flex flex-wrap gap-2",
                        for (size, label) in [
                            (SpacingSize::Zero, "0"),
                            (SpacingSize::One, "1"), 
                            (SpacingSize::Two, "2"),
                            (SpacingSize::Three, "3"),
                            (SpacingSize::Four, "4"),
                            (SpacingSize::Five, "5"),
                            (SpacingSize::Auto, "auto")
                        ] {
                            Badge { 
                                variant: BadgeVariant::Secondary,
                                "{label}: {margin_class(size)}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component] 
pub fn ColorUtilities() -> Element {
    rsx! {
        ExampleSection {
            title: "Color Utilities".to_string(),
            description: Some("Text and background color utilities for semantic color schemes.".to_string()),
            code: r#"// Text colors
TextColor::Primary.into()    // "text-primary"
TextColor::Success.into()    // "text-success" 
TextColor::Danger.into()     // "text-danger"
TextColor::Muted.into()      // "text-muted"

// Usage
p { class: TextColor::Primary.into(), "Primary text" }
p { class: TextColor::Success.into(), "Success text" }"#.to_string(),
            
            div {
                div { class: "mb-4",
                    h6 { "Text Colors" }
                    div { class: "row g-2",
                        for (color, name) in [
                            (TextColor::Primary, "Primary"),
                            (TextColor::Secondary, "Secondary"), 
                            (TextColor::Success, "Success"),
                            (TextColor::Danger, "Danger"),
                            (TextColor::Warning, "Warning"),
                            (TextColor::Info, "Info"),
                            (TextColor::Light, "Light"),
                            (TextColor::Dark, "Dark"),
                            (TextColor::Muted, "Muted"),
                            (TextColor::Body, "Body")
                        ] {
                            div { class: "col-md-6",
                                p { 
                                    class: match color {
                                        TextColor::Primary => "text-primary",
                                        TextColor::Secondary => "text-secondary",
                                        TextColor::Success => "text-success",
                                        TextColor::Danger => "text-danger",
                                        TextColor::Warning => "text-warning",
                                        TextColor::Info => "text-info",
                                        TextColor::Light => "text-light",
                                        TextColor::Dark => "text-dark",
                                        TextColor::Muted => "text-muted",
                                        TextColor::Body => "text-body",
                                        _ => "text-body",
                                    },
                                    "{name} text color"
                                }
                            }
                        }
                    }
                }
                
                div { class: "mb-4",
                    h6 { "Background Colors (via BackGroundColor)" }
                    div { class: "row g-2",
                        for (bg, name) in [
                            (BackGroundColor::Primary, "Primary"),
                            (BackGroundColor::Secondary, "Secondary"),
                            (BackGroundColor::Tertiary, "Tertiary")
                        ] {
                            div { class: "col-md-6",
                                div { 
                                    class: {
                                        let bg_class = match bg {
                                            BackGroundColor::None => "",
                                            BackGroundColor::Primary => "bg-body-primary",
                                            BackGroundColor::Secondary => "bg-body-secondary",
                                            BackGroundColor::Tertiary => "bg-body-tertiary",
                                        };
                                        let text_class = "text-body";
                                        format!("{} {} p-2 rounded", bg_class, text_class)
                                    },
                                    "{name} background"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn DisplayUtilities() -> Element {
    rsx! {
        ExampleSection {
            title: "Display Utilities".to_string(),
            description: Some("Control the display property of elements.".to_string()),
            code: r#"Display::None.into()         // "d-none"
Display::Block.into()        // "d-block" 
Display::Flex.into()         // "d-flex"
Display::InlineFlex.into()   // "d-inline-flex"
Display::Grid.into()         // "d-grid"

// Usage
div { class: Display::Flex.into(), "Flex container" }
div { class: Display::None.into(), "Hidden element" }"#.to_string(),
            
            div {
                h6 { "Display Values" }
                div { class: "row g-3",
                    for (display, name, description) in [
                        (Display::Block, "Block", "Takes full width"),
                        (Display::Inline, "Inline", "Inline with text flow"),
                        (Display::InlineBlock, "Inline Block", "Inline but accepts width/height"),
                        (Display::Flex, "Flex", "Flexible box layout"),
                        (Display::InlineFlex, "Inline Flex", "Inline flexible box"),
                        (Display::Grid, "Grid", "CSS Grid container"),
                        (Display::None, "None", "Hidden from view")
                    ] {
                        div { class: "col-md-6",
                            Badge { 
                                variant: BadgeVariant::Light,
                                class: "me-2".to_string(),
                                {match display {
                                    Display::Block => "d-block",
                                    Display::InlineBlock => "d-inline-block", 
                                    Display::Inline => "d-inline",
                                    Display::Flex => "d-flex",
                                    Display::InlineFlex => "d-inline-flex",
                                    Display::Grid => "d-grid",
                                    Display::Table => "d-table",
                                    Display::TableRow => "d-table-row",
                                    Display::TableCell => "d-table-cell",
                                    Display::None => "d-none",
                                }}
                            }
                            strong { "{name}: " }
                            span { class: "text-muted", "{description}" }
                        }
                    }
                }
                
                div { class: "mt-4",
                    h6 { "Flex Display Example" }
                    div { 
                        class: format!("{} gap-2", Into::<&str>::into(Display::Flex)),
                        div { class: "p-2 bg-primary text-white rounded", "Flex item 1" }
                        div { class: "p-2 bg-secondary text-white rounded", "Flex item 2" }  
                        div { class: "p-2 bg-success text-white rounded", "Flex item 3" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn FlexUtilities() -> Element {
    rsx! {
        ExampleSection {
            title: "Flexbox Utilities".to_string(),
            description: Some("Control flexbox direction, wrapping, and alignment.".to_string()),
            code: r#"FlexDirection::Row.into()        // "flex-row"
FlexDirection::Column.into()     // "flex-column"
FlexWrap::Wrap.into()           // "flex-wrap"
FlexWrap::Nowrap.into()         // "flex-nowrap"

// Usage with other components  
Row { 
    justify_content: Some(JustifyContent::Center),
    align_items: Some(AlignItems::Center),
    // content
}"#.to_string(),
            
            div {
                div { class: "mb-4",
                    h6 { "Flex Direction" }
                    div { class: "mb-2",
                        strong { "Row (default): " }
                        div { 
                            class: {
                                let flex: &str = Display::Flex.into();
                                let row: &str = FlexDirection::Row.into();
                                format!("{} {} gap-2", flex, row)
                            },
                            div { class: "p-2 bg-info text-white rounded", "1" }
                            div { class: "p-2 bg-info text-white rounded", "2" }
                            div { class: "p-2 bg-info text-white rounded", "3" }
                        }
                    }
                    div {
                        strong { "Column: " }
                        div { 
                            class: {
                                let flex: &str = Display::Flex.into();
                                let column: &str = FlexDirection::Column.into();
                                format!("{} {} gap-2", flex, column)
                            },
                            style: "width: 100px;",
                            div { class: "p-2 bg-warning text-dark rounded", "1" }
                            div { class: "p-2 bg-warning text-dark rounded", "2" }
                            div { class: "p-2 bg-warning text-dark rounded", "3" }
                        }
                    }
                }
                
                div { class: "mb-4",
                    h6 { "Flex Wrap" }
                    div { class: "mb-2",
                        strong { "Wrap: " }
                        div { 
                            class: {
                                let flex: &str = Display::Flex.into();
                                let wrap: &str = FlexWrap::Wrap.into();
                                format!("{} {} gap-2", flex, wrap)
                            },
                            style: "width: 200px;",
                            for i in 1..=8 {
                                div { 
                                    class: "p-2 bg-success text-white rounded",
                                    style: "min-width: 60px;",
                                    "{i}"
                                }
                            }
                        }
                    }
                }
                
                div {
                    h6 { "Flex Utilities Reference" }
                    div { class: "table-responsive",
                        table { class: "table table-sm",
                            thead {
                                tr {
                                    th { "Utility" }
                                    th { "Class" }
                                    th { "Description" }
                                }
                            }
                            tbody {
                                tr {
                                    td { "FlexDirection::Row" }
                                    td { code { "flex-row" } }
                                    td { "Horizontal direction" }
                                }
                                tr {
                                    td { "FlexDirection::Column" }
                                    td { code { "flex-column" } }
                                    td { "Vertical direction" }
                                }
                                tr {
                                    td { "FlexWrap::Wrap" }
                                    td { code { "flex-wrap" } }
                                    td { "Allow wrapping" }
                                }
                                tr {
                                    td { "FlexWrap::Nowrap" }
                                    td { code { "flex-nowrap" } }
                                    td { "Prevent wrapping" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component] 
pub fn BorderUtilities() -> Element {
    rsx! {
        ExampleSection {
            title: "Border Utilities".to_string(),
            description: Some("Control border radius and rounded corners.".to_string()),
            code: r#"BorderRadius::Normal.into()     // "rounded"
BorderRadius::Small.into()      // "rounded-1" 
BorderRadius::Large.into()      // "rounded-3"
BorderRadius::Pill.into()       // "rounded-pill"
BorderRadius::Circle.into()     // "rounded-circle"

// Usage
div { class: BorderRadius::Pill.into(), "Pill shaped" }"#.to_string(),
            
            div {
                h6 { "Border Radius Examples" }
                div { class: "row g-3",
                    for (radius, name, description) in [
                        (BorderRadius::None, "None", "No rounding"),
                        (BorderRadius::Small, "Small", "Small rounded corners"),
                        (BorderRadius::Normal, "Normal", "Default rounded corners"),
                        (BorderRadius::Large, "Large", "Large rounded corners"), 
                        (BorderRadius::Pill, "Pill", "Fully rounded ends"),
                        (BorderRadius::Circle, "Circle", "Perfect circle")
                    ] {
                        div { class: "col-md-4",
                            div { 
                                class: {
                                    let radius_class = match radius {
                                        BorderRadius::None => "rounded-0",
                                        BorderRadius::Small => "rounded-1",
                                        BorderRadius::Normal => "rounded",
                                        BorderRadius::Large => "rounded-3",
                                        BorderRadius::Circle => "rounded-circle",
                                        BorderRadius::Pill => "rounded-pill",
                                    };
                                    format!("p-3 bg-body-secondary border text-center {}", radius_class)
                                },
                                strong { "{name}" }
                                br {}
                                small { class: "text-muted", 
                                    {match radius {
                                        BorderRadius::None => "rounded-0",
                                        BorderRadius::Small => "rounded-1",
                                        BorderRadius::Normal => "rounded",
                                        BorderRadius::Large => "rounded-3",
                                        BorderRadius::Circle => "rounded-circle",
                                        BorderRadius::Pill => "rounded-pill",
                                    }}
                                }
                                br {}
                                span { class: "small", "{description}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn PositionUtilities() -> Element {
    rsx! {
        ExampleSection {
            title: "Position & Shadow Utilities".to_string(),
            description: Some("Control positioning and shadow effects.".to_string()),
            code: r#"Position::Relative.into()    // "position-relative"
Position::Absolute.into()    // "position-absolute"
Position::Fixed.into()       // "position-fixed"  
Position::Sticky.into()      // "position-sticky"

Shadow::Normal.into()        // "shadow"
Shadow::Small.into()         // "shadow-sm"
Shadow::Large.into()         // "shadow-lg"
Shadow::None.into()          // "shadow-none""#.to_string(),
            
            div {
                div { class: "mb-4",
                    h6 { "Position Values" }
                    div { class: "row g-2",
                        for (pos, name) in [
                            (Position::Static, "Static"),
                            (Position::Relative, "Relative"),
                            (Position::Absolute, "Absolute"),
                            (Position::Fixed, "Fixed"),
                            (Position::Sticky, "Sticky")
                        ] {
                            div { class: "col-md-6",
                                Badge { 
                                    variant: BadgeVariant::Info,
                                    class: "me-2".to_string(),
                                    {match pos {
                                        Position::Static => "position-static",
                                        Position::Relative => "position-relative", 
                                        Position::Absolute => "position-absolute",
                                        Position::Fixed => "position-fixed",
                                        Position::Sticky => "position-sticky",
                                    }}
                                }
                                span { "{name} positioning" }
                            }
                        }
                    }
                }
                
                div {
                    h6 { "Shadow Examples" }
                    div { class: "row g-3",
                        for (shadow, name) in [
                            (Shadow::None, "No Shadow"),
                            (Shadow::Small, "Small Shadow"),
                            (Shadow::Normal, "Normal Shadow"),
                            (Shadow::Large, "Large Shadow")
                        ] {
                            div { class: "col-md-3",
                                div { 
                                    class: {
                                        let shadow_class = match shadow {
                                            Shadow::None => "shadow-none",
                                            Shadow::Small => "shadow-sm",
                                            Shadow::Normal => "shadow",
                                            Shadow::Large => "shadow-lg",
                                        };
                                        format!("p-3 bg-white text-center {}", shadow_class)
                                    },
                                    strong { "{name}" }
                                    br {}
                                    small { class: "text-muted",
                                        {match shadow {
                                            Shadow::None => "shadow-none",
                                            Shadow::Small => "shadow-sm",
                                            Shadow::Normal => "shadow",
                                            Shadow::Large => "shadow-lg",
                                        }}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
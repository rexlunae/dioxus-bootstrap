use dioxus::prelude::*;
use super::size::*;
use dioxus_router::navigation::NavigationTarget;
use dioxus_router::components::Link;

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Basic,
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
    Link,
}

impl Into<&'static str> for ButtonVariant {
    fn into(self) -> &'static str {
        match self {
            ButtonVariant::Primary => "primary",
            ButtonVariant::Secondary => "secondary",
            ButtonVariant::Success => "success",
            ButtonVariant::Danger => "danger",
            ButtonVariant::Warning => "warning",
            ButtonVariant::Info => "info",
            ButtonVariant::Light => "light",
            ButtonVariant::Dark => "dark",
            ButtonVariant::Link => "link",
            _ => "",
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq)]
pub enum ButtonType {
    #[default]
    Button,
    Reset,
    Submit,
}

impl Into<&'static str> for ButtonType {
    fn into(self) -> &'static str {
        match self {
            ButtonType::Button => "button",
            ButtonType::Reset => "reset",
            ButtonType::Submit => "submit",
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct ButtonProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = ButtonVariant::Basic)]
    variant: ButtonVariant,
    #[props(optional, default = Size::Normal)]
    size: Size,
    #[props(optional, default = false)]
    disabled: bool,
    #[props(optional, default = false)]
    outline: bool,
    #[props(optional, default = false)]
    nowrap: bool,
    #[props(optional, default = false)]
    toggle: bool,
    /// active controls whether a toggle button is on or off.
    #[props(optional, default = false)]
    active: bool,
    #[props(optional, default = "".to_string())]
    style: String,
    #[props(optional, default = ButtonType::Button)]
    button_type: ButtonType,
    
    /// Additional CSS classes
    #[props(optional, default = "".to_string())]
    class: String,
    
    /// Button text content (alternative to children)
    #[props(optional, default = None)]
    text: Option<String>,
    
    /// Loading state
    #[props(optional, default = false)]
    loading: bool,
    
    /// Close button variant
    #[props(optional, default = false)]
    close: bool,
    
    /// Floating action button
    #[props(optional, default = false)]
    floating: bool,

    /// If present, generate the button as an 'a' tag using the dioxus router.
    #[props(optional, default = None)]
    link_to: Option<NavigationTarget>,

    children: Element,
    #[props(optional)]
    onclick: EventHandler<MouseEvent>,
    #[props(optional)]
    onmounted: EventHandler<MountedEvent>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let mut class_list = vec!["btn".to_string()];

    if props.disabled { class_list.push("btn-disabled".into()) }
    if props.toggle && props.active { class_list.push("active".into()) }

    let variant: &str = props.variant.into();
    if variant.len() > 0 {
        if props.outline {
            class_list.push(format!("btn-outline-{}", variant))
        } else {
            class_list.push(format!("btn-{}", variant))
        }
    }

    let size: &str = props.size.into();
    if props.size != Size::Normal {
        class_list.push(format!("btn-{}", size));
    }

    // Add additional classes
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }
    
    if props.loading {
        class_list.push("btn-loading".to_string());
    }
    
    if props.floating {
        class_list.push("btn-floating".to_string());
    }
    
    let class_list = class_list.join(" ");

    if props.toggle {
        return rsx! {
            button { id: props.id, r#type: "button", style: props.style, onclick: props.onclick, class: class_list, "data-bs-toggle": "button", "aria-pressed": true, onmounted: props.onmounted, {props.children} }
        }
    }
    
    // Handle close button
    if props.close {
        return rsx! {
            button {
                id: props.id,
                r#type: "button",
                class: "btn-close",
                style: props.style,
                onclick: props.onclick,
                disabled: props.disabled,
                "aria-label": "Close",
                onmounted: props.onmounted,
            }
        };
    }
    
    let button_type_str: &str = props.button_type.into();
    
    match props.button_type {
        ButtonType::Submit | ButtonType::Reset => rsx!{
            input { 
                id: props.id, 
                r#type: button_type_str, 
                value: if props.button_type == ButtonType::Submit { "Submit" } else { "Reset" }, 
                style: props.style, 
                onclick: props.onclick, 
                class: class_list, 
                disabled: props.disabled, 
                onmounted: props.onmounted 
            }
        },
        _ => match props.link_to {
            Some(t) => rsx!{
                Link {
                    to: t,
                    id: props.id,
                    onclick: props.onclick,
                    style: props.style,
                    class: class_list,
                    "aria-disabled": props.disabled,
                    {if let Some(text) = props.text { rsx! { "{text}" } } else { props.children }}
                }
            },
            _ => rsx! {
                button { 
                    id: props.id, 
                    r#type: button_type_str, 
                    style: props.style, 
                    onclick: props.onclick, 
                    class: class_list, 
                    disabled: props.disabled, 
                    onmounted: props.onmounted, 
                    {if let Some(text) = props.text { rsx! { "{text}" } } else { props.children }}
                }
            }
        }
    }

}

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonGroupOrientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Props, PartialEq)]
pub struct ButtonGroupProps {
    /// The label generates an aria-label attribute for screen readers.
    #[props(optional, default = "Button group".to_string())]
    label: String,
    #[props(optional, default = Size::Normal)]
    size: Size,
    #[props(optional, default = ButtonGroupOrientation::Horizontal)]
    orientation: ButtonGroupOrientation,
    #[props(optional, default = false)]
    toolbar: bool,
    children: Element,
}

#[component]
pub fn ButtonGroup(props: ButtonGroupProps) -> Element {
    let mut class_list = vec!["btn-group".to_string()];
    
    if props.orientation == ButtonGroupOrientation::Vertical {
        class_list = vec!["btn-group-vertical".to_string()];
    }
    
    let size: &str = props.size.into();
    if props.size != Size::Normal {
        class_list.push(format!("btn-group-{}", size));
    }
    
    if props.toolbar {
        class_list = vec!["btn-toolbar".to_string()];
    }

    let class_list = class_list.join(" ");
    let role = if props.toolbar { "toolbar" } else { "group" };
    
    rsx! {
        div {
            class: class_list,
            role: role,
            "aria-label": props.label,
            {props.children}
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct DropdownButtonProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    #[props(optional, default = ButtonVariant::Primary)]
    variant: ButtonVariant,
    #[props(optional, default = Size::Normal)]
    size: Size,
    #[props(optional, default = false)]
    disabled: bool,
    #[props(optional, default = false)]
    outline: bool,
    #[props(optional, default = false)]
    split: bool,
    #[props(optional, default = "Dropdown".to_string())]
    text: String,
    children: Element,
}

#[component]
pub fn DropdownButton(props: DropdownButtonProps) -> Element {
    let mut class_list = vec!["btn".to_string(), "dropdown-toggle".to_string()];
    
    let variant: &str = props.variant.into();
    if !variant.is_empty() {
        if props.outline {
            class_list.push(format!("btn-outline-{}", variant));
        } else {
            class_list.push(format!("btn-{}", variant));
        }
    }
    
    let size: &str = props.size.into();
    if props.size != Size::Normal {
        class_list.push(format!("btn-{}", size));
    }
    
    // Add additional classes
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }
    
    let class_list = class_list.join(" ");
    
    if props.split {
        rsx! {
            div {
                class: "btn-group",
                button {
                    r#type: "button",
                    class: class_list.replace("dropdown-toggle", "").trim(),
                    disabled: props.disabled,
                    "{props.text}"
                }
                button {
                    r#type: "button",
                    class: format!("{} dropdown-toggle dropdown-toggle-split", class_list.replace("dropdown-toggle", "").trim()),
                    "data-bs-toggle": "dropdown",
                    "aria-expanded": "false",
                    disabled: props.disabled,
                    span {
                        class: "visually-hidden",
                        "Toggle Dropdown"
                    }
                }
                ul {
                    class: "dropdown-menu",
                    {props.children}
                }
            }
        }
    } else {
        rsx! {
            div {
                class: "dropdown",
                button {
                    id: props.id,
                    r#type: "button",
                    class: class_list,
                    "data-bs-toggle": "dropdown",
                    "aria-expanded": "false",
                    disabled: props.disabled,
                    "{props.text}"
                }
                ul {
                    class: "dropdown-menu",
                    {props.children}
                }
            }
        }
    }
}

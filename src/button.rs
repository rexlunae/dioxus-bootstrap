use dioxus::prelude::*;
use super::size::*;

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
pub enum ButtonValues {
    #[default]
    None,
    Reset,
    Submit,
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
    #[props(optional, default = ButtonValues::None)]
    value: ButtonValues,
    #[props(optional)]
    children: Element,
    #[props(optional)]
    onclick: EventHandler<MouseEvent>,
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

    let class_list = class_list.join(" ");

    if props.toggle {
        return rsx! {
            button { id: props.id, type: "button", style: props.style, onclick: props.onclick, class: class_list, "data-bs-toggle": "button", "aria-pressed": true, {props.children} }
        }

    }

    match props.value {
        ButtonValues::Submit => rsx!{
            input { id: props.id, type: "submit", value: "Submit", style: props.style, onclick: props.onclick, class: class_list, "aria-disabled": props.disabled,  {props.children} }
        },
        ButtonValues::Reset => rsx!{
            input { id: props.id, type: "reset", value: "Reset", style: props.style, onclick: props.onclick, class: class_list, "aria-disabled": props.disabled, {props.children} }
        },
        _ => rsx! {
            button { id: props.id, type: "button", style: props.style, onclick: props.onclick, class: class_list, "aria-disabled": props.disabled, {props.children} }
        }
    }

}

#[derive(Clone, Props, PartialEq)]
pub struct ButtonGroupProps {
    /// The label generates an aria-label attribute for screen readers.
    label: String,
    #[props(optional, default = Size::Normal)]
    size: Size,
    children: Element,
}

#[component]
pub fn ButtonGroup(props: ButtonGroupProps) -> Element {
    let class_list = vec!["btn-group".to_string()];

    let class_list = class_list.join(" ");
    rsx! {
        div {
            class: class_list,
            role: "group",
            "aria-label": props.label,
            role: "group",
            {props.children}
        }
    }
}
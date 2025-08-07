use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum AlertVariant {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
}

impl Into<&'static str> for AlertVariant {
    fn into(self) -> &'static str {
        match self {
            AlertVariant::Primary => "alert-primary",
            AlertVariant::Secondary => "alert-secondary",
            AlertVariant::Success => "alert-success",
            AlertVariant::Danger => "alert-danger",
            AlertVariant::Warning => "alert-warning",
            AlertVariant::Info => "alert-info",
            AlertVariant::Light => "alert-light",
            AlertVariant::Dark => "alert-dark",
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct AlertProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    #[props(optional, default = AlertVariant::Primary)]
    variant: AlertVariant,
    #[props(optional, default = false)]
    dismissible: bool,
    #[props(optional, default = false)]
    fade: bool,
    #[props(optional, default = false)]
    show: bool,
    children: Element,
}

#[component]
pub fn Alert(props: AlertProps) -> Element {
    let mut class_list = vec!["alert".to_string()];
    
    let variant_class: &str = props.variant.into();
    class_list.push(variant_class.to_string());
    
    if props.dismissible {
        class_list.push("alert-dismissible".to_string());
    }
    
    if props.fade {
        class_list.push("fade".to_string());
    }
    
    if props.show {
        class_list.push("show".to_string());
    }
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }
    
    let class_list = class_list.join(" ");
    
    rsx! {
        div {
            id: props.id,
            class: class_list,
            role: "alert",
            {props.children}
            if props.dismissible {
                button {
                    r#type: "button",
                    class: "btn-close",
                    "data-bs-dismiss": "alert",
                    "aria-label": "Close",
                }
            }
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct AlertHeadingProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    #[props(optional, default = "h4".to_string())]
    tag: String,
    children: Element,
}

#[component]
pub fn AlertHeading(props: AlertHeadingProps) -> Element {
    let mut class_list = vec!["alert-heading".to_string()];
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }
    
    let class_list = class_list.join(" ");
    
    match props.tag.as_str() {
        "h1" => rsx! { h1 { id: props.id, class: class_list, {props.children} } },
        "h2" => rsx! { h2 { id: props.id, class: class_list, {props.children} } },
        "h3" => rsx! { h3 { id: props.id, class: class_list, {props.children} } },
        "h5" => rsx! { h5 { id: props.id, class: class_list, {props.children} } },
        "h6" => rsx! { h6 { id: props.id, class: class_list, {props.children} } },
        _ => rsx! { h4 { id: props.id, class: class_list, {props.children} } },
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct AlertLinkProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    #[props(optional, default = "".to_string())]
    href: String,
    children: Element,
}

#[component]
pub fn AlertLink(props: AlertLinkProps) -> Element {
    let mut class_list = vec!["alert-link".to_string()];
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }
    
    let class_list = class_list.join(" ");
    
    rsx! {
        a {
            id: props.id,
            class: class_list,
            href: props.href,
            {props.children}
        }
    }
}
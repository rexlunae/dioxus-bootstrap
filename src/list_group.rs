use dioxus::prelude::*;
use dioxus_router::navigation::NavigationTarget;
use dioxus_router::components::Link;

#[derive(Clone, Props, PartialEq)]
pub struct ListGroupProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    #[props(optional, default = false)]
    flush: bool,
    #[props(optional, default = false)]
    horizontal: bool,
    #[props(optional, default = false)]
    numbered: bool,
    children: Element,
}

#[component]
pub fn ListGroup(props: ListGroupProps) -> Element {
    let mut class_list = vec!["list-group".to_string()];
    
    if props.flush {
        class_list.push("list-group-flush".to_string());
    }
    
    if props.horizontal {
        class_list.push("list-group-horizontal".to_string());
    }
    
    if props.numbered {
        class_list.push("list-group-numbered".to_string());
    }
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }
    
    let class_list = class_list.join(" ");
    
    if props.numbered {
        rsx! {
            ol {
                id: props.id,
                class: class_list,
                {props.children}
            }
        }
    } else {
        rsx! {
            ul {
                id: props.id,
                class: class_list,
                {props.children}
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ListGroupItemVariant {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
}

impl Into<&'static str> for ListGroupItemVariant {
    fn into(self) -> &'static str {
        match self {
            ListGroupItemVariant::Primary => "list-group-item-primary",
            ListGroupItemVariant::Secondary => "list-group-item-secondary",
            ListGroupItemVariant::Success => "list-group-item-success",
            ListGroupItemVariant::Danger => "list-group-item-danger",
            ListGroupItemVariant::Warning => "list-group-item-warning",
            ListGroupItemVariant::Info => "list-group-item-info",
            ListGroupItemVariant::Light => "list-group-item-light",
            ListGroupItemVariant::Dark => "list-group-item-dark",
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct ListGroupItemProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    #[props(optional, default = false)]
    active: bool,
    #[props(optional, default = false)]
    disabled: bool,
    #[props(optional, default = None)]
    variant: Option<ListGroupItemVariant>,
    #[props(optional)]
    href: Option<String>,
    #[props(optional)]
    link_to: Option<NavigationTarget>,
    #[props(optional)]
    onclick: EventHandler<MouseEvent>,
    children: Element,
}

#[component]
pub fn ListGroupItem(props: ListGroupItemProps) -> Element {
    let mut class_list = vec!["list-group-item".to_string()];
    
    if props.active {
        class_list.push("active".to_string());
    }
    
    if props.disabled {
        class_list.push("disabled".to_string());
    }
    
    if let Some(variant) = props.variant {
        let variant_class: &str = variant.into();
        class_list.push(variant_class.to_string());
    }
    
    // Add action class if it's clickable
    if props.href.is_some() || props.link_to.is_some() {
        class_list.push("list-group-item-action".to_string());
    }
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }
    
    let class_list = class_list.join(" ");
    
    match (props.link_to, props.href) {
        (Some(to), _) if !props.disabled => rsx! {
            Link {
                id: props.id,
                class: class_list,
                to: to,
                onclick: props.onclick,
                {props.children}
            }
        },
        (None, Some(href)) if !props.disabled => rsx! {
            a {
                id: props.id,
                class: class_list,
                href: href,
                onclick: props.onclick,
                {props.children}
            }
        },
        _ => rsx! {
            li {
                id: props.id,
                class: class_list,
                onclick: props.onclick,
                {props.children}
            }
        }
    }
}
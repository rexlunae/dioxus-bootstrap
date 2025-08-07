use dioxus::prelude::*;
use dioxus_router::navigation::NavigationTarget;
use dioxus_router::components::Link;
use super::size::Size;

#[derive(Clone, Props, PartialEq)]
pub struct PaginationProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    #[props(optional, default = Size::Normal)]
    size: Size,
    children: Element,
}

#[component]
pub fn Pagination(props: PaginationProps) -> Element {
    let mut class_list = vec!["pagination".to_string()];
    
    let size: &str = props.size.into();
    if props.size != Size::Normal {
        class_list.push(format!("pagination-{}", size));
    }
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }
    
    let class_list = class_list.join(" ");
    
    rsx! {
        nav {
            id: props.id,
            "aria-label": "Page navigation",
            ul {
                class: class_list,
                {props.children}
            }
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct PaginationItemProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    #[props(optional, default = false)]
    active: bool,
    #[props(optional, default = false)]
    disabled: bool,
    #[props(optional)]
    href: Option<String>,
    #[props(optional)]
    link_to: Option<NavigationTarget>,
    #[props(optional)]
    onclick: EventHandler<MouseEvent>,
    children: Element,
}

#[component]
pub fn PaginationItem(props: PaginationItemProps) -> Element {
    let mut class_list = vec!["page-item".to_string()];
    
    if props.active {
        class_list.push("active".to_string());
    }
    
    if props.disabled {
        class_list.push("disabled".to_string());
    }
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }
    
    let class_list = class_list.join(" ");
    
    rsx! {
        li {
            id: props.id,
            class: class_list,
            match props.link_to {
                Some(to) if !props.disabled => rsx! {
                    Link {
                        to: to,
                        class: "page-link",
                        onclick: props.onclick,
                        {props.children}
                    }
                },
                None if !props.disabled => rsx! {
                    a {
                        class: "page-link",
                        href: props.href.unwrap_or("#".to_string()),
                        onclick: props.onclick,
                        {props.children}
                    }
                },
                _ => rsx! {
                    span {
                        class: "page-link",
                        {props.children}
                    }
                }
            }
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct PaginationLinkProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    #[props(optional, default = false)]
    disabled: bool,
    #[props(optional)]
    href: Option<String>,
    #[props(optional)]
    link_to: Option<NavigationTarget>,
    #[props(optional)]
    onclick: EventHandler<MouseEvent>,
    children: Element,
}

#[component]
pub fn PaginationLink(props: PaginationLinkProps) -> Element {
    let mut class_list = vec!["page-link".to_string()];
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }
    
    let class_list = class_list.join(" ");
    
    if props.disabled {
        rsx! {
            span {
                id: props.id,
                class: class_list,
                {props.children}
            }
        }
    } else {
        match props.link_to {
            Some(to) => rsx! {
                Link {
                    id: props.id,
                    class: class_list,
                    to: to,
                    onclick: props.onclick,
                    {props.children}
                }
            },
            None => rsx! {
                a {
                    id: props.id,
                    class: class_list,
                    href: props.href.unwrap_or("#".to_string()),
                    onclick: props.onclick,
                    {props.children}
                }
            }
        }
    }
}
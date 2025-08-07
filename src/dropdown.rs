use dioxus::prelude::*;
use dioxus_router::navigation::NavigationTarget;
use dioxus_router::components::Link;

#[derive(Clone, Props, PartialEq)]
pub struct DropdownProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    children: Element,
}

#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    let mut class_list = vec!["dropdown".to_string()];
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }
    
    let class_list = class_list.join(" ");
    
    rsx! {
        div {
            id: props.id,
            class: class_list,
            {props.children}
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct DropdownMenuProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    #[props(optional, default = false)]
    dark: bool,
    children: Element,
}

#[component]
pub fn DropdownMenu(props: DropdownMenuProps) -> Element {
    let mut class_list = vec!["dropdown-menu".to_string()];
    
    if props.dark {
        class_list.push("dropdown-menu-dark".to_string());
    }
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }
    
    let class_list = class_list.join(" ");
    
    rsx! {
        ul {
            id: props.id,
            class: class_list,
            {props.children}
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct DropdownItemProps {
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
pub fn DropdownItem(props: DropdownItemProps) -> Element {
    let mut class_list = vec!["dropdown-item".to_string()];
    
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
            match props.link_to {
                Some(to) if !props.disabled => rsx! {
                    Link {
                        id: props.id,
                        class: class_list,
                        to: to,
                        onclick: props.onclick,
                        {props.children}
                    }
                },
                None if !props.disabled => rsx! {
                    a {
                        id: props.id,
                        class: class_list,
                        href: props.href.unwrap_or("#".to_string()),
                        onclick: props.onclick,
                        {props.children}
                    }
                },
                _ => rsx! {
                    span {
                        id: props.id,
                        class: class_list,
                        {props.children}
                    }
                }
            }
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct DropdownHeaderProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    children: Element,
}

#[component]
pub fn DropdownHeader(props: DropdownHeaderProps) -> Element {
    let mut class_list = vec!["dropdown-header".to_string()];
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }
    
    let class_list = class_list.join(" ");
    
    rsx! {
        li {
            h6 {
                id: props.id,
                class: class_list,
                {props.children}
            }
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct DropdownDividerProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
}

#[component]
pub fn DropdownDivider(props: DropdownDividerProps) -> Element {
    let mut class_list = vec!["dropdown-divider".to_string()];
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }
    
    let class_list = class_list.join(" ");
    
    rsx! {
        li {
            hr {
                id: props.id,
                class: class_list,
            }
        }
    }
}
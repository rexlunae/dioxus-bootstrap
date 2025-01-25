use dioxus::prelude::*;
use super::size::*;
use dioxus_router::navigation::NavigationTarget;
use dioxus_router::components::Link;

#[derive(Clone, Copy, PartialEq)]
pub enum TabSetVarient {
    /// Tabs look like links, without decoration
    Basic,

    /// Tabs with a basic tab outline.
    Tabs,

    /// Pill-shaped tabs
    Pills,

    /// Tabs are underlined.
    Underline,
}


#[derive(Clone, Props, PartialEq)]
pub struct TabSetProps {
    #[props(optional)]
    id: String,
    children: Element,
    #[props(optional, default = TabSetVarient::Basic)]
    varient: TabSetVarient,
    #[props(optional, default = false)]
    verticle: bool,
    #[props(optional, default = false)]
    fill: bool,
}

#[component]
pub fn TabSet(props: TabSetProps) -> Element {
    let mut class_list = String::from("nav");

    if props.verticle {
        class_list.push_str(" flex-column");
    }

    match props.varient {
        TabSetVarient::Tabs => {
            class_list.push_str(" nav-tabs");
        },
        TabSetVarient::Pills => {
            class_list.push_str(" nav-pills");
        },
        TabSetVarient::Underline => {
            class_list.push_str(" nav-underline");
        },
        _ => {}
    }

    if props.fill {
        class_list.push_str(" nav-fill");
    }

    rsx! {
        ul {
            class: class_list,
            {props.children}
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct TabProps {
    #[props(optional)]
    id: String,
    children: Element,
    #[props(optional, default = false)]
    active: bool,
    #[props(optional, default = None)]
    link_to: Option<NavigationTarget>,
    #[props(optional, default = false)]
    dropdown: bool,
    #[props(optional, default = false)]
    disabled: bool,
    #[props(optional)]
    onclick: EventHandler<MouseEvent>,
    #[props(optional)]
    onmounted: EventHandler<MountedEvent>,  
    #[props(optional, default = String::from(""))]
    style: String,
}

#[component]
pub fn Tab(props: TabProps) -> Element {
    let mut class_list = String::from("nav-item");
    if props.active {
        class_list.push_str(" active");
    }
    if props.dropdown {
        class_list.push_str(" dropdown");
    }
    
    rsx! {
        li {
            class: class_list,
            if props.dropdown {
                ul {
                    class: "dropdown-menu",
                    {props.children}
                }
            } else {
                match props.link_to {
                    Some(link) => {
                        rsx! {
                            Link {
                                to: link,
                                id: props.id,
                                onclick: props.onclick,
                                style: props.style,
                                class: "nav-link",
                                aria_disabled: props.disabled,
                                {props.children}
                            }
                        }
                    }
                    None => {
                        rsx! {
                            a {
                                class: "nav-link",
                                style: props.style,
                                aria_disabled: props.disabled,
                                onclick: props.onclick,
                                id: props.id,
                                {props.children}
                            }
                        }
                    }
                }
            }
        }
    }
}
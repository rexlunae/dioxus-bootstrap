use dioxus::prelude::*;
use super::background::*;
use dioxus_router::{
    navigation::NavigationTarget,
    components::*,
};

#[derive(Clone, Props, PartialEq)]
pub struct SidebarProps {
    #[props(optional)]
    id: String,
    children: Element,
    header: Option<Element>,
    footer: Option<Element>,
    #[props(optional, default = BackGroundColor::Tertiary)]
    background_color: BackGroundColor,
    #[props(optional, default = 0)]
    flex_shrink: u8,
    #[props(optional, default = 3)]
    p: u8,
}

#[component]
pub fn SideBar(props: SidebarProps) -> Element {
    let background_color: &str = props.background_color.into();
    let class_list = vec!["sidebar".to_string(), "flex-column".to_string(), background_color.to_string(), format!("flex-shrink-{}", props.flex_shrink), format!("p-{}", 0)];

    let class_list = class_list.join(" ");
    rsx!{
        aside {
            class: class_list,
            id: props.id,
            match props.header {
                Some(h) => rsx!{
                    {h},
                    hr{}
                },
                None => rsx!{}
            }
            {props.children}
            match props.footer {
                Some(f) => rsx!{
                    hr{}
                    {f},
                },
                None => rsx!{}
            }
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct SidebarMenuProps {
    #[props(optional)]
    id: String,
    children: Element,
    #[props(optional, default = true)]
    pills: bool,
    #[props(optional, default = true)]
    auto_size: bool,
}

#[component]
pub fn SideBarMenu(props: SidebarMenuProps) -> Element {
    let mut class_list = vec!["nav".to_string(), "flex-column".to_string()];

    if props.pills {class_list.push("nav-pills".to_string())}
    if props.auto_size {class_list.push("mb-auto".to_string())}

    let class_list = class_list.join(" ");
    rsx! {
        ul {
            class: class_list,
            {props.children}
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct SidebarMenuItemProps {
    #[props(optional)]
    id: String,
    children: Element,
    #[props(optional, default = false)]
    active: bool,
    /// Uses the Dioxus Router for the menu item.
    #[props(optional, default = None)]
    link_to: Option<NavigationTarget>,
    #[props(optional)]
    onclick: EventHandler<MouseEvent>,
    #[props(optional)]
    onmounted: EventHandler<MountedEvent>,
    #[props(optional, default = "".to_string())]
    style: String,
}

#[component]
pub fn SideBarMenuItem(props: SidebarMenuItemProps) -> Element {
    let mut class_list = vec!["nav-link".to_string()];

    if props.active {class_list.push("active".to_string())}
    else {class_list.push("link-body-emphasis".to_string())}

    let class_list = class_list.join(" ");
    rsx!{
        li {
            class: "nav-item",
            match props.link_to {
                Some(t) => rsx!{
                    Link {
                    to: t,
                    id: props.id,
                    onclick: props.onclick,
                    style: props.style,
                    class: class_list,
                    {props.children}
                }},
                None => rsx! {
                    a {
                    id: props.id,
                    href: "#",
                    onclick: props.onclick,
                    style: props.style,
                    class: class_list,
                    {props.children}
                }},
            }
        }
    }
}
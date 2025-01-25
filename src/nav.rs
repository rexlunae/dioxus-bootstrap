use dioxus::prelude::*;
use super::size::*;
use dioxus_router::navigation::NavigationTarget;
use dioxus_router::components::Link;

#[derive(Clone, Copy, PartialEq)]
pub enum NavBarVariant {
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

use super::size::Size;
use super::background::BackGroundColor;

#[derive(Clone, Props, PartialEq)]
pub struct NavBarProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = Size::Large)]
    size: Size,
    #[props(optional, default = BackGroundColor::Secondary)]
    background_color: BackGroundColor,
    children: Element,
    #[props(optional, default = Some("Navbar".to_string()))]
    brand: Option<String>,
    #[props(optional, default = None)]
    brand_image: Option<Asset>,
    #[props(optional, default = None)]
    brand_link_to: Option<NavigationTarget>,
    #[props(optional, default = None)]
    toggler_id: Option<String>,

}

#[component]
pub fn NavBar(props: NavBarProps) -> Element {
    let size: &str = props.size.into();

    let mut class_list = String::new();
    class_list.push_str("navbar");
    class_list.push_str(" ");
    class_list.push_str(format!("navbar-expand-{}", size).as_str());
    class_list.push_str(" ");
    class_list.push_str("navbar-light");
    class_list.push_str(" ");
    class_list.push_str(props.background_color.into());

    let img_element = match props.brand_image {
        Some(img) => rsx! {
            img {
                src: img,
                alt: props.brand.clone().unwrap_or("logo".to_string()),
                width: "30",
                height: "24",
                class: "d-inline-block align-text-top",
            }
        },
        None => rsx!{},
    };

    rsx! {
        nav {
            class: class_list,
            div {
                class: "container-fluid",
                match props.brand {
                    Some(brand) => match props.brand_link_to {
                        Some(link_to) => rsx! {
                            Link {
                                to: link_to.clone(),
                                class: "navbar-brand",
                                {img_element}
                                "{brand}"
                            }
                        },
                        None => rsx! {
                            a {
                                href: "#",
                                class: "navbar-brand",
                                {img_element}
                                "{brand}"
                            }
                        },
                    },
                    None => rsx!{},
                }
            }
            match props.toggler_id {
                Some(toggler_id) => rsx! {
                    button {
                        class: "navbar-toggler",
                        type: "button",
                        r#"data-bs-toggle"#: "collapse",
                        r#"data-bs-target"#: format!("#{}", toggler_id).as_str(),
                        aria_label: "toggle",
                        aria_controls: {toggler_id.clone()},
                        aria_expanded: "false",
                        span {
                            class: "navbar-toggler-icon",
                        }         
                    }
                    div {
                        class: "collapse navbar-collapse",
                        id: toggler_id,
                        {props.children}
                    }
                },
                None => props.children,
            }
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct NavBarItemProps {
    #[props(optional)]
    id: String,
    #[props(optional)]
    to: Option<NavigationTarget>,
    #[props(optional)]
    href: Option<String>,
    #[props(optional)]
    onclick: EventHandler<MouseEvent>,
    #[props(optional)]
    onmounted: EventHandler<MountedEvent>,
}
#[derive(Clone, Props, PartialEq)]
pub struct NavBarCollapseProps {
    #[props(optional)]
    id: String,
}
#[derive(Clone, Props, PartialEq)]
pub struct NavBarContentProps {
    #[props(optional)]
    id: String,
}


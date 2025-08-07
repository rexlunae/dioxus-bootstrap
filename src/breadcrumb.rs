use dioxus::prelude::*;
use dioxus_router::navigation::NavigationTarget;
use dioxus_router::components::Link;

#[derive(Clone, Props, PartialEq)]
pub struct BreadcrumbProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    #[props(optional, default = "/".to_string())]
    divider: String,
    children: Element,
}

#[component]
pub fn Breadcrumb(props: BreadcrumbProps) -> Element {
    let mut class_list = vec!["breadcrumb".to_string()];
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }
    
    let class_list = class_list.join(" ");
    let style = if props.divider != "/" {
        format!("--bs-breadcrumb-divider: '{}';", props.divider)
    } else {
        String::new()
    };
    
    rsx! {
        nav {
            id: props.id,
            "aria-label": "breadcrumb",
            ol {
                class: class_list,
                style: style,
                {props.children}
            }
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct BreadcrumbItemProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    #[props(optional, default = false)]
    active: bool,
    #[props(optional)]
    href: Option<String>,
    #[props(optional)]
    link_to: Option<NavigationTarget>,
    children: Element,
}

#[component]
pub fn BreadcrumbItem(props: BreadcrumbItemProps) -> Element {
    let mut class_list = vec!["breadcrumb-item".to_string()];
    
    if props.active {
        class_list.push("active".to_string());
    }
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }
    
    let class_list = class_list.join(" ");
    
    if props.active {
        rsx! {
            li {
                id: props.id,
                class: class_list,
                "aria-current": "page",
                {props.children}
            }
        }
    } else {
        rsx! {
            li {
                id: props.id,
                class: class_list,
                match props.link_to {
                    Some(to) => rsx! {
                        Link {
                            to: to,
                            {props.children}
                        }
                    },
                    None => rsx! {
                        a {
                            href: props.href.unwrap_or("#".to_string()),
                            {props.children}
                        }
                    }
                }
            }
        }
    }
}
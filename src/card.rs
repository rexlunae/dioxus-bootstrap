use dioxus::prelude::*;
use dioxus_router::navigation::NavigationTarget;
use dioxus_router::components::Link;

#[derive(Clone, Props, PartialEq)]
pub struct CardProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    children: Element,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    let mut class_list = vec!["card".to_string()];
    
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
pub struct CardHeaderProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    children: Element,
}

#[component]
pub fn CardHeader(props: CardHeaderProps) -> Element {
    let mut class_list = vec!["card-header".to_string()];
    
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
pub struct CardBodyProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    children: Element,
}

#[component]
pub fn CardBody(props: CardBodyProps) -> Element {
    let mut class_list = vec!["card-body".to_string()];
    
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
pub struct CardFooterProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    children: Element,
}

#[component]
pub fn CardFooter(props: CardFooterProps) -> Element {
    let mut class_list = vec!["card-footer".to_string()];
    
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
pub struct CardTitleProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    #[props(optional, default = "h5".to_string())]
    tag: String,
    children: Element,
}

#[component]
pub fn CardTitle(props: CardTitleProps) -> Element {
    let mut class_list = vec!["card-title".to_string()];
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }
    
    let class_list = class_list.join(" ");
    
    match props.tag.as_str() {
        "h1" => rsx! { h1 { id: props.id, class: class_list, {props.children} } },
        "h2" => rsx! { h2 { id: props.id, class: class_list, {props.children} } },
        "h3" => rsx! { h3 { id: props.id, class: class_list, {props.children} } },
        "h4" => rsx! { h4 { id: props.id, class: class_list, {props.children} } },
        "h6" => rsx! { h6 { id: props.id, class: class_list, {props.children} } },
        _ => rsx! { h5 { id: props.id, class: class_list, {props.children} } },
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct CardTextProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    children: Element,
}

#[component]
pub fn CardText(props: CardTextProps) -> Element {
    let mut class_list = vec!["card-text".to_string()];
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }
    
    let class_list = class_list.join(" ");
    
    rsx! {
        p {
            id: props.id,
            class: class_list,
            {props.children}
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct CardImageProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    #[props(optional, default = false)]
    top: bool,
    #[props(optional, default = false)]
    bottom: bool,
    src: String,
    #[props(optional, default = "".to_string())]
    alt: String,
}

#[component]
pub fn CardImage(props: CardImageProps) -> Element {
    let mut class_list = if props.top {
        vec!["card-img-top".to_string()]
    } else if props.bottom {
        vec!["card-img-bottom".to_string()]
    } else {
        vec!["card-img".to_string()]
    };
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }
    
    let class_list = class_list.join(" ");
    
    rsx! {
        img {
            id: props.id,
            class: class_list,
            src: props.src,
            alt: props.alt,
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct CardLinkProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    #[props(optional)]
    href: Option<String>,
    #[props(optional)]
    link_to: Option<NavigationTarget>,
    children: Element,
}

#[component]
pub fn CardLink(props: CardLinkProps) -> Element {
    let mut class_list = vec!["card-link".to_string()];
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }
    
    let class_list = class_list.join(" ");
    
    match props.link_to {
        Some(to) => rsx! {
            Link {
                id: props.id,
                class: class_list,
                to: to,
                {props.children}
            }
        },
        None => rsx! {
            a {
                id: props.id,
                class: class_list,
                href: props.href.unwrap_or_default(),
                {props.children}
            }
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct CardGroupProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    children: Element,
}

#[component]
pub fn CardGroup(props: CardGroupProps) -> Element {
    let mut class_list = vec!["card-group".to_string()];
    
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
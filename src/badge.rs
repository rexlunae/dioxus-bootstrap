use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum BadgeVariant {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
}

impl Into<&'static str> for BadgeVariant {
    fn into(self) -> &'static str {
        match self {
            BadgeVariant::Primary => "text-bg-primary",
            BadgeVariant::Secondary => "text-bg-secondary",
            BadgeVariant::Success => "text-bg-success",
            BadgeVariant::Danger => "text-bg-danger",
            BadgeVariant::Warning => "text-bg-warning",
            BadgeVariant::Info => "text-bg-info",
            BadgeVariant::Light => "text-bg-light",
            BadgeVariant::Dark => "text-bg-dark",
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct BadgeProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    #[props(optional, default = BadgeVariant::Primary)]
    variant: BadgeVariant,
    #[props(optional, default = false)]
    pill: bool,
    children: Element,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let mut class_list = vec!["badge".to_string()];
    
    let variant_class: &str = props.variant.into();
    class_list.push(variant_class.to_string());
    
    if props.pill {
        class_list.push("rounded-pill".to_string());
    }
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }
    
    let class_list = class_list.join(" ");
    
    rsx! {
        span {
            id: props.id,
            class: class_list,
            {props.children}
        }
    }
}
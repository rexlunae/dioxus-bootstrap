use dioxus::prelude::*;
use super::size::*;

#[derive(Clone, Props, PartialEq)]
pub struct ContainerProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = ExtendedSize::Normal)]
    size: ExtendedSize,
    #[props(optional, default = false)]
    text_center: bool,
    #[props(optional, default = None)]
    max_width: Option<u16>,
    children: Element,
}

#[component]
pub fn Container(props: ContainerProps) -> Element {
    let mut base_class_name = "container".to_string();
    let size_str: &str = props.size.into();

    if size_str.len() > 0 {
        base_class_name = format!("{}-{}", base_class_name, size_str);
    }

    let mut class_list = vec![base_class_name];

    if props.text_center {
        class_list.push("text-center".to_string());
    }

    let class_list = class_list.join(" ");
    rsx! {
        div {
            class: class_list,
            {props.children}
        }
    }
}

#[component]
pub fn Row() -> Element {
    rsx!{
        div { class: "row" }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct ColProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = 0)]
    span: u8,
    #[props(optional, default = ExtendedSize::Normal)]
    size: ExtendedSize,
}

#[component]
pub fn Col(props: ColProps) -> Element {
    let mut class = "col".to_string();
    if props.span > 0 {
        if props.size != ExtendedSize::Normal {
            let size: &str = props.size.into();
            class = format!("{}-{}-{}", class, size, props.span)
        }
        class = format!("{}-{}", class, props.span)
    }

    rsx!{
        div { class: class }
    }
}

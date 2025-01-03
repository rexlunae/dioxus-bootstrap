use dioxus::prelude::*;
use super::size::*;

#[derive(Clone, Props, PartialEq)]
pub struct ContainerProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = ExtendedSize::Normal)]
    size: ExtendedSize,
    children: Element,
}

#[component]
pub fn Container(props: ContainerProps) -> Element {
    let mut class = "container".to_string();
    let size_str: &str = props.size.into();

    if size_str.len() > 0 {
        class = format!("{}-{}", class, size_str);
    }

    rsx! {
        div {
            class: class,
            {props.children}
        }
    }
}
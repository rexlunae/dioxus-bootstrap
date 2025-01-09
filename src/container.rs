use dioxus::prelude::*;
//use crate::FlexBoxClassSet;

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

#[derive(Clone, Props, PartialEq)]
pub struct RowProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = false)]
    no_wrap: bool,
    //#[props(optional, default = FlexBoxClassSet::new())]
    //flexbox: super::FlexBoxClassSet<RowProps>,
    children: Element,
}

//impl super::FlexBoxContainerProps for RowProps {
//    fn prefix() -> &'static str {"row"}
//}

#[component]
pub fn Row(props: RowProps) -> Element {
    let mut class_list = vec!["row".to_string()];

    if props.no_wrap {
        class_list.push("d-flex flex-nowrap".to_string())
    }

    let class_list = class_list.join(" ");
    rsx!{
        div {
            id: props.id,
            class: class_list,
            {props.children}
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct ColProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = 0)]
    span: u8,
    #[props(optional, default = false)]
    no_wrap: bool,
    #[props(optional, default = ExtendedSize::Normal)]
    size: ExtendedSize,
    //#[props(optional, default = FlexBoxClassSet::new())]
    //flexbox: super::FlexBoxClassSet<ColProps>,
    children: Element,
}

//impl super::FlexBoxContainerProps for ColProps {
//    fn prefix() -> &'static str {"col"}
//}

#[component]
pub fn Col(props: ColProps) -> Element {
    let mut class_list = vec!["col".to_string()];

    if props.no_wrap {
        class_list.push("d-flex flex-nowrap".to_string())
    }

    if props.span > 0 {
        if props.size != ExtendedSize::Normal {
            let size: &str = props.size.into();
            class_list.push(format!("col-{}-{}", size, props.span))
        }
        else { class_list.push(format!("col-{}", props.span)) }
    }

    let class_list = class_list.join(" ");
    rsx!{
        div { class: class_list, {props.children} }
    }
}

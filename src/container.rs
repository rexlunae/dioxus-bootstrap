use dioxus::prelude::*;
//use crate::FlexBoxClassSet;

use super::size::*;

#[derive(Clone, Copy, PartialEq)]
pub enum ContainerType {
    Container,
    ContainerFluid,
}

#[derive(Clone, Props, PartialEq)]
pub struct ContainerProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = ExtendedSize::Normal)]
    size: ExtendedSize,
    #[props(optional, default = ContainerType::Container)]
    container_type: ContainerType,
    #[props(optional, default = false)]
    text_center: bool,
    #[props(optional, default = None)]
    max_width: Option<u16>,
    #[props(optional, default = "".to_string())]
    class: String,
    children: Element,
}

#[component]
pub fn Container(props: ContainerProps) -> Element {
    let mut class_list = match props.container_type {
        ContainerType::ContainerFluid => vec!["container-fluid".to_string()],
        ContainerType::Container => {
            let size_str: &str = props.size.into();
            if size_str.is_empty() {
                vec!["container".to_string()]
            } else {
                vec![format!("container-{}", size_str)]
            }
        }
    };

    if props.text_center {
        class_list.push("text-center".to_string());
    }
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }

    let class_list = class_list.join(" ");
    let style = props.max_width.map(|w| format!("max-width: {}px;", w));
    
    rsx! {
        div {
            id: props.id,
            class: class_list,
            style: style.unwrap_or_default(),
            {props.children}
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum AlignItems {
    Start,
    Center,
    End,
    Baseline,
    Stretch,
}

impl Into<&'static str> for AlignItems {
    fn into(self) -> &'static str {
        match self {
            AlignItems::Start => "align-items-start",
            AlignItems::Center => "align-items-center",
            AlignItems::End => "align-items-end",
            AlignItems::Baseline => "align-items-baseline",
            AlignItems::Stretch => "align-items-stretch",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum JustifyContent {
    Start,
    Center,
    End,
    Around,
    Between,
    Evenly,
}

impl Into<&'static str> for JustifyContent {
    fn into(self) -> &'static str {
        match self {
            JustifyContent::Start => "justify-content-start",
            JustifyContent::Center => "justify-content-center",
            JustifyContent::End => "justify-content-end",
            JustifyContent::Around => "justify-content-around",
            JustifyContent::Between => "justify-content-between",
            JustifyContent::Evenly => "justify-content-evenly",
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct RowProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = false)]
    no_wrap: bool,
    #[props(optional, default = None)]
    align_items: Option<AlignItems>,
    #[props(optional, default = None)]
    justify_content: Option<JustifyContent>,
    #[props(optional, default = None)]
    gutter: Option<u8>,
    #[props(optional, default = "".to_string())]
    class: String,
    children: Element,
}

//impl super::FlexBoxContainerProps for RowProps {
//    fn prefix() -> &'static str {"row"}
//}

#[component]
pub fn Row(props: RowProps) -> Element {
    let mut class_list = vec!["row".to_string()];

    if props.no_wrap {
        class_list.push("flex-nowrap".to_string());
    }
    
    if let Some(align) = props.align_items {
        let align_str: &str = align.into();
        class_list.push(align_str.to_string());
    }
    
    if let Some(justify) = props.justify_content {
        let justify_str: &str = justify.into();
        class_list.push(justify_str.to_string());
    }
    
    if let Some(g) = props.gutter {
        class_list.push(format!("g-{}", g));
    }
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
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
    /// Column width (1-12)
    #[props(optional, default = None)]
    span: Option<u8>,
    /// Auto-sizing column
    #[props(optional, default = false)]
    auto: bool,
    /// Breakpoint-specific spans
    #[props(optional, default = None)]
    xs: Option<u8>,
    #[props(optional, default = None)]
    sm: Option<u8>,
    #[props(optional, default = None)]
    md: Option<u8>,
    #[props(optional, default = None)]
    lg: Option<u8>,
    #[props(optional, default = None)]
    xl: Option<u8>,
    #[props(optional, default = None)]
    xxl: Option<u8>,
    /// Offset columns
    #[props(optional, default = None)]
    offset: Option<u8>,
    #[props(optional, default = None)]
    offset_sm: Option<u8>,
    #[props(optional, default = None)]
    offset_md: Option<u8>,
    #[props(optional, default = None)]
    offset_lg: Option<u8>,
    #[props(optional, default = None)]
    offset_xl: Option<u8>,
    #[props(optional, default = None)]
    offset_xxl: Option<u8>,
    /// Order
    #[props(optional, default = None)]
    order: Option<u8>,
    #[props(optional, default = false)]
    no_wrap: bool,
    #[props(optional, default = "".to_string())]
    class: String,
    children: Element,
}

//impl super::FlexBoxContainerProps for ColProps {
//    fn prefix() -> &'static str {"col"}
//}

#[component]
pub fn Col(props: ColProps) -> Element {
    let mut class_list = vec![];
    
    // Base column class
    if props.auto {
        class_list.push("col-auto".to_string());
    } else if let Some(span) = props.span {
        class_list.push(format!("col-{}", span));
    } else {
        class_list.push("col".to_string());
    }
    
    // Responsive breakpoint classes
    if let Some(xs) = props.xs {
        class_list.push(format!("col-{}", xs));
    }
    if let Some(sm) = props.sm {
        class_list.push(format!("col-sm-{}", sm));
    }
    if let Some(md) = props.md {
        class_list.push(format!("col-md-{}", md));
    }
    if let Some(lg) = props.lg {
        class_list.push(format!("col-lg-{}", lg));
    }
    if let Some(xl) = props.xl {
        class_list.push(format!("col-xl-{}", xl));
    }
    if let Some(xxl) = props.xxl {
        class_list.push(format!("col-xxl-{}", xxl));
    }
    
    // Offset classes
    if let Some(offset) = props.offset {
        class_list.push(format!("offset-{}", offset));
    }
    if let Some(offset_sm) = props.offset_sm {
        class_list.push(format!("offset-sm-{}", offset_sm));
    }
    if let Some(offset_md) = props.offset_md {
        class_list.push(format!("offset-md-{}", offset_md));
    }
    if let Some(offset_lg) = props.offset_lg {
        class_list.push(format!("offset-lg-{}", offset_lg));
    }
    if let Some(offset_xl) = props.offset_xl {
        class_list.push(format!("offset-xl-{}", offset_xl));
    }
    if let Some(offset_xxl) = props.offset_xxl {
        class_list.push(format!("offset-xxl-{}", offset_xxl));
    }
    
    // Order
    if let Some(order) = props.order {
        class_list.push(format!("order-{}", order));
    }

    if props.no_wrap {
        class_list.push("flex-nowrap".to_string());
    }
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
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

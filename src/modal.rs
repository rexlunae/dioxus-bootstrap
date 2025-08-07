use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum ModalSize {
    Small,
    Default,
    Large,
    ExtraLarge,
}

impl Into<&'static str> for ModalSize {
    fn into(self) -> &'static str {
        match self {
            ModalSize::Small => "modal-sm",
            ModalSize::Default => "",
            ModalSize::Large => "modal-lg",
            ModalSize::ExtraLarge => "modal-xl",
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct ModalProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    #[props(optional, default = ModalSize::Default)]
    size: ModalSize,
    #[props(optional, default = false)]
    fade: bool,
    #[props(optional, default = false)]
    centered: bool,
    #[props(optional, default = false)]
    scrollable: bool,
    #[props(optional, default = false)]
    fullscreen: bool,
    #[props(optional, default = false)]
    static_backdrop: bool,
    children: Element,
}

#[component]
pub fn Modal(props: ModalProps) -> Element {
    let mut modal_classes = vec!["modal".to_string()];
    let mut dialog_classes = vec!["modal-dialog".to_string()];
    
    if props.fade {
        modal_classes.push("fade".to_string());
    }
    
    if props.centered {
        dialog_classes.push("modal-dialog-centered".to_string());
    }
    
    if props.scrollable {
        dialog_classes.push("modal-dialog-scrollable".to_string());
    }
    
    if props.fullscreen {
        dialog_classes.push("modal-fullscreen".to_string());
    } else {
        let size_class: &str = props.size.into();
        if !size_class.is_empty() {
            dialog_classes.push(size_class.to_string());
        }
    }
    
    if !props.class.is_empty() {
        modal_classes.push(props.class.clone());
    }
    
    let modal_classes = modal_classes.join(" ");
    let dialog_classes = dialog_classes.join(" ");
    let backdrop_attr = if props.static_backdrop { "static" } else { "true" };
    
    rsx! {
        div {
            id: props.id,
            class: modal_classes,
            tabindex: "-1",
            role: "dialog",
            "data-bs-backdrop": backdrop_attr,
            "data-bs-keyboard": "true",
            div {
                class: dialog_classes,
                role: "document",
                div {
                    class: "modal-content",
                    {props.children}
                }
            }
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct ModalHeaderProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    #[props(optional, default = true)]
    close_button: bool,
    children: Element,
}

#[component]
pub fn ModalHeader(props: ModalHeaderProps) -> Element {
    let mut class_list = vec!["modal-header".to_string()];
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }
    
    let class_list = class_list.join(" ");
    
    rsx! {
        div {
            id: props.id,
            class: class_list,
            {props.children}
            if props.close_button {
                button {
                    r#type: "button",
                    class: "btn-close",
                    "data-bs-dismiss": "modal",
                    "aria-label": "Close",
                }
            }
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct ModalBodyProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    children: Element,
}

#[component]
pub fn ModalBody(props: ModalBodyProps) -> Element {
    let mut class_list = vec!["modal-body".to_string()];
    
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
pub struct ModalFooterProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    children: Element,
}

#[component]
pub fn ModalFooter(props: ModalFooterProps) -> Element {
    let mut class_list = vec!["modal-footer".to_string()];
    
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
pub struct ModalTitleProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    #[props(optional, default = "h1".to_string())]
    tag: String,
    children: Element,
}

#[component]
pub fn ModalTitle(props: ModalTitleProps) -> Element {
    let mut class_list = vec!["modal-title".to_string()];
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }
    
    let class_list = class_list.join(" ");
    
    match props.tag.as_str() {
        "h2" => rsx! { h2 { id: props.id, class: class_list, {props.children} } },
        "h3" => rsx! { h3 { id: props.id, class: class_list, {props.children} } },
        "h4" => rsx! { h4 { id: props.id, class: class_list, {props.children} } },
        "h5" => rsx! { h5 { id: props.id, class: class_list, {props.children} } },
        "h6" => rsx! { h6 { id: props.id, class: class_list, {props.children} } },
        _ => rsx! { h1 { id: props.id, class: class_list, {props.children} } },
    }
}
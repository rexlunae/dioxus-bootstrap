use dioxus::prelude::*;

#[derive(Clone, Props, PartialEq)]
pub struct ToastProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    #[props(optional, default = false)]
    fade: bool,
    #[props(optional, default = false)]
    show: bool,
    children: Element,
}

#[component]
pub fn Toast(props: ToastProps) -> Element {
    let mut class_list = vec!["toast".to_string()];
    
    if props.fade {
        class_list.push("fade".to_string());
    }
    
    if props.show {
        class_list.push("show".to_string());
    }
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }
    
    let class_list = class_list.join(" ");
    
    rsx! {
        div {
            id: props.id,
            class: class_list,
            role: "alert",
            "aria-live": "assertive",
            "aria-atomic": "true",
            {props.children}
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct ToastHeaderProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    #[props(optional, default = true)]
    close_button: bool,
    children: Element,
}

#[component]
pub fn ToastHeader(props: ToastHeaderProps) -> Element {
    let mut class_list = vec!["toast-header".to_string()];
    
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
                    "data-bs-dismiss": "toast",
                    "aria-label": "Close",
                }
            }
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct ToastBodyProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    children: Element,
}

#[component]
pub fn ToastBody(props: ToastBodyProps) -> Element {
    let mut class_list = vec!["toast-body".to_string()];
    
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
pub struct ToastContainerProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    children: Element,
}

#[component]
pub fn ToastContainer(props: ToastContainerProps) -> Element {
    let mut class_list = vec!["toast-container".to_string()];
    
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
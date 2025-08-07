use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum FormMethod {
    Get,
    Post,
    None,
}

impl From<FormMethod> for &str {
    fn from(value: FormMethod) -> Self {
        match value {
            FormMethod::Get => "get",
            FormMethod::Post => "post",
            _ => "",
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct FormProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    #[props(optional, default = false)]
    disabled: bool,
    #[props(optional, default = None)]
    action: Option<String>,
    #[props(optional, default = FormMethod::None)]
    method: FormMethod,
    children: Element,
}

#[component]
pub fn Form(props: FormProps) -> Element {
    //let mut class_list = vec!["btn".to_string()];
    //let class_list = class_list.join(" ");
    let method: &str = props.method.into();
    rsx!{
        form {
            id: props.id,
            class: props.class,
            action: props.action,
            method: method,
            fieldset {
                disabled: props.disabled,
                {props.children}
            }
        }
    }
}
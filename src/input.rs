use dioxus::prelude::*;
use super::size::Size;

#[derive(Clone, Copy, PartialEq)]
pub enum InputType {
    Text,
    Email,
    Password,
    Number,
    Tel,
    Url,
    Search,
    Date,
    Time,
    DateTime,
    Month,
    Week,
    Color,
    File,
    Hidden,
    Range,
}

impl Into<&'static str> for InputType {
    fn into(self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Email => "email",
            InputType::Password => "password",
            InputType::Number => "number",
            InputType::Tel => "tel",
            InputType::Url => "url",
            InputType::Search => "search",
            InputType::Date => "date",
            InputType::Time => "time",
            InputType::DateTime => "datetime-local",
            InputType::Month => "month",
            InputType::Week => "week",
            InputType::Color => "color",
            InputType::File => "file",
            InputType::Hidden => "hidden",
            InputType::Range => "range",
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct InputProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    #[props(optional, default = InputType::Text)]
    input_type: InputType,
    #[props(optional, default = Size::Normal)]
    size: Size,
    #[props(optional, default = "".to_string())]
    placeholder: String,
    #[props(optional, default = "".to_string())]
    value: String,
    #[props(optional, default = false)]
    disabled: bool,
    #[props(optional, default = false)]
    readonly: bool,
    #[props(optional, default = false)]
    plaintext: bool,
    #[props(optional, default = None)]
    min: Option<String>,
    #[props(optional, default = None)]
    max: Option<String>,
    #[props(optional, default = None)]
    step: Option<String>,
    #[props(optional, default = None)]
    pattern: Option<String>,
    #[props(optional)]
    oninput: EventHandler<FormEvent>,
    #[props(optional)]
    onchange: EventHandler<FormEvent>,
    #[props(optional)]
    onfocus: EventHandler<FocusEvent>,
    #[props(optional)]
    onblur: EventHandler<FocusEvent>,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    let mut class_list = if props.plaintext {
        vec!["form-control-plaintext".to_string()]
    } else {
        vec!["form-control".to_string()]
    };
    
    let size: &str = props.size.into();
    if props.size != Size::Normal && !props.plaintext {
        class_list.push(format!("form-control-{}", size));
    }
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }
    
    let class_list = class_list.join(" ");
    let input_type: &str = props.input_type.into();
    
    rsx! {
        input {
            id: props.id,
            r#type: input_type,
            class: class_list,
            placeholder: props.placeholder,
            value: props.value,
            disabled: props.disabled,
            readonly: props.readonly,
            min: props.min,
            max: props.max,
            step: props.step,
            pattern: props.pattern,
            oninput: props.oninput,
            onchange: props.onchange,
            onfocus: props.onfocus,
            onblur: props.onblur,
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct TextareaProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    #[props(optional, default = Size::Normal)]
    size: Size,
    #[props(optional, default = "".to_string())]
    placeholder: String,
    #[props(optional, default = "".to_string())]
    value: String,
    #[props(optional, default = false)]
    disabled: bool,
    #[props(optional, default = false)]
    readonly: bool,
    #[props(optional, default = None)]
    rows: Option<u32>,
    #[props(optional, default = None)]
    cols: Option<u32>,
    #[props(optional)]
    oninput: EventHandler<FormEvent>,
    #[props(optional)]
    onchange: EventHandler<FormEvent>,
    #[props(optional)]
    onfocus: EventHandler<FocusEvent>,
    #[props(optional)]
    onblur: EventHandler<FocusEvent>,
}

#[component]
pub fn Textarea(props: TextareaProps) -> Element {
    let mut class_list = vec!["form-control".to_string()];
    
    let size: &str = props.size.into();
    if props.size != Size::Normal {
        class_list.push(format!("form-control-{}", size));
    }
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }
    
    let class_list = class_list.join(" ");
    
    rsx! {
        textarea {
            id: props.id,
            class: class_list,
            placeholder: props.placeholder,
            value: props.value,
            disabled: props.disabled,
            readonly: props.readonly,
            rows: props.rows,
            cols: props.cols,
            oninput: props.oninput,
            onchange: props.onchange,
            onfocus: props.onfocus,
            onblur: props.onblur,
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct SelectProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    #[props(optional, default = Size::Normal)]
    size: Size,
    #[props(optional, default = "".to_string())]
    value: String,
    #[props(optional, default = false)]
    disabled: bool,
    #[props(optional, default = false)]
    multiple: bool,
    #[props(optional)]
    onchange: EventHandler<FormEvent>,
    children: Element,
}

#[component]
pub fn Select(props: SelectProps) -> Element {
    let mut class_list = vec!["form-select".to_string()];
    
    let size: &str = props.size.into();
    if props.size != Size::Normal {
        class_list.push(format!("form-select-{}", size));
    }
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }
    
    let class_list = class_list.join(" ");
    
    rsx! {
        select {
            id: props.id,
            class: class_list,
            value: props.value,
            disabled: props.disabled,
            multiple: props.multiple,
            onchange: props.onchange,
            {props.children}
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct CheckboxProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    #[props(optional, default = "".to_string())]
    value: String,
    #[props(optional, default = false)]
    checked: bool,
    #[props(optional, default = false)]
    disabled: bool,
    #[props(optional, default = false)]
    inline: bool,
    #[props(optional, default = false)]
    switch: bool,
    #[props(optional)]
    onchange: EventHandler<FormEvent>,
    #[props(optional, default = None)]
    label: Option<String>,
}

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let input_class = vec!["form-check-input".to_string()];
    
    let mut container_class = if props.switch {
        vec!["form-check".to_string(), "form-switch".to_string()]
    } else {
        vec!["form-check".to_string()]
    };
    
    if props.inline {
        container_class.push("form-check-inline".to_string());
    }
    
    if !props.class.is_empty() {
        container_class.push(props.class.clone());
    }
    
    let input_class = input_class.join(" ");
    let container_class = container_class.join(" ");
    let label_id = format!("{}-label", &props.id);
    
    rsx! {
        div {
            class: container_class,
            input {
                id: props.id.clone(),
                r#type: "checkbox",
                class: input_class,
                value: props.value,
                checked: props.checked,
                disabled: props.disabled,
                onchange: props.onchange,
            }
            if let Some(label_text) = props.label {
                label {
                    id: label_id,
                    class: "form-check-label",
                    r#for: props.id.clone(),
                    "{label_text}"
                }
            }
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct RadioProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    #[props(optional, default = "".to_string())]
    name: String,
    #[props(optional, default = "".to_string())]
    value: String,
    #[props(optional, default = false)]
    checked: bool,
    #[props(optional, default = false)]
    disabled: bool,
    #[props(optional, default = false)]
    inline: bool,
    #[props(optional)]
    onchange: EventHandler<FormEvent>,
    #[props(optional, default = None)]
    label: Option<String>,
}

#[component]
pub fn Radio(props: RadioProps) -> Element {
    let input_class = "form-check-input";
    
    let mut container_class = vec!["form-check".to_string()];
    
    if props.inline {
        container_class.push("form-check-inline".to_string());
    }
    
    if !props.class.is_empty() {
        container_class.push(props.class.clone());
    }
    
    let container_class = container_class.join(" ");
    let label_id = format!("{}-label", &props.id);
    
    rsx! {
        div {
            class: container_class,
            input {
                id: props.id.clone(),
                r#type: "radio",
                class: input_class,
                name: props.name,
                value: props.value,
                checked: props.checked,
                disabled: props.disabled,
                onchange: props.onchange,
            }
            if let Some(label_text) = props.label {
                label {
                    id: label_id,
                    class: "form-check-label",
                    r#for: props.id.clone(),
                    "{label_text}"
                }
            }
        }
    }
}
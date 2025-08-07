use dioxus::prelude::*;

#[derive(Clone, Props, PartialEq)]
pub struct AccordionProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    #[props(optional, default = false)]
    flush: bool,
    children: Element,
}

#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    let mut class_list = vec!["accordion".to_string()];
    
    if props.flush {
        class_list.push("accordion-flush".to_string());
    }
    
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
pub struct AccordionItemProps {
    #[props(optional)]
    id: String,
    #[props(optional, default = "".to_string())]
    class: String,
    /// Unique identifier for this accordion item
    #[props(optional)]
    item_id: String,
    /// Parent accordion ID for proper collapse behavior
    parent_id: String,
    /// Header text for the accordion item
    header: String,
    /// Whether this item is open by default
    #[props(optional, default = false)]
    show: bool,
    children: Element,
}

#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element {
    let mut class_list = vec!["accordion-item".to_string()];
    
    if !props.class.is_empty() {
        class_list.push(props.class.clone());
    }
    
    let class_list = class_list.join(" ");
    let collapse_id = if props.item_id.is_empty() {
        format!("collapse-{}", props.id)
    } else {
        props.item_id.clone()
    };
    let header_id = format!("heading-{}", collapse_id);
    
    let mut collapse_classes = vec!["accordion-collapse".to_string(), "collapse".to_string()];
    if props.show {
        collapse_classes.push("show".to_string());
    }
    let collapse_class = collapse_classes.join(" ");
    
    rsx! {
        div {
            id: props.id,
            class: class_list,
            h2 {
                id: header_id.clone(),
                class: "accordion-header",
                button {
                    r#type: "button",
                    class: if props.show { "accordion-button" } else { "accordion-button collapsed" },
                    "data-bs-toggle": "collapse",
                    "data-bs-target": format!("#{}", collapse_id),
                    "aria-expanded": if props.show { "true" } else { "false" },
                    "aria-controls": collapse_id.clone(),
                    "{props.header}"
                }
            }
            div {
                id: collapse_id.clone(),
                class: collapse_class,
                "data-bs-parent": format!("#{}", props.parent_id),
                div {
                    class: "accordion-body",
                    {props.children}
                }
            }
        }
    }
}
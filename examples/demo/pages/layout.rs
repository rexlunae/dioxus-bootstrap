use dioxus::prelude::*;
use dioxus_bootstrap::*;
use crate::components::*;

#[component]
pub fn LayoutPage() -> Element {
    rsx! {
        div {
            id: "layout",
            h2 { class: "mb-4", "Layout Components" }
            p {
                class: "lead",
                "Bootstrap's powerful, responsive layout system with containers, grids, and cards."
            }
            
            ContainerSection {}
            GridSection {}
            CardSection {}
            ModalSection {}
            AccordionSection {}
        }
    }
}

#[component]
pub fn ContainerSection() -> Element {
    rsx! {
        ExampleSection {
            title: "Containers".to_string(),
            description: Some("Container components provide responsive fixed-width or fluid-width containers.".to_string()),
            code: r#"Container {
    h4 { "Regular Container" }
    p { "This container has responsive fixed max-widths." }
}

Container {
    container_type: ContainerType::ContainerFluid,
    h4 { "Fluid Container" }
    p { "This container spans the full width of the viewport." }
}"#.to_string(),
            
            div {
                id: "containers",
                Container {
                    class: "border p-3 mb-3".to_string(),
                    h4 { "Regular Container" }
                    p { "This container has responsive fixed max-widths." }
                }
                
                Container {
                    container_type: ContainerType::ContainerFluid,
                    class: "border p-3".to_string(),
                    h4 { "Fluid Container" }
                    p { "This container spans the full width of the viewport." }
                }
            }
        }
    }
}

#[component]
pub fn GridSection() -> Element {
    rsx! {
        ExampleSection {
            title: "Grid System".to_string(),
            description: Some("Bootstrap's grid system uses flexbox and is fully responsive. Use Row and Col components.".to_string()),
            code: r#"Row {
    gutter: Some(3),
    Col { md: Some(4), "Column 1" }
    Col { md: Some(4), "Column 2" }
    Col { md: Some(4), "Column 3" }
}

Row {
    Col { sm: Some(6), md: Some(8), "sm-6 md-8" }
    Col { sm: Some(6), md: Some(4), "sm-6 md-4" }
}

Row {
    justify_content: Some(JustifyContent::Center),
    align_items: Some(AlignItems::Center),
    Col { 
        span: Some(6), 
        class: "border p-4 text-center bg-body-secondary".to_string(),
        "Centered Column" 
    }
}"#.to_string(),
            
            div {
                id: "grid",
                Row {
                    gutter: Some(3),
                    Col { 
                        md: Some(4), 
                        class: "border p-2 text-center".to_string(),
                        "Column 1" 
                    }
                    Col { 
                        md: Some(4), 
                        class: "border p-2 text-center".to_string(),
                        "Column 2" 
                    }
                    Col { 
                        md: Some(4), 
                        class: "border p-2 text-center".to_string(),
                        "Column 3" 
                    }
                }
                
                Row {
                    class: "mt-3".to_string(),
                    Col { 
                        sm: Some(6), 
                        md: Some(8), 
                        class: "border p-2 text-center".to_string(),
                        "sm-6 md-8" 
                    }
                    Col { 
                        sm: Some(6), 
                        md: Some(4), 
                        class: "border p-2 text-center".to_string(),
                        "sm-6 md-4" 
                    }
                }
                
                Row {
                    class: "mt-3".to_string(),
                    justify_content: Some(JustifyContent::Center),
                    align_items: Some(AlignItems::Center),
                    Col { 
                        span: Some(6), 
                        class: "border p-4 text-center bg-body-secondary".to_string(),
                        "Centered Column" 
                    }
                }
            }
        }
    }
}

#[component]
pub fn CardSection() -> Element {
    rsx! {
        ExampleSection {
            title: "Cards".to_string(),
            description: Some("Cards provide a flexible container for displaying content with optional headers, bodies, and footers.".to_string()),
            code: r#"Card {
    CardHeader { "Featured" }
    CardBody {
        CardTitle { "Special title treatment" }
        CardText { "With supporting text below as a natural lead-in to additional content." }
        Button { variant: ButtonVariant::Primary, "Go somewhere" }
    }
    CardFooter { class: "text-muted", "2 days ago" }
}"#.to_string(),
            
            div {
                id: "cards",
                Row {
                    Col { md: Some(6),
                        Card {
                            CardHeader { "Featured" }
                            CardBody {
                                CardTitle { "Special title treatment" }
                                CardText { "With supporting text below as a natural lead-in to additional content." }
                                Button { variant: ButtonVariant::Primary, "Go somewhere" }
                            }
                            CardFooter { class: "text-muted".to_string(), "2 days ago" }
                        }
                    }
                    Col { md: Some(6),
                        Card {
                            CardImage { 
                                src: "https://picsum.photos/300/150".to_string(), 
                                alt: "Card image".to_string(), 
                                top: true 
                            }
                            CardBody {
                                CardTitle { "Image card" }
                                CardText { "This card has an image at the top to showcase the CardImage component." }
                                Badge { variant: BadgeVariant::Info, "New" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ModalSection() -> Element {
    rsx! {
        ExampleSection {
            title: "Modals".to_string(),
            description: Some("Modal dialogs for lightboxes, user notifications, or custom content.".to_string()),
            code: r##"// Button to trigger modal
Button {
    variant: ButtonVariant::Primary,
    r#"data-bs-toggle": "modal",
    r#"data-bs-target": "#exampleModal",
    "Launch demo modal"
}

// Modal component
Modal {
    id: "exampleModal",
    fade: true,
    ModalHeader {
        ModalTitle { "Modal title" }
    }
    ModalBody {
        "Modal body text goes here."
    }
    ModalFooter {
        Button { variant: ButtonVariant::Secondary, r#"data-bs-dismiss": "modal", "Close" }
        Button { variant: ButtonVariant::Primary, "Save changes" }
    }
}"##.to_string(),
            
            div {
                id: "modals",
                button {
                    class: "btn btn-primary",
                    "data-bs-toggle": "modal",
                    "data-bs-target": "#exampleModal",
                    "Launch demo modal"
                }
                
                Modal {
                    id: "exampleModal".to_string(),
                    fade: true,
                    ModalHeader {
                        ModalTitle { "Modal title" }
                    }
                    ModalBody {
                        "Modal body text goes here. This modal demonstrates the basic structure with header, body, and footer."
                    }
                    ModalFooter {
                        button { 
                            class: "btn btn-secondary",
                            "data-bs-dismiss": "modal", 
                            "Close" 
                        }
                        Button { variant: ButtonVariant::Primary, "Save changes" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn AccordionSection() -> Element {
    rsx! {
        ExampleSection {
            title: "Accordions".to_string(),
            description: Some("Vertically collapsing accordions that expand and collapse content panels.".to_string()),
            code: r#"Accordion {
    id: "accordionExample",
    AccordionItem {
        parent_id: "accordionExample",
        item_id: "collapseOne",
        header: "Accordion Item #1",
        show: true,
        "This is the first item's accordion body."
    }
    AccordionItem {
        parent_id: "accordionExample",
        item_id: "collapseTwo", 
        header: "Accordion Item #2",
        "This is the second item's accordion body."
    }
}"#.to_string(),
            
            div {
                id: "accordions",
                Accordion {
                    id: "layoutAccordion".to_string(),
                    AccordionItem {
                        parent_id: "layoutAccordion".to_string(),
                        item_id: "collapseOne".to_string(),
                        header: "Accordion Item #1".to_string(),
                        show: true,
                        "This is the first item's accordion body. It is shown by default, until the collapse plugin adds the appropriate classes that we use to style each element."
                    }
                    AccordionItem {
                        parent_id: "layoutAccordion".to_string(),
                        item_id: "collapseTwo".to_string(),
                        header: "Accordion Item #2".to_string(),
                        "This is the second item's accordion body. It is hidden by default, until the collapse plugin adds the appropriate classes."
                    }
                    AccordionItem {
                        parent_id: "layoutAccordion".to_string(),
                        item_id: "collapseThree".to_string(),
                        header: "Accordion Item #3".to_string(),
                        "This is the third item's accordion body. You can add any HTML content here, including other Bootstrap components."
                    }
                }
            }
        }
    }
}
use dioxus::prelude::*;
use dioxus_bootstrap::*;

#[component]
pub fn Page1() -> Element {
    rsx!{
        Container {
            h1 {"Bootstrap Component Showcase"}
            
            Row {
                gutter: Some(4),
                Col {
                    md: Some(6),
                    Card {
                        CardHeader { "Buttons & Form Controls" }
                        CardBody {
                            ButtonGroup {
                                label: "Button group example".to_string(),
                                Button { variant: ButtonVariant::Primary, "Primary" }
                                Button { variant: ButtonVariant::Secondary, "Secondary" }
                                Button { variant: ButtonVariant::Success, "Success" }
                            }
                            
                            div { class: "mt-3",
                                Input { 
                                    placeholder: "Enter your name".to_string(),
                                    class: "mb-2".to_string()
                                }
                                Select {
                                    class: "mb-2".to_string(),
                                    option { value: "", "Choose an option..." }
                                    option { value: "1", "Option 1" }
                                    option { value: "2", "Option 2" }
                                }
                                Checkbox { 
                                    id: "check1".to_string(),
                                    label: Some("Check me out".to_string()) 
                                }
                            }
                        }
                    }
                }
                
                Col {
                    md: Some(6),
                    Card {
                        CardHeader { "Alerts & Badges" }
                        CardBody {
                            Alert { 
                                variant: AlertVariant::Success, 
                                dismissible: true,
                                AlertHeading { "Well done!" }
                                "You successfully read this important alert message."
                            }
                            
                            div { class: "mt-3",
                                Badge { variant: BadgeVariant::Primary, "Primary" }
                                " "
                                Badge { variant: BadgeVariant::Secondary, pill: true, "Secondary Pill" }
                                " "
                                Badge { variant: BadgeVariant::Success, "Success" }
                            }
                        }
                    }
                }
            }
            
            Row {
                class: "mt-4".to_string(),
                Col {
                    Card {
                        CardHeader { "List Groups & Breadcrumbs" }
                        CardBody {
                            Breadcrumb {
                                BreadcrumbItem { active: false, href: Some("#".to_string()), "Home" }
                                BreadcrumbItem { active: false, href: Some("#".to_string()), "Library" }
                                BreadcrumbItem { active: true, "Data" }
                            }
                            
                            div { class: "mt-3",
                                ListGroup {
                                    ListGroupItem { 
                                        variant: Some(ListGroupItemVariant::Success),
                                        "A successful list group item"
                                    }
                                    ListGroupItem { "A simple default list group item" }
                                    ListGroupItem { 
                                        variant: Some(ListGroupItemVariant::Warning),
                                        "A warning list group item"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            Row {
                class: "mt-4".to_string(),
                Col {
                    Card {
                        CardHeader { "Accordion Example" }
                        CardBody {
                            Accordion {
                                id: "accordionExample".to_string(),
                                AccordionItem {
                                    parent_id: "accordionExample".to_string(),
                                    item_id: "collapse1".to_string(),
                                    header: "Accordion Item #1".to_string(),
                                    show: true,
                                    "This is the first item's accordion body. It is shown by default."
                                }
                                AccordionItem {
                                    parent_id: "accordionExample".to_string(),
                                    item_id: "collapse2".to_string(),
                                    header: "Accordion Item #2".to_string(),
                                    "This is the second item's accordion body."
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Page2(id: i32) -> Element {
    let text = format!("Advanced Components - Item {}", id);
    rsx!{
        Container {
            h1 {{text}}
            
            Row {
                Col {
                    md: Some(4),
                    DropdownButton {
                        text: "Dropdown Example".to_string(),
                        variant: ButtonVariant::Primary,
                        DropdownItem { href: Some("#".to_string()), "Action" }
                        DropdownItem { href: Some("#".to_string()), "Another action" }
                        DropdownDivider {}
                        DropdownItem { href: Some("#".to_string()), "Separated link" }
                    }
                }
                
                Col {
                    md: Some(8),
                    Card {
                        CardImage { src: "https://picsum.photos/400/200".to_string(), alt: "Card image".to_string(), top: true }
                        CardBody {
                            CardTitle { "Card with image" }
                            CardText { "This card has an image at the top and demonstrates the CardImage component." }
                            Button { variant: ButtonVariant::Primary, "Go somewhere" }
                        }
                    }
                }
            }
            
            Row {
                class: "mt-4".to_string(),
                Col {
                    h3 { "Pagination Example" }
                    Pagination {
                        PaginationItem { disabled: true, "Previous" }
                        PaginationItem { active: true, "1" }
                        PaginationItem { "2" }
                        PaginationItem { "3" }
                        PaginationItem { "Next" }
                    }
                }
            }
        }
    }
}
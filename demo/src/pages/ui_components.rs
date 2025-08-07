use dioxus::prelude::*;
use dioxus_bootstrap::*;
use crate::components::*;

#[component]
pub fn UIComponentsPage() -> Element {
    rsx! {
        div {
            id: "components",
            h2 { class: "mb-4", "UI Components" }
            p {
                class: "lead",
                "Interactive UI components including alerts, badges, navigation, and feedback components."
            }
            
            AlertSection {}
            BadgeSection {}
            BreadcrumbSection {}
            DropdownSection {}
            ListGroupSection {}
            PaginationSection {}
            ToastSection {}
        }
    }
}

#[component]
pub fn AlertSection() -> Element {
    let mut show_dismissible = use_signal(|| true);
    
    rsx! {
        ExampleSection {
            title: "Alerts".to_string(),
            description: Some("Contextual feedback messages for user actions with alert variants.".to_string()),
            code: r#"Alert { variant: AlertVariant::Success, "Success alert!" }
Alert { variant: AlertVariant::Info, "Info alert with more text!" }
Alert { variant: AlertVariant::Warning, "Warning alert!" }
Alert { variant: AlertVariant::Danger, "Error alert!" }

Alert {
    variant: AlertVariant::Primary,
    dismissible: true,
    AlertHeading { "Well done!" }
    "You successfully read this important alert message."
}"#.to_string(),
            
            div {
                id: "alerts",
                Alert { 
                    variant: AlertVariant::Success, 
                    class: "mb-3".to_string(),
                    "Success! Your action was completed successfully." 
                }
                Alert { 
                    variant: AlertVariant::Info, 
                    class: "mb-3".to_string(),
                    "Info! This alert needs your attention, but it's not super important." 
                }
                Alert { 
                    variant: AlertVariant::Warning, 
                    class: "mb-3".to_string(),
                    "Warning! Better check yourself, you're not looking too good." 
                }
                Alert { 
                    variant: AlertVariant::Danger, 
                    class: "mb-3".to_string(),
                    "Error! Something went wrong. Please try again." 
                }
                
                if show_dismissible() {
                    Alert {
                        variant: AlertVariant::Primary,
                        dismissible: true,
                        fade: true,
                        show: true,
                        AlertHeading { "Well done!" }
                        "You successfully read this important alert message. This example shows a dismissible alert that you can close."
                    }
                } else {
                    Button {
                        variant: ButtonVariant::Secondary,
                        outline: true,
                        onclick: move |_| show_dismissible.set(true),
                        "Show Dismissible Alert Again"
                    }
                }
            }
        }
    }
}

#[component]  
pub fn BadgeSection() -> Element {
    rsx! {
        ExampleSection {
            title: "Badges".to_string(),
            description: Some("Small count and labeling component with various colors and styles.".to_string()),
            code: r#"Badge { variant: BadgeVariant::Primary, "Primary" }
Badge { variant: BadgeVariant::Success, "Success" }
Badge { variant: BadgeVariant::Danger, "99+" }
Badge { variant: BadgeVariant::Warning, pill: true, "Pill badge" }

h4 { 
    "Example heading "
    Badge { variant: BadgeVariant::Secondary, "New" }
}"#.to_string(),
            
            div {
                id: "badges",
                div {
                    class: "mb-3",
                    h6 { "Basic Badges" }
                    Badge { variant: BadgeVariant::Primary, class: "me-1".to_string(), "Primary" }
                    Badge { variant: BadgeVariant::Secondary, class: "me-1".to_string(), "Secondary" }
                    Badge { variant: BadgeVariant::Success, class: "me-1".to_string(), "Success" }
                    Badge { variant: BadgeVariant::Danger, class: "me-1".to_string(), "Danger" }
                    Badge { variant: BadgeVariant::Warning, class: "me-1".to_string(), "Warning" }
                    Badge { variant: BadgeVariant::Info, class: "me-1".to_string(), "Info" }
                    Badge { variant: BadgeVariant::Light, class: "me-1".to_string(), "Light" }
                    Badge { variant: BadgeVariant::Dark, class: "me-1".to_string(), "Dark" }
                }
                
                div {
                    class: "mb-3",
                    h6 { "Pill Badges" }
                    Badge { variant: BadgeVariant::Primary, pill: true, class: "me-1".to_string(), "Primary" }
                    Badge { variant: BadgeVariant::Success, pill: true, class: "me-1".to_string(), "Success" }
                    Badge { variant: BadgeVariant::Danger, pill: true, class: "me-1".to_string(), "99+" }
                }
                
                div {
                    h6 { "Badges in Context" }
                    h4 { 
                        "Example heading "
                        Badge { variant: BadgeVariant::Secondary, "New" }
                    }
                    p {
                        "Notifications "
                        Badge { variant: BadgeVariant::Danger, pill: true, "4" }
                    }
                    Button { 
                        variant: ButtonVariant::Primary,
                        "Profile "
                        Badge { variant: BadgeVariant::Light, "9" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn BreadcrumbSection() -> Element {
    rsx! {
        ExampleSection {
            title: "Breadcrumbs".to_string(),
            description: Some("Navigation breadcrumbs to indicate current page location.".to_string()),
            code: r#"Breadcrumb {
    BreadcrumbItem { href: Some("/"), "Home" }
    BreadcrumbItem { href: Some("/components"), "Components" }
    BreadcrumbItem { active: true, "Breadcrumbs" }
}

Breadcrumb {
    divider: ">",
    BreadcrumbItem { href: Some("/"), "Home" }
    BreadcrumbItem { active: true, "Current" }
}"#.to_string(),
            
            div {
                id: "breadcrumbs",
                Breadcrumb {
                    class: "mb-3".to_string(),
                    BreadcrumbItem { active: false, href: Some("/".to_string()), "Home" }
                    BreadcrumbItem { active: false, href: Some("/components".to_string()), "Components" }
                    BreadcrumbItem { active: true, "Breadcrumbs" }
                }
                
                Breadcrumb {
                    divider: ">".to_string(),
                    class: "mb-3".to_string(),
                    BreadcrumbItem { active: false, href: Some("/".to_string()), "Home" }
                    BreadcrumbItem { active: false, href: Some("/docs".to_string()), "Docs" }
                    BreadcrumbItem { active: true, "Current" }
                }
                
                Breadcrumb {
                    divider: "»".to_string(),
                    BreadcrumbItem { active: false, href: Some("#".to_string()), "Products" }
                    BreadcrumbItem { active: false, href: Some("#".to_string()), "Laptops" }
                    BreadcrumbItem { active: false, href: Some("#".to_string()), "Gaming" }
                    BreadcrumbItem { active: true, "Laptop Model X" }
                }
            }
        }
    }
}

#[component]
pub fn DropdownSection() -> Element {
    rsx! {
        ExampleSection {
            title: "Dropdowns".to_string(),
            description: Some("Toggle contextual overlays for displaying lists of links and actions.".to_string()),
            code: r##"DropdownButton {
    text: "Dropdown",
    variant: ButtonVariant::Primary,
    DropdownItem { href: Some("#"), "Action" }
    DropdownItem { href: Some("#"), "Another action" }
    DropdownDivider {}
    DropdownItem { href: Some("#"), "Separated link" }
}

DropdownButton {
    text: "Split dropdown",
    variant: ButtonVariant::Success,
    split: true,
    DropdownItem { href: Some("#"), "Action" }
    DropdownItem { href: Some("#"), "Another action" }
}"##.to_string(),
            
            div {
                id: "dropdowns",
                div {
                    class: "mb-3",
                    DropdownButton {
                        text: "Primary Dropdown".to_string(),
                        variant: ButtonVariant::Primary,
                        class: "me-2".to_string(),
                        DropdownItem { href: Some("#".to_string()), "Action" }
                        DropdownItem { href: Some("#".to_string()), "Another action" }
                        DropdownItem { href: Some("#".to_string()), "Something else here" }
                        DropdownDivider {}
                        DropdownItem { href: Some("#".to_string()), "Separated link" }
                    }
                    
                    DropdownButton {
                        text: "Success Split".to_string(),
                        variant: ButtonVariant::Success,
                        split: true,
                        class: "me-2".to_string(),
                        DropdownItem { href: Some("#".to_string()), "Action" }
                        DropdownItem { href: Some("#".to_string()), "Another action" }
                        DropdownItem { href: Some("#".to_string()), "Something else here" }
                    }
                    
                    DropdownButton {
                        text: "Danger".to_string(),
                        variant: ButtonVariant::Danger,
                        DropdownHeader { "Dropdown header" }
                        DropdownItem { href: Some("#".to_string()), "Action" }
                        DropdownItem { href: Some("#".to_string()), "Another action" }
                        DropdownDivider {}
                        DropdownItem { href: Some("#".to_string()), "Delete" }
                    }
                }
                
                p { class: "text-muted", "Click the buttons above to see the dropdown menus in action." }
            }
        }
    }
}

#[component]
pub fn ListGroupSection() -> Element {
    rsx! {
        ExampleSection {
            title: "List Groups".to_string(),
            description: Some("Display a series of content with list group items and variants.".to_string()),
            code: r#"ListGroup {
    ListGroupItem { "An item" }
    ListGroupItem { active: true, "A second item" }
    ListGroupItem { "A third item" }
    ListGroupItem { disabled: true, "A disabled item" }
}

ListGroup {
    flush: true,
    ListGroupItem { variant: Some(ListGroupItemVariant::Success), "Success item" }
    ListGroupItem { variant: Some(ListGroupItemVariant::Warning), "Warning item" }
    ListGroupItem { variant: Some(ListGroupItemVariant::Danger), "Danger item" }
}"#.to_string(),
            
            div {
                id: "listgroups",
                Row {
                    Col { md: Some(6),
                        h6 { "Basic List Group" }
                        ListGroup {
                            class: "mb-3".to_string(),
                            ListGroupItem { "An item" }
                            ListGroupItem { active: true, "A second item" }
                            ListGroupItem { "A third item" }
                            ListGroupItem { "A fourth item" }
                            ListGroupItem { disabled: true, "A disabled item" }
                        }
                    }
                    Col { md: Some(6),
                        h6 { "Contextual Classes" }
                        ListGroup {
                            ListGroupItem { variant: Some(ListGroupItemVariant::Success), "A successful list group item" }
                            ListGroupItem { variant: Some(ListGroupItemVariant::Info), "An info list group item" }
                            ListGroupItem { variant: Some(ListGroupItemVariant::Warning), "A warning list group item" }
                            ListGroupItem { variant: Some(ListGroupItemVariant::Danger), "A danger list group item" }
                        }
                    }
                }
                
                h6 { "Flush List Group" }
                ListGroup {
                    flush: true,
                    ListGroupItem { "An item" }
                    ListGroupItem { "A second item" } 
                    ListGroupItem { "A third item" }
                }
            }
        }
    }
}

#[component]
pub fn PaginationSection() -> Element {
    let mut current_page = use_signal(|| 1);
    let total_pages = 5;
    
    rsx! {
        ExampleSection {
            title: "Pagination".to_string(),
            description: Some("Navigation for paginated content with active and disabled states.".to_string()),
            code: r#"Pagination {
    PaginationItem { disabled: current_page == 1, "Previous" }
    for page in 1..=total_pages {
        PaginationItem { 
            active: page == current_page,
            onclick: move |_| current_page.set(page),
            "{page}"
        }
    }
    PaginationItem { disabled: current_page == total_pages, "Next" }
}

Pagination {
    size: Size::Small,
    PaginationItem { "1" }
    PaginationItem { active: true, "2" }
    PaginationItem { "3" }
}"#.to_string(),
            
            div {
                id: "pagination",
                div {
                    class: "mb-3",
                    h6 { "Interactive Pagination" }
                    Pagination {
                        PaginationItem { 
                            disabled: current_page() == 1,
                            onclick: move |_| if current_page() > 1 { current_page.set(current_page() - 1) },
                            "Previous"
                        }
                        for page in 1..=total_pages {
                            PaginationItem { 
                                active: page == current_page(),
                                onclick: move |_| current_page.set(page),
                                "{page}"
                            }
                        }
                        PaginationItem { 
                            disabled: current_page() == total_pages,
                            onclick: move |_| if current_page() < total_pages { current_page.set(current_page() + 1) },
                            "Next"
                        }
                    }
                    p { class: "text-muted", "Current page: {current_page()}" }
                }
                
                div {
                    class: "mb-3",
                    h6 { "Pagination Sizes" }
                    Pagination {
                        size: Size::Large,
                        class: "mb-2".to_string(),
                        PaginationItem { "1" }
                        PaginationItem { active: true, "2" }
                        PaginationItem { "3" }
                    }
                    Pagination {
                        class: "mb-2".to_string(),
                        PaginationItem { "1" }
                        PaginationItem { active: true, "2" }  
                        PaginationItem { "3" }
                    }
                    Pagination {
                        size: Size::Small,
                        PaginationItem { "1" }
                        PaginationItem { active: true, "2" }
                        PaginationItem { "3" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ToastSection() -> Element {
    let mut show_toast = use_signal(|| false);
    
    rsx! {
        ExampleSection {
            title: "Toasts".to_string(),
            description: Some("Push notifications to visitors with toast messages.".to_string()),
            code: r#"Toast {
    show: true,
    fade: true,
    ToastHeader {
        strong { class: "me-auto", "Bootstrap" }
        small { "11 mins ago" }
    }
    ToastBody {
        "Hello, world! This is a toast message."
    }
}"#.to_string(),
            
            div {
                id: "toasts",
                Button {
                    variant: ButtonVariant::Primary,
                    class: "mb-3".to_string(),
                    onclick: move |_| show_toast.set(true),
                    "Show Toast"
                }
                
                if show_toast() {
                    div {
                        class: "position-relative",
                        style: "min-height: 200px;",
                        div {
                            class: "position-absolute top-0 end-0",
                            ToastContainer {
                                Toast {
                                    show: true,
                                    fade: true,
                                    ToastHeader {
                                        strong { class: "me-auto", "🎉 Bootstrap" }
                                        small { "just now" }
                                        button {
                                            r#type: "button",
                                            class: "btn-close",
                                            onclick: move |_| show_toast.set(false)
                                        }
                                    }
                                    ToastBody {
                                        "Hello, world! This is a toast message demonstrating the Toast component."
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
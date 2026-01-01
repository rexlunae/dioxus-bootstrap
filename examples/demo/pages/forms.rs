use dioxus::prelude::*;
use dioxus_bootstrap::*;
use crate::components::*;

#[component]
pub fn FormsPage() -> Element {
    rsx! {
        div {
            id: "forms",
            h2 { class: "mb-4", "Form Components" }
            p {
                class: "lead",
                "Complete form controls with Bootstrap styling, validation states, and accessibility features."
            }
            
            InputSection {}
            ButtonSection {}
            CheckboxRadioSection {}
            SelectSection {}
            InteractiveFormSection {}
        }
    }
}

#[component]
pub fn InputSection() -> Element {
    rsx! {
        ExampleSection {
            title: "Input Fields".to_string(),
            description: Some("Text inputs with various types, sizes, and states.".to_string()),
            code: r#"Input {
    placeholder: "Enter your name",
    class: "mb-3"
}
Input {
    input_type: InputType::Email,
    placeholder: "email@example.com",
    class: "mb-3"
}
Input {
    input_type: InputType::Password,
    placeholder: "Password",
    class: "mb-3"
}
Textarea {
    placeholder: "Leave a comment here",
    rows: Some(3)
}"#.to_string(),
            
            div {
                id: "inputs",
                Input {
                    placeholder: "Enter your name".to_string(),
                    class: "mb-3".to_string()
                }
                Input {
                    input_type: InputType::Email,
                    placeholder: "email@example.com".to_string(),
                    class: "mb-3".to_string()
                }
                Input {
                    input_type: InputType::Password,
                    placeholder: "Password".to_string(),
                    class: "mb-3".to_string()
                }
                Input {
                    input_type: InputType::Date,
                    class: "mb-3".to_string()
                }
                Textarea {
                    placeholder: "Leave a comment here".to_string(),
                    rows: Some(3)
                }
            }
        }
    }
}

#[component]
pub fn ButtonSection() -> Element {
    rsx! {
        ExampleSection {
            title: "Buttons".to_string(),
            description: Some("Bootstrap styled buttons with variants, sizes, and states.".to_string()),
            code: r#"Button { variant: ButtonVariant::Primary, "Primary" }
Button { variant: ButtonVariant::Secondary, "Secondary" }
Button { variant: ButtonVariant::Success, "Success" }
Button { variant: ButtonVariant::Danger, "Danger" }
Button { variant: ButtonVariant::Warning, "Warning" }
Button { variant: ButtonVariant::Info, "Info" }
Button { variant: ButtonVariant::Light, "Light" }
Button { variant: ButtonVariant::Dark, "Dark" }

// Outline buttons
Button { variant: ButtonVariant::Primary, outline: true, "Primary Outline" }

// Button sizes
Button { variant: ButtonVariant::Primary, size: Size::Small, "Small" }
Button { variant: ButtonVariant::Primary, size: Size::Large, "Large" }"#.to_string(),
            
            div {
                id: "buttons",
                div {
                    class: "mb-3",
                    h6 { "Standard Buttons" }
                    Button { variant: ButtonVariant::Primary, class: "me-2 mb-2".to_string(), "Primary" }
                    Button { variant: ButtonVariant::Secondary, class: "me-2 mb-2".to_string(), "Secondary" }
                    Button { variant: ButtonVariant::Success, class: "me-2 mb-2".to_string(), "Success" }
                    Button { variant: ButtonVariant::Danger, class: "me-2 mb-2".to_string(), "Danger" }
                    Button { variant: ButtonVariant::Warning, class: "me-2 mb-2".to_string(), "Warning" }
                    Button { variant: ButtonVariant::Info, class: "me-2 mb-2".to_string(), "Info" }
                    Button { variant: ButtonVariant::Light, class: "me-2 mb-2".to_string(), "Light" }
                    Button { variant: ButtonVariant::Dark, class: "me-2 mb-2".to_string(), "Dark" }
                }
                
                div {
                    class: "mb-3",
                    h6 { "Outline Buttons" }
                    Button { variant: ButtonVariant::Primary, outline: true, class: "me-2 mb-2".to_string(), "Primary" }
                    Button { variant: ButtonVariant::Secondary, outline: true, class: "me-2 mb-2".to_string(), "Secondary" }
                    Button { variant: ButtonVariant::Success, outline: true, class: "me-2 mb-2".to_string(), "Success" }
                }
                
                div {
                    class: "mb-3",
                    h6 { "Button Sizes" }
                    Button { variant: ButtonVariant::Primary, size: Size::Small, class: "me-2 mb-2".to_string(), "Small" }
                    Button { variant: ButtonVariant::Primary, class: "me-2 mb-2".to_string(), "Normal" }
                    Button { variant: ButtonVariant::Primary, size: Size::Large, class: "me-2 mb-2".to_string(), "Large" }
                }
                
                div {
                    h6 { "Button Group" }
                    ButtonGroup {
                        label: "Button group example".to_string(),
                        Button { variant: ButtonVariant::Primary, outline: true, "Left" }
                        Button { variant: ButtonVariant::Primary, "Middle" }
                        Button { variant: ButtonVariant::Primary, "Right" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn CheckboxRadioSection() -> Element {
    let mut checked_values = use_signal(|| vec![false, true, false]);
    let mut radio_value = use_signal(|| "option1");
    
    rsx! {
        ExampleSection {
            title: "Checkboxes & Radio Buttons".to_string(),
            description: Some("Form controls for single and multiple choice selections.".to_string()),
            code: r#"Checkbox {
    id: "check1",
    label: Some("Default checkbox"),
    checked: false
}
Checkbox {
    id: "check2", 
    label: Some("Checked checkbox"),
    checked: true
}
Checkbox {
    id: "switch1",
    label: Some("Default switch"),
    switch: true
}

Radio {
    id: "radio1",
    name: "exampleRadios",
    value: "option1",
    label: Some("Default radio"),
    checked: true
}
Radio {
    id: "radio2",
    name: "exampleRadios", 
    value: "option2",
    label: Some("Second default radio")
}"#.to_string(),
            
            div {
                id: "checkboxes",
                div {
                    class: "mb-4",
                    h6 { "Checkboxes" }
                    Checkbox {
                        id: "check1".to_string(),
                        label: Some("Default checkbox".to_string()),
                        checked: checked_values()[0],
                        onchange: move |_| {
                            let mut vals = checked_values();
                            vals[0] = !vals[0];
                            checked_values.set(vals);
                        }
                    }
                    Checkbox {
                        id: "check2".to_string(),
                        label: Some("Checked checkbox".to_string()),
                        checked: checked_values()[1],
                        onchange: move |_| {
                            let mut vals = checked_values();
                            vals[1] = !vals[1];
                            checked_values.set(vals);
                        }
                    }
                    Checkbox {
                        id: "check3".to_string(),
                        label: Some("Disabled checkbox".to_string()),
                        disabled: true,
                        checked: checked_values()[2]
                    }
                }
                
                div {
                    class: "mb-4",
                    h6 { "Switches" }
                    Checkbox {
                        id: "switch1".to_string(),
                        label: Some("Default switch".to_string()),
                        switch: true,
                        checked: true
                    }
                    Checkbox {
                        id: "switch2".to_string(),
                        label: Some("Checked switch".to_string()),
                        switch: true,
                        checked: false
                    }
                }
                
                div {
                    h6 { "Radio buttons" }
                    Radio {
                        id: "radio1".to_string(),
                        name: "exampleRadios".to_string(),
                        value: "option1".to_string(),
                        label: Some("Default radio".to_string()),
                        checked: radio_value() == "option1",
                        onchange: move |_| radio_value.set("option1")
                    }
                    Radio {
                        id: "radio2".to_string(),
                        name: "exampleRadios".to_string(),
                        value: "option2".to_string(),
                        label: Some("Second default radio".to_string()),
                        checked: radio_value() == "option2",
                        onchange: move |_| radio_value.set("option2")
                    }
                    Radio {
                        id: "radio3".to_string(),
                        name: "exampleRadios".to_string(),
                        value: "option3".to_string(),
                        label: Some("Disabled radio".to_string()),
                        disabled: true,
                        checked: false
                    }
                }
            }
        }
    }
}

#[component]
pub fn SelectSection() -> Element {
    let mut select_value = use_signal(|| String::new());
    
    rsx! {
        ExampleSection {
            title: "Select Dropdowns".to_string(),
            description: Some("Dropdown select menus with Bootstrap styling.".to_string()),
            code: r#"Select {
    class: "mb-3",
    option { value: "", "Choose..." }
    option { value: "1", "Option 1" }
    option { value: "2", "Option 2" } 
    option { value: "3", "Option 3" }
}

Select {
    multiple: true,
    option { value: "1", "Option 1" }
    option { value: "2", "Option 2" }
    option { value: "3", "Option 3" }
}"#.to_string(),
            
            div {
                id: "selects",
                Select {
                    class: "mb-3".to_string(),
                    value: select_value().to_string(),
                    onchange: move |evt: FormEvent| select_value.set(evt.value()),
                    option { value: "", "Choose..." }
                    option { value: "1", "Option 1" }
                    option { value: "2", "Option 2" }
                    option { value: "3", "Option 3" }
                }
                
                Select {
                    multiple: true,
                    class: "mb-3".to_string(),
                    option { value: "1", "Option 1" }
                    option { value: "2", "Option 2" }
                    option { value: "3", "Option 3" }
                    option { value: "4", "Option 4" }
                    option { value: "5", "Option 5" }
                }
                
                div {
                    h6 { "Select Sizes" }
                    Select {
                        size: Size::Small,
                        class: "mb-2".to_string(),
                        option { value: "", "Small select" }
                        option { value: "1", "Option 1" }
                    }
                    Select {
                        size: Size::Large,
                        option { value: "", "Large select" }
                        option { value: "1", "Option 1" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn InteractiveFormSection() -> Element {
    let mut form_data = use_signal(|| FormData {
        name: String::new(),
        email: String::new(),
        message: String::new(),
        newsletter: false,
        plan: String::new(),
    });
    
    rsx! {
        ExampleSection {
            title: "Interactive Form Example".to_string(),
            description: Some("A complete form demonstrating all form components working together.".to_string()),
            code: r#"Form {
    Row {
        Col { md: Some(6),
            label { class: "form-label", "Name" }
            Input { 
                placeholder: "Enter your full name",
                value: form_data.name,
                oninput: |evt| form_data.name = evt.data.value()
            }
        }
        Col { md: Some(6),
            label { class: "form-label", "Email" }
            Input {
                input_type: InputType::Email,
                placeholder: "Enter your email",
                value: form_data.email,
                oninput: |evt| form_data.email = evt.data.value()
            }
        }
    }
    // ... more form fields
}"#.to_string(),
            
            div {
                Form {
                    class: "p-4 border rounded".to_string(),
                    Row {
                        Col { md: Some(6),
                            label { 
                                class: "form-label",
                                r#for: "demoName",
                                "Name"
                            }
                            Input {
                                id: "demoName".to_string(),
                                placeholder: "Enter your full name".to_string(),
                                value: form_data().name.clone(),
                                class: "mb-3".to_string(),
                                oninput: move |evt: FormEvent| {
                                    let mut data = form_data();
                                    data.name = evt.value().clone();
                                    form_data.set(data);
                                }
                            }
                        }
                        Col { md: Some(6),
                            label { 
                                class: "form-label",
                                r#for: "demoEmail", 
                                "Email"
                            }
                            Input {
                                id: "demoEmail".to_string(),
                                input_type: InputType::Email,
                                placeholder: "Enter your email".to_string(),
                                value: form_data().email.clone(),
                                class: "mb-3".to_string(),
                                oninput: move |evt: FormEvent| {
                                    let mut data = form_data();
                                    data.email = evt.value().clone();
                                    form_data.set(data);
                                }
                            }
                        }
                    }
                    
                    label { 
                        class: "form-label",
                        r#for: "demoMessage",
                        "Message"
                    }
                    Textarea {
                        id: "demoMessage".to_string(),
                        placeholder: "Enter your message".to_string(),
                        rows: Some(4),
                        class: "mb-3".to_string(),
                        value: form_data().message.clone(),
                        oninput: move |evt: FormEvent| {
                            let mut data = form_data();
                            data.message = evt.value().clone();
                            form_data.set(data);
                        }
                    }
                    
                    Select {
                        class: "mb-3".to_string(),
                        value: form_data().plan.clone(),
                        onchange: move |evt: FormEvent| {
                            let mut data = form_data();
                            data.plan = evt.value().clone();
                            form_data.set(data);
                        },
                        option { value: "", "Choose a plan..." }
                        option { value: "basic", "Basic Plan - $9/month" }
                        option { value: "pro", "Pro Plan - $19/month" }
                        option { value: "enterprise", "Enterprise - $49/month" }
                    }
                    
                    Checkbox {
                        id: "newsletter".to_string(),
                        label: Some("Subscribe to newsletter".to_string()),
                        checked: form_data().newsletter,
                        class: "mb-3".to_string(),
                        onchange: move |_| {
                            let mut data = form_data();
                            data.newsletter = !data.newsletter;
                            form_data.set(data);
                        }
                    }
                    
                    div {
                        Button { 
                            variant: ButtonVariant::Primary, 
                            button_type: ButtonType::Submit,
                            class: "me-2".to_string(),
                            "Submit Form" 
                        }
                        Button { 
                            variant: ButtonVariant::Secondary,
                            button_type: ButtonType::Reset,
                            onclick: move |_| {
                                form_data.set(FormData {
                                    name: String::new(),
                                    email: String::new(), 
                                    message: String::new(),
                                    newsletter: false,
                                    plan: String::new(),
                                });
                            },
                            "Reset"
                        }
                    }
                    
                    if !form_data().name.is_empty() || !form_data().email.is_empty() {
                        div {
                            class: "mt-4 p-3 bg-body-secondary rounded",
                            h6 { "Form Data Preview:" }
                            pre {
                                class: "mb-0",
                                {format!("Name: {}\nEmail: {}\nPlan: {}\nNewsletter: {}\nMessage: {}", 
                                    form_data().name,
                                    form_data().email,
                                    form_data().plan,
                                    form_data().newsletter,
                                    form_data().message
                                )}
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct FormData {
    name: String,
    email: String,
    message: String,
    newsletter: bool,
    plan: String,
}
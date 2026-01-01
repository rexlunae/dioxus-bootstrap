//! Utility types for flexbox-related classes.

/// Implement this type for the Props object for a tag that can use flexbox.
pub trait FlexBoxContainerProps {
    fn prefix() -> &'static str;
}

/// ContainerType will either be ColProps or RowProps.
#[derive(Clone, PartialEq)]
pub enum FlexBoxClass<T: FlexBoxContainerProps> {
    /// Maps to the class "flex-nowrap"
    NoWrap,
    /// Maps to the class "col-auto or row-auto"
    Auto(T),
    /// Maps to d-flex or d-inline-flex.
    DFlex{inline: bool, size: Option<super::size::ExtendedSize>},
    /// Maps to either flex-row, flex-col, flex-row-reverse, flex-col-reverse
    Flex{reverse: bool, container: T},
    /// Maps to p-{size}
    Pad{size: u8}
}

use FlexBoxClass::*;

impl<T: FlexBoxContainerProps + Into<&'static str>> Into<String> for FlexBoxClass<T> {
    fn into(self) -> String {
        match self {
            NoWrap => "no-wrap".to_string(),
            Auto(t) => format!("{}-auto", T::prefix()),
            DFlex { inline, size } => {
                match size {
                    Some(s) => if inline {
                            let s: &str = s.into();
                            format!("d-{}-inline-flex", s)
                        } else {
                            let s: &str = s.into();
                            format!("d-{}-flex", s)
                        },
                    None => if inline {
                            "d-inline-flex".to_string()
                        } else {
                            "d-flex".to_string()
                        }
                }
            },
            Flex{reverse, container} => if reverse {
                let t: &str = container.into();
                format!("flex-{}-reverse", t)
            } else {
                let t: &str = container.into();
                format!("flex-{}", t)
            }
            _ => "".to_string()
        }
    }
}

pub type FlexBoxClassSet<T> = Vec<FlexBoxClass<T>>;

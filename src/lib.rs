pub mod background;
pub use background::*;

pub mod button;
pub use button::*;

pub mod form;
pub use form::*;

pub mod nav;
pub use nav::*;

pub mod size;
pub use size::*;

pub mod theme;
pub use theme::*;

pub mod container;
pub use container::*;

pub mod sidebar;
pub use sidebar::*;

pub mod tab;
pub use tab::*;

pub mod card;
pub use card::*;

pub mod modal;
pub use modal::*;

pub mod accordion;
pub use accordion::*;

pub mod alert;
pub use alert::*;

pub mod badge;
pub use badge::*;

pub mod breadcrumb;
pub use breadcrumb::*;

pub mod pagination;
pub use pagination::*;

pub mod input;
pub use input::*;

pub mod dropdown;
pub use dropdown::*;

pub mod list_group;
pub use list_group::*;

pub mod toast;
pub use toast::*;

pub mod utilities;
pub use utilities::*;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_theme_mode_from_str() {
        assert_eq!(ThemeMode::Dark, "dark".into());
        assert_eq!(ThemeMode::Light, "light".into());
        assert_eq!(ThemeMode::Auto, "auto".into());
        assert_eq!(ThemeMode::Auto, "invalid".into());
    }
    
    #[test]
    fn test_utility_spacing_functions() {
        assert_eq!("m-0", margin_class(SpacingSize::Zero));
        assert_eq!("m-1", margin_class(SpacingSize::One));
        assert_eq!("m-auto", margin_class(SpacingSize::Auto));
        
        assert_eq!("mx-2", margin_x_class(SpacingSize::Two));
        assert_eq!("my-3", margin_y_class(SpacingSize::Three));
        
        assert_eq!("p-4", padding_class(SpacingSize::Four));
        assert_eq!("px-5", padding_x_class(SpacingSize::Five));
        assert_eq!("py-0", padding_y_class(SpacingSize::Zero));
    }
    
    #[test]
    fn test_button_variant_basic() {
        let variant: &str = ButtonVariant::Primary.into();
        assert_eq!("primary", variant);
        
        let variant: &str = ButtonVariant::Basic.into();
        assert_eq!("", variant);
    }
    
    #[test]
    fn test_size_basic() {
        let size: &str = Size::Small.into();
        assert_eq!("sm", size);
        
        let size: &str = Size::Normal.into();
        assert_eq!("", size);
        
        let size: &str = Size::Large.into();
        assert_eq!("lg", size);
    }
}
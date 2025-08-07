# Dioxus Bootstrap Components

An unofficial Bootstrap 5.3 component library for Dioxus applications, providing a comprehensive set of responsive, accessible UI components with automatic light/dark mode switching.

> ⚠ This is very early work in progress. I am publishing it in case it is useful to anyone else, but please understand that it is expected to be wildly unstable for a while. ⚠

## Features

- 🎨 **Full Bootstrap 5.3 Integration** - Complete component library with Bootstrap styling
- 🌙 **Automatic Theme Switching** - Built-in light/dark mode with system preference detection
- 📱 **Responsive Design** - Mobile-first responsive grid system and components
- ♿ **Accessible** - Components follow Bootstrap's accessibility standards
- 🔗 **Router Integration** - Seamless integration with `dioxus-router`
- 📦 **CDN or Local Assets** - Flexible asset loading options
- 🧩 **Interoperable** - Mix library components with raw Bootstrap HTML

## Components

### Layout & Structure
- **Container, Row, Col** - Responsive grid system
- **Card** - Content containers with headers, bodies, and footers
- **Modal** - Dialog overlays
- **Accordion** - Collapsible content panels

### Navigation
- **NavBar** - Top navigation with branding and links
- **SideBar** - Sidebar navigation
- **Tab** - Tabbed content switching
- **Breadcrumb** - Hierarchical navigation
- **Pagination** - Page navigation controls

### Forms & Input
- **Button** - Various button styles and sizes
- **Input** - Text inputs with validation states
- **Select** - Dropdown selections
- **Textarea** - Multi-line text input
- **Checkbox, Radio** - Selection controls

### UI Elements
- **Alert** - Contextual feedback messages
- **Badge** - Small status indicators
- **Toast** - Temporary notifications
- **Dropdown** - Dropdown menus
- **ListGroup** - Flexible list components

### Utilities
- **Theme Components** - GlobalTheme and LocalTheme for theme management
- **Utility Classes** - Bootstrap utility classes for spacing, colors, display, etc.

## How to Use

To add this to your dioxus project, you can just run the following:

```bash
cargo add dioxus-bootstrap
```

Then you will need to ensure that the Bootstrap CSS and Javascript are loaded. The Bootstrap CSS does a lot of styling that may interfere with your existing style, so be prepared to change how you're doing CSS when you do this.

There are a couple of ways to do this:

### Using the GlobalTheme {} Component

There is a `GlobalTheme` component that will enable light or dark mode, and automatic switching (by default). It will also load the Bootstrap assets via CDN by default. If you want to load the assets manually or from somewhere else, you will need to disable this.

### Using the Dioxus.toml file in your project

```toml
# Additional CSS style files
style = [
    "https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css"
]

# Additional JavaScript files
script = [
    "https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js"
]
```

### Using the Dioxus interface in code

Within your App component, add this:

```rust
    rsx!{
        document::Link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css",
            integrity: "sha384-QWTKZyjpPEjISv5WaRU9OFeRpok6YctnYmDr5pNlyT2bRjXh0JMhjY6hW+ALEwIH",
            crossorigin: "anonymous"
        }
        document::Script {
            src: "https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js",
            integrity: "sha384-YvpcrYf0tY3lHB60NNkmXc5s9fDVZLESaAA55NDzOxhy9GkcIdslK1eN7N6jIeHz",
            crossorigin: "anonymous"
        }

    }
```

## Library State

Although this library is very early, I think it has several useful purposes. While it may be missing a lot of the ergonomic components that I hope to eventually create, there is nothing special about these components. You can mix and match the crate's components with other Bootstrap elements, which should work as you expect. I'm hoping that eventually, other people will make general-purpose components and contribute them back as well.

One of the more annoying things that I've figured out is automatically switching between dark and light mode based on system settings, which isn't totally obvious in Dioxus. That alone makes it worthwhile for me to use.

## Demo Application

This repository includes a comprehensive demo application that showcases all available components and their usage patterns.

### Running the Demo

```bash
# Clone the repository
git clone https://github.com/your-repo/dioxus-bootstrap
cd dioxus-bootstrap

# Run the web demo (recommended)
cd demo && dx serve --platform web

# Or run on desktop
cd demo && dx serve --platform desktop

# Or run on mobile
cd demo && dx serve --platform mobile
```

The demo application serves as both documentation and a testing environment. It demonstrates:
- All component variations and props
- Theme switching functionality
- Responsive behavior
- Integration patterns with dioxus-router
- Best practices for component composition

### Demo Structure

The demo is organized into several pages:
- **Home** - Overview and getting started
- **Components** - Interactive examples of all components
- **Layout** - Grid system and layout components
- **Forms** - Form components and validation
- **Navigation** - Navigation patterns and components
- **Utilities** - Utility classes and theme management

## Basic Usage Example

```rust
use dioxus::prelude::*;
use dioxus_bootstrap::prelude::*;

fn App() -> Element {
    rsx! {
        // Enable Bootstrap assets and theme switching
        GlobalTheme { theme: Theme::Auto }
        
        Container {
            Row {
                Col { md: 6,
                    Card {
                        CardHeader { "Welcome" }
                        CardBody {
                            "This is a Bootstrap card in Dioxus!"
                            Button {
                                variant: ButtonVariant::Primary,
                                "Click me"
                            }
                        }
                    }
                }
            }
        }
    }
}
```

## Development

### Building the Library

```bash
# Build the library
cargo build

# Run with release optimizations
cargo build --release
```

### Project Structure

- **`src/`** - Main library code with component modules
- **`demo/`** - Demo application showcasing component usage
- **`assets/`** - JavaScript files for theme switching functionality

### Contributing

This library follows Bootstrap 5.3 specifications and maintains compatibility with standard Bootstrap HTML/CSS. When adding components:

1. Follow existing patterns for props and styling
2. Ensure components work with both light and dark themes
3. Test in the demo application
4. Maintain interoperability with raw Bootstrap HTML

## More Information

- **Demo Application** - The best way to learn component usage and see examples
- **Bootstrap Documentation** - Official Bootstrap 5.3 docs for styling reference
- **Dioxus Documentation** - For general Dioxus framework information

## License

This project is licensed under the same terms as Bootstrap and Dioxus.

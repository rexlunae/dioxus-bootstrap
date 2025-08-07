# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is `dioxus-bootstrap`, an unofficial Bootstrap-based component library for Dioxus applications. It provides Bootstrap-styled components with automatic light/dark mode switching and CDN asset loading capabilities.

## Development Commands

### Building and Running
```bash
# Build the library
cargo build

# Run the demo application (primary way to test the library)
cd demo && dx serve --platform web

# Run demo on different platforms
cd demo && dx serve --platform desktop
cd demo && dx serve --platform mobile
```

### Testing
No formal test suite is configured. Testing is primarily done through the demo application.

## Architecture

### Core Library Structure (`src/`)
- **`lib.rs`**: Main module exports, re-exports all component modules
- **`theme.rs`**: Core theming system with `GlobalTheme` and `LocalTheme` components
- **Layout modules**: `container.rs` (Container, Row, Col), `card.rs`, `modal.rs`, `accordion.rs`
- **Navigation modules**: `nav.rs`, `sidebar.rs`, `tab.rs`, `breadcrumb.rs`, `pagination.rs`
- **Form modules**: `form.rs`, `button.rs`, `input.rs` (Input, Select, Checkbox, Radio, Textarea)
- **UI modules**: `alert.rs`, `badge.rs`, `toast.rs`, `dropdown.rs`, `list_group.rs`
- **Utility modules**: `size.rs`, `background.rs`, `utilities.rs`

### Key Components
- **`GlobalTheme`**: Automatically loads Bootstrap assets from CDN and handles system-level dark/light mode switching
- **`LocalTheme`**: Overrides theme for specific component subtrees
- **Layout Components**: Container, Row, Col with full responsive grid support, Card, Modal, Accordion
- **Navigation Components**: NavBar, SideBar, Tab systems, Breadcrumb, Pagination
- **Form Components**: Input, Select, Textarea, Checkbox, Radio, Button with full variants
- **UI Components**: Alert, Badge, Toast, Dropdown, ListGroup
- **Utility Components**: Comprehensive Bootstrap utility classes for spacing, display, flexbox, colors, etc.

### Demo Application (`demo/`)
- **`main.rs`**: Main app with routing setup, demonstrates component usage patterns
- **`views.rs`**: Page components showing library usage examples
- Serves as both documentation and testing environment

### Asset Management
- JavaScript files in `/assets/` handle theme switching: `auto_mode.js`, `dark_mode.js`, `light_mode.js`
- Bootstrap CSS/JS loaded via CDN by default through `GlobalTheme` component
- Can be disabled with `cdn_load_assets: false` prop

## Component Patterns

### Theme Integration
All components support Bootstrap's `data-bs-theme` attribute system for light/dark mode. Use `GlobalTheme` at app root and `LocalTheme` for local overrides.

### Size System
Components use consistent size enums from `size.rs` (`Size`, `ExtendedSize`) that map to Bootstrap size classes.

### Navigation Integration  
Components integrate with `dioxus-router` via `NavigationTarget` for consistent routing behavior.

## Development Workflow

1. Make changes to component modules in `src/`
2. Test changes in demo application: `cd demo && dx serve --platform web`
3. Components can be mixed with raw Bootstrap HTML - library is designed for interoperability
4. No formal linting/testing setup - rely on Rust compiler and demo testing

## Bootstrap Integration

- Uses Bootstrap 5.3.3 from CDN by default
- Components generate Bootstrap-compliant HTML with proper CSS classes
- Supports both automatic CDN loading and manual asset management
- Dark/light mode handled via Bootstrap's built-in theme system
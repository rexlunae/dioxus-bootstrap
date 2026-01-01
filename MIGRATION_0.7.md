# Migration Guide: Dioxus Bootstrap 0.7.0

This document details the changes made to update dioxus-bootstrap to support Dioxus 0.7.

## Version Information

- **Library Version**: 0.7.0 (aligns with Dioxus version for clarity)
- **Dioxus Version**: 0.7 (from 0.6.3)
- **Rust Edition**: 2024
- **Bootstrap Version**: 5.3.3 (unchanged)

## Breaking Changes

### 1. Dioxus Dependency Update

**Before:**

```toml
dioxus = { version = "0.6.3", features = ["fullstack"] }
dioxus-router = "0.6.3"
```

**After:**

```toml
dioxus = { version = "0.7", features = ["fullstack"] }
dioxus-router = "0.7"
```

### 2. Component Function Signatures

While component function signatures remain largely compatible, ensure you're using the latest Dioxus 0.7 patterns:

- Component functions use `#[component]` attribute macro
- Return type is `Element` (not `VNode` or `Fragment`)
- Props are defined using `#[derive(Props)]` with `Clone` and `PartialEq`

### 3. Event Handlers

Event handler patterns remain consistent, but ensure you're using the correct event types from `dioxus::prelude::*`.

## Code Changes Summary

### Library Changes (`src/`)

- ✅ All components are compatible with Dioxus 0.7
- ✅ No breaking changes to public APIs
- ✅ Removed unused import from `utilities.rs`
- ✅ All `Props` structs remain compatible with `#[props(optional)]` attributes

### Demo Application (`demo/`)

- ✅ Updated to Dioxus 0.7 dependencies
- ✅ All component usage patterns remain valid
- ✅ Router integration works with `dioxus-router` 0.7

## Testing

The library has been verified to compile cleanly with:

```bash
cargo check --lib
```

The demo application has been verified to compile with:

```bash
cargo check -p demo
```

Both build without errors or warnings (except the expected pre-existing ones).

## No Code Changes Required For Users

Most users won't need to change their code when updating to this version. The components maintain API stability while leveraging the improvements in Dioxus 0.7.

### Dioxus 0.7 improvements for your own code

1. **Better error messages** - The Dioxus compiler provides clearer error messages
2. **Improved performance** - Various rendering optimizations
3. **New hooks** - Access to new reactive primitives and hooks
4. **Better JSX syntax** - More ergonomic JSX patterns

## Resources

- [Dioxus 0.7 Release Notes](https://dioxuslabs.com/)
- [Dioxus Documentation](https://dioxuslabs.com/docs/0.7)
- [Bootstrap 5.3 Documentation](https://getbootstrap.com/docs/5.3/)

## Migration Checklist for Your Projects

If you're using dioxus-bootstrap 0.6.3 or earlier:

- [ ] Update `Cargo.toml` to use `dioxus-bootstrap` 0.7.0
- [ ] Ensure your project uses Dioxus 0.7
- [ ] Run `cargo check` to verify compatibility
- [ ] No code changes should be necessary

## Known Issues

None at this time.

## Future Improvements

- Consider adding more advanced components
- Performance optimizations leveraging Dioxus 0.7 features
- Enhanced accessibility features

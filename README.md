# Dioxus Bootstrap Components

This is an (unofficial) component library bringing Bootstrap-based components to the Dioxus stack.

> ⚠ This is very early work in progress. I am publishing it in case it is useful to anyone else, but please understand that it is expected to be wildly unstable for a while. ⚠

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

## More Information

The best way to see how to use this crate is to look at the demo application included.

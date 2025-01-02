function set_mode() {
    if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
        console.log("dark mode");
        document.getElementsByTagName("html")[0].setAttribute("data-bs-theme", "dark");

    }
    else {
        console.log("light mode");
        document.getElementsByTagName("html")[0].setAttribute("data-bs-theme", "light");
    }

}

set_mode();

// Watch for changes to the preferred mode.
window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', event => {
    const newColorScheme = event.matches ? "dark" : "light";
    set_mode();
});

use dioxus::prelude::*;

use dioxus_bootstrap::*;

mod views;

const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️
    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        GlobalTheme {}
        Container {
            size: ExtendedSize::Fluid,
            Router::<Route> {}
        }
    }
}

#[component]
pub fn Menu() -> Element {
    let path: Route = use_route();
    let menu1: NavigationTarget = Route::Page1{}.into();
    let menu2: NavigationTarget = Route::Page2{id:0}.into();
    rsx!{
        SideBar {
            SideBarMenu {
                SideBarMenuItem {active: path == Route::Page1{},link_to: Some(menu1), "Menu 1" }
                SideBarMenuItem {active: path == Route::Page2{id:0}, link_to: Some(menu2), "Menu 2" }
                SideBarMenuItem {active: false, "Menu 3"}
            }
        }
        Outlet::<Route> {}
    }
}

use crate::views::{Page1, Page2};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Menu)]
    #[route("/")]
    Page1 {},
    #[route("/page2/:id")]
    Page2 { id: i32 },
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}
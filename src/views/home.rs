use crate::components::counter::Counter;
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        p { "Hello, Dioxus and Haruki7049!!" }
        p { "This is an example-counter-app page." }
        a {
            href: "https://github.com/haruki7049/example-counter-app",
            "github.com/haruki7049/example-counter-app",
        }
        hr { }

        Counter { }
    }
}

use dioxus::prelude::*;

mod components;
mod backend;
use crate::components::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/main.css") }
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        document::Title { "Planner" }
        div {
            img { src: asset!("/assets/header.svg"), alt: "Planner Logo" }
            h1 { "Planner" }
        }
        div {
            class: "inbox-app-container",
            InboxApp {}
        }
    }
}

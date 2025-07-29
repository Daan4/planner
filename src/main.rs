use dioxus::prelude::*;

mod components;
mod backend;
use crate::components::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        document::Stylesheet { href: asset!("/assets/theme.css") }
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        document::Title { "Planner" }
        div {
            class: "flex",
            InboxApp {}
            ScheduleApp {}
        }
    }
}

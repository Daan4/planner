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
            class: "grid grid-cols-3 gap-4 p-4 h-screen",
            
            // Left column: Inbox
            div {
                class: "col-span-1 overflow-y-auto",
                InboxApp {}
            }

            // Right column: vertical layout with Schedule on top, Backlog below
            div {
                class: "col-span-2 flex flex-col min-h-0",
                
                // Schedule takes all available space
                div {
                    class: "flex-grow min-h-0 overflow-y-auto",
                    ScheduleApp {}
                }

                // Backlog fits its content
                div {
                    class: "overflow-y-auto",
                    BacklogApp {}
                }
            }
        }
    }
}

use dioxus::prelude::*;
use super::item::ItemList;

#[component]
pub fn InboxApp() -> Element {
    rsx! { 
        document::Stylesheet { href: asset!("/assets/inbox.css") }
        div {
            class: "flex-1 border border-gray-400 bg-gray-100 text-center p-4",
            "Inbox",
            ItemList { day: None } 
        }
    }
}

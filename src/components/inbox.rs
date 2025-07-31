use dioxus::prelude::*;
use super::item::ItemList;

#[component]
pub fn InboxApp() -> Element {
    rsx! { 
        document::Stylesheet { href: asset!("/assets/inbox.css") }
        ItemList { day: None } 
    }
}

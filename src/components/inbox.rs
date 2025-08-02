use dioxus::prelude::*;
use super::item::ItemList;
use crate::backend::model::TaskFilter;

#[component]
pub fn InboxApp() -> Element {
    rsx! { 
        document::Stylesheet { href: asset!("/assets/inbox.css") }
        div {
            class: "flex-1 border border-gray-400 bg-gray-100 text-center p-4",
            "Inbox",
            ItemList { filter: TaskFilter {scheduled_date: None, backlog_id: None} } 
        }
    }
}

use dioxus::prelude::*;
use super::item::ItemList;
use crate::backend::server;
use crate::backend::model::{Backlog, TaskFilter};
use dioxus_primitives::dialog::{DialogContent, DialogDescription, DialogRoot, DialogTitle};

#[component]
pub fn BacklogApp() -> Element {
    let mut dialog_open = use_signal(|| false);
    let mut backlogs = use_signal(|| vec![]);

    use_hook(|| {
        spawn(async move {
            match server::get_backlogs().await {
                Ok(fetched) => backlogs.set(fetched),
                Err(e) => eprintln!("Failed to fetch backlogs: {}", e),
            }
        });
    });

    rsx! { 
        div {
            class: "flex-1 border border-gray-400 bg-gray-100 text-center p-4",
            "Backlog",
            document::Stylesheet { href: asset!("/assets/dialog.css") }
            document::Stylesheet { href: asset!("/assets/button.css") }
            button {
                class: "button",
                "data-style": "outline",
                style: "margin-bottom: 1.5rem;",
                onclick: move |_| dialog_open.set(true),
                "Manage Backlogs"
            }
            BacklogList { backlogs: backlogs.read().clone() },
            DialogRoot {
                class: "dialog-backdrop",
                open: dialog_open(),
                on_open_change: move |v| dialog_open.set(v),
                DialogContent {
                    class: "dialog",
                    button {
                        class: "dialog-close",
                        aria_label: "Close",
                        tabindex: if dialog_open() { "0" } else { "-1" },
                        onclick: move |_| dialog_open.set(false),
                        "×"
                    }
                    BacklogManager { }
                }
            }
        }
    }
}

#[component]
pub fn BacklogList(backlogs: Vec<Backlog>) -> Element {
    rsx! {
        for backlog in backlogs {
            BacklogBoard {
                key: "{backlog.id.0}",
                backlog: backlog.clone(),
            }
        }
    }
}

#[component]
pub fn BacklogBoard(backlog: Backlog) -> Element {
    rsx! { 
        div {
            class: "flex-1 border border-gray-400 bg-gray-100 text-center p-4",
            "{backlog.name}",
            ItemList { filter: TaskFilter {scheduled_date: None, backlog_id: Some(backlog.id)} } 
        }
    }
}

#[component]
pub fn BacklogManager() -> Element {
    let mut new_backlog = use_signal(|| String::new());

    let create_backlog_fn = {
        move || {
            let name = new_backlog.read().clone();
            if name.is_empty() {
                return;
            }
            spawn({
                async move {
                    match server::create_backlog(name).await {
                        Ok(backlog) => {
                            //backlogs.push(task);
                            new_backlog.set(String::new());
                        },
                        Err(e) => eprintln!("Failed to create backlog: {}", e),
                    }
                }
            });
        }
    };

    // let delete_backlog_fn = {
    //     move |id: Id| {
    //         spawn({
    //             async move {
    //                 match server::delete_backlog(id).await {
    //                     Ok(_) => backlogs.retain(|b| b.id != id),
    //                     Err(e) => eprintln!("Failed to delete backlog: {}", e),
    //                 }
    //             }
    //         });
    //     }
    // };

    rsx! {
        DialogTitle {
            class: "dialog-title",
            "Backlog Management"
        }
        DialogDescription {
            input {
                class: "inbox-input flex-1",
                r#type: "text",
                placeholder: "Backlog name",
                value: "{new_backlog}",
                oninput: move |evt| new_backlog.set(evt.value()),
                onkeydown: move |evt| match evt.key() {
                    Key::Enter => create_backlog_fn(),
                    Key::Escape => new_backlog.set(String::new()),
                    _ => {}
                }
            }
            button { 
                class: "inbox-button",
                onclick: move |_| create_backlog_fn(), "Create Backlog" 
            }
            // button {
            //     class: "inbox-button",
            //     onclick: move |_| delete_backlog_fn(), "Delete Backlog"
            // }
        }
    }
}

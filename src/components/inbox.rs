use dioxus::prelude::*;
use crate::backend::server;
use crate::backend::model::{Task, Id};

#[component]
pub fn InboxApp() -> Element {
    rsx! {
        InboxItemList { }
    }
}

#[component]
fn InboxItemList() -> Element {
    let mut new_task_content = use_signal(|| String::from(""));
    let mut tasks: Signal<Vec<Task>> = use_signal(|| vec![]);

    use_hook(|| {
        spawn(async move {
            if let Ok(fetched) = server::get_tasks().await {
                tasks.set(fetched);
            }
        });
    });

    let create_task = move || async move {
        let content = new_task_content.read().clone();
        if content.is_empty() {
            return;
        }

        if let Ok(task) = server::create_task(content).await {
            tasks.write().push(task);
            new_task_content.set(String::from(""));
        }
    };

    let delete_task = move |id: Id| {
        spawn(async move {
            match server::delete_task(id).await {
                Ok(_) => {
                    tasks.write().retain(|task| task.id != id);
                }
                Err(e) => eprintln!("Failed to delete task: {}", e),
            }
        });
    };

    rsx! {
        div {
            class: "inbox-component",

            div { id: "title",
                h2 { "Inbox" }
            }

            div { id: "inbox-entry",
                input {
                    r#type: "text",
                    placeholder: "Enter a task",
                    value: "{new_task_content}",
                    oninput: move |evt| new_task_content.set(evt.value()),
                    onkeydown: move |evt| {
                        if evt.key() == Key::Enter {
                            spawn(create_task());
                        } else if evt.key() == Key::Escape {
                            new_task_content.set(String::from(""));
                        }
                    }
                }

                button {
                    onclick: move |_| create_task(),
                    "Add"
                }
            }

            div { id: "inbox-items",
                for task in tasks.read().iter() {
                    InboxItem {
                        key: "{task.id.0}",
                        task: task.clone(),
                        on_delete: move |evt| delete_task(evt),
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Clone)]
enum ItemState {
    Normal,
    Hovered,
    Selected,
}

#[component]
fn InboxItem(task: Task, on_delete: EventHandler<Id>) -> Element {
    let mut state = use_signal(|| ItemState::Normal);
    let mut disabled = use_signal(|| true);
    let mut old_content = use_signal(|| task.content.clone());
    let mut content = use_signal(|| task.content.clone());

    use_effect(move || {
        content.set(task.content.clone());
    });

    let update_task = move || {
        spawn(async move {
            server::update_task(task.id, content.read().clone()).await.unwrap(); // TODO: handle error
        });
    };

    rsx! {
        div {
            input {
                r#type: "text",
                class: {
                    match *state.read() {
                        ItemState::Normal => "inbox-item",
                        ItemState::Hovered => "inbox-item hovered",
                        ItemState::Selected => "inbox-item selected",
                    }
                },
                disabled: "{disabled}",
                value: "{content}",
                onmouseenter: move |_| {
                    if state.read().clone() != ItemState::Selected {
                        state.set(ItemState::Hovered);
                        disabled.set(false);
                    }
                },
                onmouseleave: move |_| {
                    if state.read().clone() != ItemState::Selected {
                        state.set(ItemState::Normal);
                        disabled.set(true);
                    }
                },
                onclick: move |_| {
                    state.set(ItemState::Selected);
                    old_content.set(content.read().clone());
                },
                oninput: move |evt| content.set(evt.value()),
                onfocusout: move |_| {
                    if content.read().is_empty() {
                        on_delete.call(task.id);
                    } else if content.read().clone() != old_content.read().clone() {
                        update_task();
                    }
                    state.set(ItemState::Normal);
                    disabled.set(true);
                },
                onkeydown: move |evt| {
                    match evt.key() {
                        Key::Enter => {
                            update_task();
                            state.set(ItemState::Normal);
                            disabled.set(true);
                        },
                        Key::Escape => {
                            state.set(ItemState::Normal);
                            disabled.set(true);
                            content.set(old_content.read().clone());
                        },
                        Key::Delete => {
                            on_delete.call(task.id);
                        },
                        _ => {}
                    }
                }
            }
        }
    }
}

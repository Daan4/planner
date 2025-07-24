use dioxus::prelude::*;
use crate::backend::server;
use crate::backend::model::{Task, Id};

#[component]
pub fn InboxApp() -> Element {
    rsx! { InboxItemList {} }
}

#[component]
fn InboxItemList() -> Element {
    let mut new_task = use_signal(|| String::new());
    let mut tasks: Signal<Vec<Task>> = use_signal(|| vec![]);

    use_hook(|| {
        spawn(async move {
            if let Ok(fetched) = server::get_tasks().await {
                tasks.set(fetched);
            }
        });
    });

    let create_task = {
        move || {
            let content = new_task.read().clone();
            if content.is_empty() {
                return;
            }
            spawn({
                async move {
                    if let Ok(task) = server::create_task(content).await {
                        tasks.write().push(task);
                        new_task.set(String::new());
                    }
                }
            });
        }
    };

    let delete_task = {
        let mut tasks = tasks.clone();
        move |id: Id| {
            spawn({
                async move {
                    if server::delete_task(id).await.is_ok() {
                        tasks.write().retain(|t| t.id != id);
                    }
                }
            });
        }
    };

    rsx! {
        div {
            class: "inbox-component",

            h2 { "Inbox" }

            div {
                input {
                    r#type: "text",
                    placeholder: "Enter a task",
                    value: "{new_task}",
                    oninput: move |evt| new_task.set(evt.value()),
                    onkeydown: move |evt| match evt.key() {
                        Key::Enter => create_task(),
                        Key::Escape => new_task.set(String::new()),
                        _ => {}
                    }
                }
                button { onclick: move |_| create_task(), "Add" }
            }

            div {
                for task in tasks.read().clone().iter() {
                    InboxItem {
                        key: "{task.id.0}",
                        task: task.clone(),
                        on_delete: delete_task.clone()
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
    let mut content = use_signal(|| task.content.clone());
    let mut old_content = use_signal(|| task.content.clone());

    use_effect(move || {
        content.set(task.content.clone());
    });

    let update_task = move || {
        spawn(async move {
            server::update_task(task.id, content.read().clone()).await.unwrap(); // TODO: handle error
        });
    };

    let apply_state_class = || match state.read().clone() {
        ItemState::Normal => "inbox-item",
        ItemState::Hovered => "inbox-item hovered",
        ItemState::Selected => "inbox-item selected",
    };

    rsx! {
        input {
            r#type: "text",
            class: "{apply_state_class()}",
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
                if content.read().clone().is_empty() {
                    on_delete.call(task.id);
                } else if content.read().clone() != old_content.read().clone() {
                    update_task();
                }
                state.set(ItemState::Normal);
                disabled.set(true);
            },
            onkeydown: move |evt| match evt.key() {
                Key::Enter => {
                    update_task();
                    state.set(ItemState::Normal);
                    disabled.set(true);
                },
                Key::Escape => {
                    content.set(old_content.read().clone());
                    state.set(ItemState::Normal);
                    disabled.set(true);
                },
                Key::Delete => on_delete.call(task.id),
                _ => {}
            }
        }
    }
}

use dioxus::prelude::*;
use std::sync::{Arc, Mutex};
use crate::backend::server;
use crate::backend::model::{Task, Id};

type DeleteTaskFn = Arc<Mutex<dyn FnMut(Id)>>;

#[derive(Clone)]
struct InboxItemListContext {
    delete_task_fn: Signal<DeleteTaskFn>,
}

#[component]
pub fn InboxApp() -> Element {
    // State to store the tasks
    let tasks: Signal<Vec<Task>> = use_signal(|| vec![]);

    // Fetch tasks once (only when the component is mounted)
    use_hook(|| {
        // Clone signal into the hook's closure
        let mut tasks = tasks.clone();
        spawn(async move {
            tasks.set(server::get_tasks().await.unwrap()); // Todo Handle error
        });
    });

    rsx! {
        InboxItemList { tasks: tasks.read().clone() }
    }
}

#[component]
fn InboxItemList(tasks: Vec<Task>) -> Element {
   let mut new_task_content = use_signal(|| String::from(""));
    let tasks: Signal<Vec<Task>> = use_signal(|| tasks);

    let create_task = move || async move {
        let mut new_task_content = new_task_content.clone();
        let mut tasks = tasks.clone();
        if !new_task_content.read().is_empty() {
            let task = server::create_task(new_task_content.read().clone()).await.unwrap(); // Todo Handle error
            tasks.write().push(task);
            new_task_content.set(String::from(""));
        }
    };

    // Provide context allowing child items to delete themselves
    let delete_task_fn = {
        let tasks = tasks.clone();
        move |task_id: Id| {
            let mut tasks = tasks.clone();
            spawn(async move {
                server::delete_task(task_id).await.unwrap();
                tasks.write().retain(|t| t.id != task_id);
            });
        }
    };
    provide_context(InboxItemListContext {
        delete_task_fn: Signal::new(Arc::new(Mutex::new(delete_task_fn))),
    });

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
                    id: "inbox-add-button",
                    onclick: move |_| create_task(),
                    "Add"
                }
            }

            div { id: "inbox-items",
                for task in tasks.read().iter() {
                    InboxItem { task: task.clone() }
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
fn InboxItem(task: Task) -> Element {
    let mut state = use_signal(|| ItemState::Normal);
    let mut disabled = use_signal(|| true);
    let mut old_content = use_signal(|| task.content.clone());
    let mut content = use_signal(|| task.content.clone());
    let context = use_context::<InboxItemListContext>();
    let delete_task_fn = context.delete_task_fn.read().clone();

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
                    state.set(ItemState::Normal);
                    disabled.set(true);
                },
                onkeydown: move |evt| {
                    if evt.key() == Key::Enter {
                        spawn(async move {
                            server::update_task(task.id, content.read().clone()).await.unwrap(); // Todo: Handle error properly
                        });
                        state.set(ItemState::Normal);
                        disabled.set(true);
                    } else if evt.key() == Key::Escape {
                        state.set(ItemState::Normal);
                        disabled.set(true);
                        content.set(old_content.read().clone());
                    } else if evt.key() == Key::Delete {
                        spawn(async move {
                            server::delete_task(task.id).await.unwrap(); // Todo: Handle error properly
                        });
                       delete_task_fn.lock().unwrap()(task.id);
                    }
                }
            }
        }
    }
}

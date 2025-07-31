use dioxus::prelude::*;
use crate::backend::server;
use chrono::NaiveDate;
use crate::backend::model::{Task, Id};

#[component]
pub fn ItemList(day: Option<NaiveDate>) -> Element {
    let mut new_task = use_signal(|| String::new());
    let mut tasks = use_signal(|| vec![]);
    use_hook(|| {
        spawn(async move {
            match server::get_tasks(day).await {
                Ok(fetched) => tasks.set(fetched),
                Err(e) => eprintln!("Failed to fetch tasks: {}", e),
            }
        });
    });

    let create_task_fn = {
        move || {
            let title = new_task.read().clone();
            if title.is_empty() {
                return;
            }
            spawn({
                async move {
                    match server::create_task(title, day).await {
                        Ok(task) => {
                            tasks.write().push(task);
                            new_task.set(String::new());
                        },
                        Err(e) => eprintln!("Failed to create task: {}", e),
                    }
                }
            });
        }
    };

    let delete_task_fn = {
        let mut tasks = tasks.clone();
        move |id: Id| {
            spawn({
                async move {
                    match server::delete_task(id).await {
                        Ok(_) => tasks.write().retain(|t| t.id != id),
                        Err(e) => eprintln!("Failed to delete task: {}", e),
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
                class: "flex",
                input {
                    class: "inbox-input flex-1",
                    r#type: "text",
                    placeholder: "Enter a task",
                    value: "{new_task}",
                    oninput: move |evt| new_task.set(evt.value()),
                    onkeydown: move |evt| match evt.key() {
                        Key::Enter => create_task_fn(),
                        Key::Escape => new_task.set(String::new()),
                        _ => {}
                    }
                }
                button { 
                    class: "inbox-button",
                    onclick: move |_| create_task_fn(), "Add" 
                }
            }

            div {
                for task in tasks.read().clone().iter() {
                    ScheduleItem {
                        key: "{task.id.0}",
                        task: task.clone(),
                        on_delete: delete_task_fn.clone()
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
fn ScheduleItem(task: Task, on_delete: EventHandler<Id>) -> Element {
    let mut state = use_signal(|| ItemState::Normal);
    let mut disabled = use_signal(|| true);
    let mut title = use_signal(|| task.title.clone());
    let mut old_title = use_signal(|| task.title.clone());

    use_effect(move || {
        title.set(task.title.clone());
    });

    let update_task = move || {
        spawn(async move {
            match server::update_task(task.id, title.read().clone()).await {
                Ok(_) => {},
                Err(e) => eprintln!("Failed to update task {}: {}", task.id.0, e),
            }
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
            value: "{title}",

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
                old_title.set(title.read().clone());
            },
            oninput: move |evt| title.set(evt.value()),
            onblur: move |_| {
                if title.read().clone().is_empty() {
                    on_delete.call(task.id);
                } else if title.read().clone() != old_title.read().clone() {
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
                    title.set(old_title.read().clone());
                    state.set(ItemState::Normal);
                    disabled.set(true);
                },
                Key::Delete => on_delete.call(task.id),
                _ => {}
            }
        }
    }
}
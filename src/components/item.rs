use dioxus::prelude::*;
use crate::backend::server;
use chrono::NaiveDate;
use crate::backend::model::{Task, Id};
use std::sync::Mutex;

static DRAGGING_ITEM: GlobalSignal<Option<Task>> = Signal::global(|| None);
static DROPPED_ITEM: GlobalSignal<Mutex<Option<Task>>> = Signal::global(|| Mutex::new(None));

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

    let update_task_fn = move |task: Task| {
        spawn(async move {
            let id = task.id.0;
            match server::update_task(task).await {
                Ok(_) => {},
                Err(e) => eprintln!("Failed to update task {}: {}", id, e),
            }
        });
    };

    // Only visually removes the task from the list, used to hide an item after dragging
    let remove_task_fn = move |id: Id| {
        tasks.write().retain(|t| t.id != id);
    };

    rsx! {
        div {
            class: "inbox-component",

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
                class: "p-4 border border-zinc-700 rounded w-1/2 bg-zinc-800 shadow-md",
                ondragover: move |e| e.prevent_default(),
                ondrop: move |_| {
                    if let Some(mut task) = DRAGGING_ITEM.read().clone() {
                        if task.scheduled_date != day {
                            task.scheduled_date = day;
                            let t = task.clone();
                            spawn(async move {server::update_task(t).await.unwrap();});
                        }
                        tasks.write().push(task.clone());
                        *DRAGGING_ITEM.write() = Some(task.clone());
                    }
                },
                for task in tasks.read().clone().iter() {
                    Item {
                        key: "{task.id.0}",
                        task: task.clone(),
                        on_delete: delete_task_fn.clone(),
                        on_update: update_task_fn.clone(),
                        on_remove: remove_task_fn.clone(),
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
fn Item(task: Task, on_delete: EventHandler<Id>, on_update: EventHandler<Task>, on_remove: EventHandler<Id>) -> Element {
    let mut state = use_signal(|| ItemState::Normal);
    let mut disabled = use_signal(|| true);
    let mut title = use_signal(|| task.title.clone());
    let mut old_title = use_signal(|| task.title.clone());

    // Prevent moving task into closure
    let t = task.clone();
    use_effect(move || {
        title.set(t.title.clone());
    });

    use_effect(move || {
        if let Some(t) = DROPPED_ITEM.read().lock().unwrap().clone() {
            if t.id == task.id {
                on_remove.call(t.id);
            }
        }
    });

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
            draggable: "true",

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
            onblur: {
                let mut task = task.clone();
                move |_| {
                    if title.read().clone().is_empty() {
                        on_delete.call(task.id);
                    } else if title.read().clone() != old_title.read().clone() {
                        task.title = title.read().clone();
                        on_update.call(task.clone());
                    }
                    state.set(ItemState::Normal);
                    disabled.set(true);
                }
            },
            onkeydown: {
                let mut task = task.clone();
                move |evt| match evt.key() {
                Key::Enter => {
                    task.title = title.read().clone();
                    on_update.call(task.clone());
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
            }},
            ondragstart: move |_| {
                *DRAGGING_ITEM.write() = Some(task.clone());
            }
        }
    }
}
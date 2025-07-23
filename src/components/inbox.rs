use dioxus::prelude::*;

#[component]
pub fn InboxApp() -> Element {
    let mut new_task = use_signal(|| String::from(""));
    let mut tasks: Signal<Vec<String>> = use_signal(|| Vec::new());

    let mut add = move || {
        if !new_task.read().is_empty() {
            let mut msgs = tasks.write();
            msgs.push(new_task.read().clone());
            new_task.set(String::from(""));
        }
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
                    value: "{new_task}",
                    oninput: move |evt| new_task.set(evt.value()),
                    onkeydown: move |evt| {
                        if evt.key() == Key::Enter {
                            add();
                        }
                    }
                }

                button {
                    id: "inbox-add-button",
                    onclick: move |_| add(),
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
fn InboxItem(task: String) -> Element {
    let mut state = use_signal(|| ItemState::Normal);
    let mut disabled = use_signal(|| true);
    let mut old_task = use_signal(|| task.clone());
    let mut task = use_signal(|| task.clone());

    rsx! {
        div {
            input {
                r#type: "text",
                // Dynamic class based on state
                class: {
                    match *state.read() {
                        ItemState::Normal => "inbox-item",
                        ItemState::Hovered => "inbox-item hovered",
                        ItemState::Selected => "inbox-item selected",
                    }
                },
                disabled: "{disabled}",
                value: "{task}",
                onmouseenter: move |_| {
                    state.set(ItemState::Hovered);
                    disabled.set(false);
                },
                onmouseleave: move |_| {
                    if state.read().clone() != ItemState::Selected {
                        state.set(ItemState::Normal);
                        disabled.set(true);
                    }
                },
                onclick: move |_| {
                    state.set(ItemState::Selected);
                    old_task.set(task.read().clone());
                },
                oninput: move |evt| task.set(evt.value()),
                onfocusout: move |_| {
                    state.set(ItemState::Normal);
                    disabled.set(true);
                },
                onkeydown: move |evt| {
                    if evt.key() == Key::Enter {
                        state.set(ItemState::Normal);
                        disabled.set(true);
                    } else if evt.key() == Key::Escape {
                        state.set(ItemState::Normal);
                        disabled.set(true);
                        task.set(old_task.read().clone());
                    } else if evt.key() == Key::Delete {
                        task.set(String::from(""));
                        state.set(ItemState::Normal);
                        disabled.set(true);
                    }
                }
            }
        }
    }
}

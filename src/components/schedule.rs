use dioxus::prelude::*;
use dioxus_primitives::calendar::{
    Calendar, CalendarContext, CalendarDate, CalendarGrid, CalendarHeader, CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton
};
use chrono::{Datelike, Utc, NaiveDate, Weekday, Days};
use crate::backend::server;
use crate::backend::model::{Task, Id};

#[component]
pub fn ScheduleApp() -> Element {
    let mut selected_date: Signal<NaiveDate> = use_signal(|| Utc::now().naive_local().date());
    let week_nr = use_memo(move || selected_date.read().iso_week().week());

    rsx! {
        document::Stylesheet { href: asset!("/assets/schedule.css") }

        div {
            class: "p-4 flex flex-col gap-4",

            div {
                class: "w-fit", // prevents stretching
                DatePicker {
                    on_selection_change: move |date| selected_date.set(date)
                },
                "Week {week_nr}",
            }

            div {
                WeeklySchedule {
                    selected_date: selected_date()
                }
            }
        }
    }
}

#[component]
fn WeeklySchedule(selected_date: NaiveDate) -> Element {
    let first_weekday = selected_date.week(Weekday::Mon).first_day();

    rsx! {
        div {
            class: "flex gap-2 justify-between",
            for i in 0..7 {
                let d = first_weekday.checked_add_days(Days::new(i)).unwrap();
                DailySchedule {
                    key: "{d}",
                    day: d}
            }
        }
    }
}

#[component]
fn DailySchedule(day: NaiveDate) -> Element {
    rsx! {
        div {
            class: "flex-1 border border-gray-400 bg-gray-100 text-center p-4",
            "Schedule for {day.weekday()} {day}",
            ScheduleItemList{ day }
        }
    }}


#[component]
fn ScheduleItemList(day: NaiveDate) -> Element {
    let mut new_task = use_signal(|| String::new());
    let mut tasks = use_signal(|| vec![]);
    use_hook(|| {
        spawn(async move {
            match server::get_tasks(Some(day)).await {
                Ok(fetched) => tasks.set(fetched),
                Err(e) => eprintln!("Failed to fetch tasks: {}", e),
            }
        });
    });

    let create_task = {
        move || {
            let title = new_task.read().clone();
            if title.is_empty() {
                return;
            }
            spawn({
                async move {
                    match server::create_task(title, Some(day)).await {
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

    let delete_task = {
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
                        Key::Enter => create_task(),
                        Key::Escape => new_task.set(String::new()),
                        _ => {}
                    }
                }
                button { 
                    class: "inbox-button",
                    onclick: move |_| create_task(), "Add" 
                }
            }

            div {
                for task in tasks.read().clone().iter() {
                    ScheduleItem {
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

#[component]
fn DatePicker(on_selection_change: EventHandler<NaiveDate>) -> Element {
    let mut view_date = use_signal(|| {
        let today = Utc::now().date_naive();
        let year: i32 = today.year() as i32;
        let month: u32 = today.month() as u32;
        let day: u32 = today.day() as u32;   
        CalendarDate::new(year, month, day)
    });
    let mut selected_date: Signal<Option<CalendarDate>> = use_signal(|| {
        let today = Utc::now().date_naive();
        let year: i32 = today.year() as i32;
        let month: u32 = today.month() as u32;
        let day: u32 = today.day() as u32;
        Some(CalendarDate::new(year, month, day))
    });

     rsx! {
        div { class: "calendar-example", style: "padding: 20px;",
            div { class: "calendar",
                Calendar {
                    selected_date: selected_date(),
                    on_date_change: move |date| {
                        selected_date.set(date);
                        match date 
                        {
                            Some(date) => {
                                on_selection_change.call(NaiveDate::from_ymd_opt(date.year, date.month, date.day).unwrap())
                            },
                            None => (),
                        }
                    },
                    view_date: view_date(),
                    on_view_change: move |new_view: CalendarDate| {
                        view_date.set(new_view);
                    },
                    CalendarHeader {
                        CalendarNavigation {
                            CalendarPreviousMonthButton {
                                svg {
                                    class: "calendar-previous-month-icon",
                                    view_box: "0 0 24 24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    polyline { points: "15 6 9 12 15 18" }
                                }
                            }
                            MonthTitle {}
                            CalendarNextMonthButton {
                                svg {
                                    class: "calendar-next-month-icon",
                                    view_box: "0 0 24 24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    polyline { points: "9 18 15 12 9 6" }
                                }
                            }
                        }
                    }
                    CalendarGrid {}
                }
            }
        }
    }
}

#[component]
fn MonthTitle() -> Element {
    let calendar: CalendarContext = use_context();
    rsx! {
        select {
            class: "calendar-month-select",
            aria_label: "Month",
            onchange: move |e| {
                let mut view_date = calendar.view_date();
                view_date.month = e.value().parse().unwrap_or(view_date.month);
                calendar.set_view_date(view_date);
            },
            for (i, month) in CalendarDate::MONTH_ABBREVIATIONS.iter().enumerate() {
                option {
                    value: i + 1,
                    selected: calendar.view_date().month == (i as u32 + 1),
                    "{month}"
                }
            }
        }

        select {
            class: "calendar-year-select",
            aria_label: "Year",
            onchange: move |e| {
                let mut view_date = calendar.view_date();
                view_date.year = e.value().parse().unwrap_or(view_date.year);
                calendar.set_view_date(view_date);
            },
            for year in 1925..=2050 {
                option {
                    value: year,
                    selected: calendar.view_date().year == year,
                    "{year}"
                }
            }
        }
    }
}
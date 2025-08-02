use dioxus::prelude::*;

use chrono::{Datelike, Utc, NaiveDate, Weekday, Days};
use super::item::ItemList;
use crate::backend::model::TaskFilter;
use super::calendar::DatePicker;

#[component]
pub fn ScheduleApp() -> Element {
    let mut selected_date: Signal<NaiveDate> = use_signal(|| Utc::now().naive_local().date());
    let week_nr = use_memo(move || selected_date.read().iso_week().week());

    rsx! {
        document::Stylesheet { href: asset!("/assets/calendar.css") }

        div {
            class: "p-4 flex flex-col gap-4",

            div {
                class: "w-fit", // prevents stretching
                "Week {week_nr}",
                DatePicker {
                    on_selection_change: move |date| selected_date.set(date)
                },
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
                DailySchedule {
                    key: "{first_weekday.checked_add_days(Days::new(i)).unwrap()}",
                    day: first_weekday.checked_add_days(Days::new(i)).unwrap()}
            }
        }
    }
}

#[component]
fn DailySchedule(day: NaiveDate) -> Element {
    rsx! {
        div {
            class: "flex-1 border border-gray-400 bg-gray-100 text-center p-4",
            "{day.weekday()} {day}",
            ItemList{ filter: TaskFilter {scheduled_date: Some(day), backlog_id: None} }
        }
    }}
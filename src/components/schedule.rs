use dioxus::prelude::*;
use dioxus_primitives::calendar::{
    Calendar, CalendarContext, CalendarDate, CalendarGrid, CalendarHeader, CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton
};
use chrono::{Datelike, Utc};

#[component]
pub fn ScheduleApp() -> Element {

    rsx! {
        div {
            class: "flex flex-col gap-4 p-4",
            document::Stylesheet { href: asset!("/assets/schedule.css") }
            div {
                DatePicker {
                    }
            }
            div {
                WeeklySchedule {
                }
            }
        }
    }
}

#[component]
fn WeeklySchedule() -> Element {
    rsx! {
        div {
            class: "flex gap-2 justify-between",
            //"Week Number {selected_date}",
            DailySchedule {}
            DailySchedule {}
            DailySchedule {},
            DailySchedule {},
            DailySchedule {},
            DailySchedule {},
            DailySchedule {},
        }
    }
}

#[component]
fn DailySchedule() -> Element {
    rsx! {
        div {
            class: "flex-1 border border-gray-400 bg-gray-100 text-center p-4",
            "Schedule for today",
        }
    }
}

#[component]
fn DatePicker() -> Element {
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

use dioxus::prelude::*;
use dioxus_primitives::calendar::{
    Calendar, CalendarContext, CalendarDate, CalendarGrid, CalendarHeader, CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton
};
use chrono::{Datelike, Utc, NaiveDate, Weekday, Days};

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
                DailySchedule {day: first_weekday.checked_add_days(Days::new(i)).unwrap()}
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

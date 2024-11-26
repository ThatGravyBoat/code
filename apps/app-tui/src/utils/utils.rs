use crate::pages::notifications::notification_handler::NotificationHandler;
use crate::utils::ui::DANGER;
use chrono::{DateTime, Local, Utc};
use ratatui::prelude::*;
use std::future::Future;
use std::time::Duration;
use theseus::*;
use tokio::runtime::Runtime;

pub const LETTERS_NUMBERS: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y', 'z'
];


pub fn get_next<T, F>(
    vec: &Vec<T>,
    predicate: F,
) -> &T where F: Fn(&T) -> bool {
    if let Some(position) = vec.iter().position(|it| predicate(it)) {
        if position < vec.len() - 1 {
            return &vec[position + 1]
        }
    }
    &vec[vec.len() - 1]
}

pub fn get_last<T, F>(
    vec: &Vec<T>,
    predicate: F,
) -> &T where F: Fn(&T) -> bool {
    if let Some(position) = vec.iter().position(|it| predicate(it)) {
        if position > 0 {
            return &vec[position - 1]
        }
    }
    &vec[0]
}

pub fn run_or_notify<R, F: Future<Output = Result<R>>, M>(
    runtime: &Runtime,
    future: F,
    error_mapper: M
) -> Option<R> where M: Fn(&ErrorKind) -> String {
    match runtime.block_on(future) {
        Err(error) => {
            NotificationHandler::push(
                "Error Occurred".to_string(),
                Some(error_mapper(error.raw.as_ref()).to_string()),
                (Color::White, DANGER),
                Duration::from_secs(5)
            );
            None
        }
        Ok(value) => Some(value)
    }
}

pub fn format_elapsed_time(date: DateTime<Utc>) -> String {
    let elapsed = Local::now().to_utc() - date;
    if elapsed.num_weeks() >= 52 {
        let years = elapsed.num_weeks() / 52;
        if years > 1 { format!("{} years ago", years) } else { "a year ago".to_string() }
    } else if elapsed.num_weeks() >= 4 {
        let months = elapsed.num_weeks() / 4;
        if months > 1 { format!("{} months ago", months) } else { "a month ago".to_string() }
    } else if elapsed.num_weeks() >= 1 {
        let weeks = elapsed.num_weeks();
        if weeks > 1 { format!("{} weeks ago", weeks) } else { "a week ago".to_string() }
    } else if elapsed.num_days() >= 1 {
        let days = elapsed.num_days();
        if days > 1 { format!("{} days ago", days) } else { "a day ago".to_string() }
    } else if elapsed.num_hours() >= 1 {
        let hours = elapsed.num_hours();
        if hours > 1 { format!("{} hours ago", hours) } else { "an hour ago".to_string() }
    } else if elapsed.num_minutes() >= 1 {
        let minutes = elapsed.num_minutes();
        if minutes > 1 { format!("{} minutes ago", minutes) } else { "a minute ago".to_string() }
    } else if elapsed.num_seconds() >= 1 {
        let seconds = elapsed.num_seconds();
        if seconds > 1 { format!("{} seconds age", seconds) } else { "a second ago".to_string() }
    } else {
        "now".to_string()
    }
}
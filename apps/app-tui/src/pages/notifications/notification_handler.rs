use ratatui::style::Color;
use std::sync::{Arc, LazyLock, Mutex};
use std::time::{Duration, Instant};
use std::vec::IntoIter;

static NOTIFICATIONS: LazyLock<Arc<Mutex<NotificationHandler>>> = LazyLock::new(|| Arc::new(Mutex::new(NotificationHandler::default())));

#[derive(Clone)]
pub struct Notification {
    pub title: String,
    pub text: Option<String>,
    pub color: (Color, Color),
    duration: Duration,
    added_time: Instant,
}

impl Notification {

    pub fn new(title: String, text: Option<String>, color: (Color, Color), duration: Duration) -> Self {
        Self { title, text, color, duration, added_time: Instant::now() }
    }

    fn is_hidden(&self) -> bool {
        self.added_time.elapsed() > self.duration
    }
}

#[derive(Default)]
pub struct NotificationHandler {
    notifications: Vec<Notification>
}

impl NotificationHandler {

    pub fn poll() -> IntoIter<Notification> {
        let mut notifications = NOTIFICATIONS.lock().unwrap();
        notifications.notifications.retain(|notification| !notification.is_hidden());
        notifications.notifications.to_vec().into_iter()
    }

    pub fn push(title: impl Into<String>, desc: Option<String>, color: (Color, Color), duration: Duration) {
        let mut notifications = NOTIFICATIONS.lock().unwrap();
        let limited_desc = desc.map(|it| {
            let mut list: Vec<String> = vec![];
            let mut last = "".to_string();
            for split in it.split(" ") {
                if last.len() + split.len() > 40 {
                    list.push(last.trim().to_string());
                    last = split.to_string();
                } else {
                    last = format!("{} {}", last, split);
                }
            }
            if !last.is_empty() {
                list.push(last.trim().to_string())
            }
            list.join("\n")
        });
        notifications.notifications.push(Notification::new(title.into(), limited_desc, color, duration));
    }
}

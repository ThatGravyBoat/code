use crate::pages::notifications::notification_handler::NotificationHandler;
use crate::utils::extensions::stylize_ext::WidgetExt;
use ratatui::layout::Constraint::{Fill, Length};
use ratatui::layout::Layout;
use ratatui::prelude::*;
use ratatui::widgets::{Clear, List, ListItem};
use ratatui::Frame;

pub(crate) fn render_notifications(frame: &mut Frame) {
    let mut max_width = 30;
    let mut height = 0;

    NotificationHandler::poll().for_each(|notification| {
        height += 3;
        max_width = max_width.max(notification.title.len());
        if let Some(text) = notification.text {
            for line in text.lines() {
                height += 1;
                max_width = max_width.max(line.len());
            }
        }
    });

    let items = NotificationHandler::poll().map(|notification| {
        let (text_color, bg_color) = notification.color;
        if let Some(text) = notification.text {
            ListItem::new(format!(
                "\n {}\n{}\n\n",
                notification.title,
                text.lines().map(|it| format!(" {}", it)).collect::<Vec<String>>().join("\n")
            )).fg(text_color).bg(bg_color)
        } else {
            ListItem::new(format!("\n {}\n\n", notification.title)).fg(text_color).bg(bg_color)
        }
    });
    let [_, notification_area_1] = Layout::horizontal([Fill(1), Length((max_width + 2) as u16)]).areas(frame.area());
    let [notification_area_2] = Layout::vertical([Length(height as u16)]).areas(notification_area_1);
    Clear.render_widget(frame, notification_area_2);
    List::new(items).render_widget(frame, notification_area_2);
}
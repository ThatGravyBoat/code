use crate::pages::page::{Page, Section};
use crate::utils::ui::{create_border_block, create_text, TEXT};
use ratatui::layout::Rect;
use ratatui::prelude::Stylize;
use ratatui::widgets::{List, ListItem};
use ratatui::Frame;
use crate::utils::extensions::stylize_ext::WidgetExt;
use crate::utils::utils::LETTERS_NUMBERS;

pub fn draw_accounts(data: &Page, frame: &mut Frame, area: Rect) {
    let is_selected = data.section == Section::Accounts;

    let items: Vec<ListItem> = data.accounts
        .iter()
        .enumerate()
        .map(|(index, account)|
            ListItem::from(format!(" {}. {}", LETTERS_NUMBERS[index], account.username)).fg(TEXT).bold()
        )
        .collect();

    List::new(items)
        .block(create_border_block().title(create_text("Accounts", is_selected)))
        .render_widget(frame, area);
}
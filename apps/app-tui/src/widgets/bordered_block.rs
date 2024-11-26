use crate::utils::ui::{BRAND, SECONDARY};
use ratatui::prelude::*;
use ratatui::widgets::block::Title;
use ratatui::widgets::*;

pub struct BorderedBlock();
impl BorderedBlock {

    pub fn new<'a>() -> Block<'a> {
        Block::bordered().border_type(BorderType::Rounded).border_style(Style::new().fg(SECONDARY))
    }
}

pub struct BlockTitle();

impl BlockTitle {

    pub fn new<'a>(text: &'a str, selected: bool) -> Title<'a> {
        let color = if selected { BRAND } else { SECONDARY };
        let (first, remainder) = text.split_at(1);
        Title::from(vec!["[".fg(SECONDARY), first.fg(color), "]".fg(SECONDARY), remainder.fg(SECONDARY)])
    }
}
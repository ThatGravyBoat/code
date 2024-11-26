use crate::utils::ui::{create_border_block, create_text, TEXT};
use crossterm::event::KeyCode;
use ratatui::prelude::{Line, Stylize};
use ratatui::widgets::Paragraph;

pub struct TextState {
    text: String,
    blink: bool,
}

impl TextState {

    pub fn new<T : Into<String>>(default: T) -> Self {
        Self { text: default.into().clone(), blink: false }
    }

    pub fn on_key_press(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Backspace | KeyCode::Delete => {
                if !self.text.is_empty() {
                    self.text.truncate(self.text.len() - 1)
                }
            },
            KeyCode::Char(char) => self.text = format!("{}{}", self.text, char),
            _ => return false,
        }
        true
    }

    pub fn on_tick(&mut self, tick_count: &u64) {
        self.blink = tick_count % 10 < 5;
    }

    pub fn get(&self) -> String {
        self.text.clone()
    }

    pub fn set(&mut self, text: String) {
        self.text = text.clone();
    }
}

pub struct TextInput();
impl TextInput {

    pub fn new<'a>(
        title: &'a str,
        selected: bool,
        state: &TextState,
    ) -> Paragraph<'a> {
        if selected && state.blink {
            Paragraph::new(Line::default().spans(vec![state.get().fg(TEXT), "_".slow_blink()]))
                .block(create_border_block().title(create_text(title, selected)))
        } else {
            Paragraph::new(state.get().fg(TEXT))
                .block(create_border_block().title(create_text(title, selected)))
        }
    }
}
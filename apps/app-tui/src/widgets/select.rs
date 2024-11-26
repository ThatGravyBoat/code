use crate::utils::ui::*;
use crossterm::event::KeyCode;
use ratatui::prelude::*;
use ratatui::widgets::*;

#[derive(Default)]
pub struct SelectState<T : Clone> {
    index: Option<usize>,
    options: Vec<T>,
}

impl <T : Clone> SelectState<T> {

    pub fn new(index: Option<usize>, options: impl IntoIterator<Item = T>) -> Self {
        let options = options.into_iter().collect::<Vec<T>>();
        Self { index, options }
    }

    pub fn on_key_press(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Up => {
                if let Some(index) = self.index {
                    if index > 0 {
                        self.index = Some(index - 1);
                    }
                } else if !self.options.is_empty() {
                    self.index = Some(0);
                }
            }
            KeyCode::Down => {
                if let Some(index) = self.index {
                    if index + 1 < self.options.len() {
                        self.index = Some(index + 1);
                    }
                } else if !self.options.is_empty() {
                    self.index = Some(0);
                }
            }
            KeyCode::PageUp => {
                if !self.options.is_empty() {
                    self.index = Some(0);
                }
            }
            KeyCode::PageDown => {
                if !self.options.is_empty() {
                    self.index = Some(self.options.len() - 1);
                }
            }
            _ => return false,
        }
        true
    }

    pub fn get(&self) -> Option<T> {
        self.index.map(|index| self.options[index].clone())
    }

    pub fn set(&mut self, index: Option<usize>) {
        self.index = index;
    }
}

pub struct SelectList<'a> {
    offset: Option<usize>,
    list: List<'a>,
}

impl <'a> SelectList<'a> {

    pub fn new<T : Clone>(
        title: &'a str,
        selected: bool,
        state: &SelectState<T>,
        formatter: fn(&T) -> String,
    ) -> Self {
        Self {
            list: List::new(state.options.iter().map(|option| ListItem::new(formatter(option).fg(TEXT))))
                .highlight_style(Style::new().fg(BRAND).bold())
                .block(create_border_block().title(create_text(title, selected))),
            offset: state.index.clone(),
        }
    }
}

impl WidgetRef for SelectList<'_> {

    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        Clear.render_ref(area, buf);
        StatefulWidgetRef::render_ref(
            &self.list,
            area,
            buf,
            &mut ListState::default().with_selected(self.offset)
        )
    }
}
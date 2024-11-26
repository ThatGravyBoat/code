use crossterm::event::KeyCode;
use ratatui::prelude::{Line, Style, Stylize};
use ratatui::widgets::Tabs;
use crate::utils::extensions::stylize_ext::StylizeExt;
use crate::utils::ui::{create_border_block, create_text, BRAND, SECONDARY, TEXT};

pub struct ChipState<T> {
    index: Option<usize>,
    options: Vec<T>,
}

impl <T : Clone> ChipState<T> {

    pub fn new(index: impl Into<Option<usize>>, options: impl IntoIterator<Item = T>) -> Self {
        let options = options.into_iter().collect::<Vec<T>>();
        Self { index: index.into(), options }
    }

    pub fn on_key_press(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Left => self.prev(),
            KeyCode::Right => self.next(),
            KeyCode::Home => self.index = Some(0),
            KeyCode::End => self.index = Some(self.options.len() - 1),
            _ => return false,
        }
        true
    }

    pub fn next(&mut self) {
        if let Some(index) = self.index {
            if index + 1 < self.options.len() {
                self.index = Some(index + 1);
            }
        } else {
            self.index = Some(0);
        }
    }

    pub fn prev(&mut self) {
        if let Some(index) = self.index {
            if index != 0 {
                self.index = Some(index - 1);
            }
        } else {
            self.index = Some(0);
        }
    }

    pub fn get(&self) -> Option<T> {
        self.index.map(|index| self.options[index].clone())
    }

    pub fn set(&mut self, index: Option<usize>) {
        self.index = index;
    }
}

pub struct ChoiceChip();
impl ChoiceChip {

    pub fn new<'a, T>(
        title: &'a str,
        selected: bool,
        state: &ChipState<T>,
        formatter: fn(&T) -> String,
    ) -> Tabs<'a> where T: Eq + ToString {
        let tabs = state.options.iter().enumerate().map(|(index, option)| {
            let is_selected = state.index.map(|it| it == index).unwrap_or(false);
            Line::default()
                .spans(vec![
                    "[".fg(SECONDARY),
                    formatter(option).with_fg(is_selected, BRAND, TEXT).with_bold(is_selected),
                    "]".fg(SECONDARY)
                ])
        });

        Tabs::new(tabs)
            .padding(" ", " ")
            .divider(" ")
            .highlight_style(Style::new())
            .block(create_border_block().title(create_text(title, selected)))
    }
}
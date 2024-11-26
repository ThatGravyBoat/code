use crate::utils::extensions::stylize_ext::WidgetExt;
use ratatui::layout::*;
use ratatui::prelude::*;
use ratatui::widgets::{Block, WidgetRef};

#[derive(Default)]
pub struct Layouts<'a> {
    direction: Direction,
    margin: u16,
    spacing: u16,

    block: Option<Block<'a>>,

    constraints: Vec<Constraint>,
    widgets: Vec<Box<dyn WidgetRef>>,
}

impl <'a> Layouts<'a> {

    pub fn vertical() -> Self {
        Self { direction: Direction::Vertical, ..Self::default() }
    }

    pub fn horizontal() -> Self {
        Self { direction: Direction::Horizontal, ..Self::default() }
    }

    pub fn margin(mut self, margin: u16) -> Self {
        self.margin = margin;
        self
    }

    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn with<T : WidgetRef + 'static, C : Into<Constraint>>(mut self, widget: T, constraint: C) -> Self {
        self.widgets.push(Box::new(widget));
        self.constraints.push(constraint.into());
        self
    }

    pub fn with_option<T : WidgetRef + 'static, C : Into<Constraint>>(mut self, widget: Option<T>, constraint: C) -> Self {
        if let Some(widget) = widget {
            self.widgets.push(Box::new(widget));
            self.constraints.push(constraint.into());
        }
        self
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn render(self, frame: &mut Frame, area: Rect) {
        let margin = if let Some(block) = self.block {
            block.render_widget(frame, area);
            self.margin + 1
        } else {
            self.margin
        };

        let areas = Layout::new(self.direction, &self.constraints)
            .spacing(self.spacing)
            .margin(margin)
            .split(area);

        let buffer = frame.buffer_mut();

        for i in 0..self.widgets.len() {
            self.widgets[i].render_ref(areas[i], buffer);
        }
    }
}
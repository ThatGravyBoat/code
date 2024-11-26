use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::BlockExt;
use ratatui::text::Text;
use ratatui::widgets::{Block, WidgetRef};

pub struct CenteredText<'a> {
    block: Option<Block<'a>>,
    text: Text<'a>
}

impl <'a> CenteredText<'a> {

    pub fn new(text: impl Into<Text<'a>>) -> Self {
        Self {
            text: text.into().alignment(Alignment::Center),
            block: None
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

}

impl <'a> WidgetRef for CenteredText<'a> {

    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        self.block.render_ref(area, buf);
        let height = self.text.lines.len() as u16;
        let area = self.block.inner_if_some(area);
        let area = Rect::new(area.x, area.y + (area.height + height) / 2, area.width, height);
        self.text.render_ref(area, buf);
    }
}
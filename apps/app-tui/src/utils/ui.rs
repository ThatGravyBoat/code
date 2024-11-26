use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType};

pub const BRAND: Color = Color::Rgb(27, 217, 106);
pub const ORANGE: Color = Color::Rgb(255, 163, 71);
pub const DANGER: Color = Color::Rgb(255, 73, 110);

pub const TEXT: Color = Color::Rgb(176, 186, 197);
pub const SECONDARY: Color = Color::Rgb(150, 162, 176);

pub fn create_border_block<'a>() -> Block<'a> {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(SECONDARY))
}

pub fn create_title(lines: Vec<Line>) -> Line {
    Line::default()
        .spans(lines.iter().flat_map(|it| it.spans.clone()).collect::<Vec<Span>>())
}

pub fn create_text(text: &str, selected: bool) -> Line {
    let color = if selected { BRAND } else { SECONDARY };
    let (first, remainder) = text.split_at(1);
    Line::default()
        .spans(vec!["[".fg(SECONDARY), first.fg(color), "]".fg(SECONDARY), remainder.fg(SECONDARY)])
}

pub fn center_layout(area: Rect, width: u16, height: u16) -> Rect {
    let [_, vertical, _] = Layout::vertical(
        [Constraint::Fill(1), Constraint::Length(height), Constraint::Fill(1)]
    ).areas(area);
    let [_, horizontal, _] = Layout::horizontal(
        [Constraint::Fill(1), Constraint::Length(width), Constraint::Fill(1)]
    ).areas(vertical);

    horizontal
}
use crate::data::texts::Texts;
use crate::pages::page::{Page, Section};
use crate::utils::extensions::stylize_ext::WidgetExt;
use crate::utils::ui::{center_layout, create_border_block, TEXT};
use crossterm::event::KeyCode;
use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::Frame;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

pub struct InstanceInstallingSection();
impl InstanceInstallingSection {

    fn is_finished(data: &mut Page) -> bool {
        data.creation_data.installer.as_ref().map(JoinHandle::is_finished).unwrap_or(false)
    }

    pub fn draw(data: &mut Page, frame: &mut Frame, area: Rect) {
        create_border_block()
            .title(Texts::INSTALLING_TITLE.fg(TEXT))
            .render_widget(frame, area);

        Line::from(if Self::is_finished(data) { Texts::INSTALLED } else { Texts::INSTALLING })
            .fg(TEXT)
            .centered()
            .render_widget(frame, center_layout(area, 40, 1));
    }

    pub fn on_key_press(data: &mut Page, key: KeyCode, runtime: &Runtime) -> bool {
        if Self::is_finished(data) && (key == KeyCode::Char('c') || key == KeyCode::Char('C')) {
            data.creation_data.installer = None;
            data.section = Section::Instances;
            data.load(runtime);
        }
        true // we want to control the entire handling until installing is done.
    }
}
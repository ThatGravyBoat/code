use crate::data::texts::Texts;
use crate::pages::page::{Page, Section};
use crate::pages::viewing::view_instance_section::InstanceViewData;
use crate::utils::ui::{create_border_block, create_text, TEXT};
use crate::utils::utils::LETTERS_NUMBERS;
use crossterm::event::KeyCode;
use ratatui::layout::Rect;
use ratatui::prelude::Stylize;
use ratatui::widgets::{List, ListItem};
use ratatui::Frame;
use theseus::state::ProfileInstallStage;
use tokio::runtime::Runtime;
use crate::utils::extensions::stylize_ext::StylizeExt;

pub struct InstancesSection();
impl InstancesSection {

    pub fn draw(data: &Page, frame: &mut Frame, area: Rect) {
        let is_selecting_instance = data.section == Section::Instances;
        let is_creating_instance = data.section == Section::InstanceCreation;

        let items: Vec<ListItem> = data.instances
            .iter()
            .enumerate()
            .map(|(index, profile)|
                ListItem::from(format!(" {}. {}", LETTERS_NUMBERS[index], profile.name))
                    .fg(TEXT)
                    .bold()
                    .with_strikethrough(profile.install_stage != ProfileInstallStage::Installed)
            )
            .collect();

        let mut block = create_border_block().title(create_text(Texts::INSTANCES_TITLE, is_selecting_instance));
        if is_selecting_instance || is_creating_instance {
            block = block.title(create_text("+", is_creating_instance).right_aligned());
        }

        frame.render_widget(List::new(items).block(block), area);
    }

    pub fn on_key_press(data: &mut Page, key: KeyCode, runtime: &Runtime) -> bool {
        if let KeyCode::Char(input) = key {
            let index = LETTERS_NUMBERS.iter().position(|&l| l == input).unwrap_or(LETTERS_NUMBERS.len());
            if index < data.instances.len() {
                data.viewing = Some(InstanceViewData::new(index, data, runtime));
                data.section = Section::InstanceViewing;
                return true;
            } else if input == '+' || input == '=' {
                data.section = Section::InstanceCreation;
                return true;
            }
        }
        false
    }
}
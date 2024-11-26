use crate::data::texts::Texts;
use crate::pages::creating::data::SelectedInput;
use crate::pages::page::{Page, Section};
use crate::utils::ui::TEXT;
use crate::widgets::bordered_block::BorderedBlock;
use crate::widgets::chip::ChoiceChip;
use crate::widgets::layouts::Layouts;
use crate::widgets::select::SelectList;
use crate::widgets::text_input::TextInput;
use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Rect};
use ratatui::prelude::Stylize;
use ratatui::Frame;
use std::string::ToString;
use theseus::prelude::create::profile_create;
use tokio::runtime::Runtime;
use crate::utils::extensions::stylize_ext::StylizeExt;

pub struct InstanceCreationSection();
impl InstanceCreationSection {

    pub fn draw(main_data: &mut Page, frame: &mut Frame, area: Rect) {
        let data = &main_data.creation_data;
        let can_save = !data.name.get().is_empty() && data.game_version.get().is_some();

        Layouts::vertical()
            .spacing(1)
            .margin(1)
            .block(BorderedBlock::new()
                .title(Texts::CREATING_TITLE.fg(TEXT))
                .title(Texts::CREATING_SAVE.fg(TEXT).with_strikethrough(!can_save).into_right_aligned_line())
            )
            .with(TextInput::new(Texts::CREATING_NAME, data.input == SelectedInput::Name, &data.name), 3)
            .with(ChoiceChip::new(Texts::CREATING_LOADER, data.input == SelectedInput::Loader, &data.loader, ToString::to_string), 3)
            .with(SelectList::new(Texts::CREATING_VERSIONS, data.input == SelectedInput::Version, &data.game_version, ToString::to_string), Constraint::Min(3))
            .render(frame, area);
    }

    pub fn on_key_press(main_data: &mut Page, key: KeyCode, runtime: &Runtime) -> bool {
        let data = &mut main_data.creation_data;

        if key == KeyCode::Esc && data.input != SelectedInput::None {
            data.input = SelectedInput::None;
            true
        } else {
            match data.input {
                SelectedInput::Name => data.name.on_key_press(key),
                SelectedInput::Loader => {
                    if data.loader.on_key_press(key) {
                        data.game_version.set(None);
                        true
                    } else {
                        false
                    }
                },
                SelectedInput::Version => data.game_version.on_key_press(key),
                SelectedInput::None => {
                    match key {
                        KeyCode::Char('n') | KeyCode::Char('N') => data.input = SelectedInput::Name,
                        KeyCode::Char('m') | KeyCode::Char('M') => data.input = SelectedInput::Loader,
                        KeyCode::Char('g') | KeyCode::Char('G') => {
                            data.input = SelectedInput::Version;
                            if data.game_version.get().is_none() {
                                data.game_version.set(Some(0));
                            }
                        },
                        KeyCode::Char('s') | KeyCode::Char('S') => {
                            let name = data.name.get();
                            let loader = data.loader.get().unwrap();

                            if let Some(version) = data.game_version.get() {
                                if !name.is_empty() {
                                    data.installer = Some(runtime.spawn(profile_create(
                                        name.clone(), version.clone(), loader, None, None, None, None
                                    )));
                                    main_data.section = Section::InstanceInstalling;
                                    return true;
                                }
                            }

                            return false;
                        }
                        _ => return false,
                    }
                    true
                }
            }
        }
    }
}
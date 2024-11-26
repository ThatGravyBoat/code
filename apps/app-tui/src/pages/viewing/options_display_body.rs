use crate::data::texts::Texts;
use crate::pages::page::{Page, Section};
use crate::pages::viewing::view_instance_section::{InstanceTab, InstanceViewData};
use crate::utils::extensions::stylize_ext::WidgetExt;
use crate::utils::profile_utils::ProfileExt;
use crate::utils::ui::{create_border_block, TEXT};
use crate::widgets::bordered_block::BorderedBlock;
use crate::widgets::chip::*;
use crate::widgets::layouts::Layouts;
use crate::widgets::text_input::{TextInput, TextState};
use crossterm::event::KeyCode;
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use std::fmt::Display;
use theseus::data::MemorySettings;
use theseus::prelude::Profile;
use tokio::runtime::Runtime;

#[derive(Eq, PartialEq)]
enum Input { None, Actions, Name, Memory, Fullscreen }

#[derive(Eq, PartialEq, Clone)]
enum ActionType {
    Delete, UpdateLoader, OpenFolder, Logs
}

impl Display for ActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ActionType::Delete => "Delete",
            ActionType::UpdateLoader => "Update Loader",
            ActionType::OpenFolder => "Open Folder",
            ActionType::Logs => "Logs",
        };
        write!(f, "{}", str)
    }
}

pub struct OptionsDisplayData {
    pub(self) input: Input,
    pub(self) deleting: bool,
    pub(self) memory: ChipState<u32>,
    pub(self) fullscreen: ChipState<bool>,
    pub(self) actions: ChipState<ActionType>,
    pub(self) name: TextState,
}

impl OptionsDisplayData {
    pub fn new(profile: &Profile) -> Self {
        Self {
            input: Input::None,
            deleting: false,
            memory: ChipState::new(
                profile.memory.map(|it| (it.maximum / 2048).min(3) as usize),
                vec![2048u32, 4096u32, 6144u32, 8192u32]
            ),
            fullscreen: ChipState::new(profile.force_fullscreen.map(|it| it as usize), vec![true, false]),
            actions: ChipState::new(None, vec![ActionType::Logs, ActionType::UpdateLoader, ActionType::OpenFolder, ActionType::Delete]),
            name: TextState::new(&profile.name)
        }
    }
}

pub struct OptionsDisplayBody();
impl OptionsDisplayBody {

    fn draw_body(data: &InstanceViewData, frame: &mut Frame, area: Rect) {
        let data = &data.options;
        Layouts::vertical()
            .margin(1)
            .spacing(1)
            .block(BorderedBlock::new().title(Texts::OPTIONS_TITLE.fg(TEXT)))
            .with(ChoiceChip::new("Actions", data.input == Input::Actions, &data.actions, ToString::to_string), 3)
            .with(TextInput::new("Name", data.input == Input::Name, &data.name), 3)
            .with(ChoiceChip::new("Memory", data.input == Input::Memory, &data.memory, |memory| format!("{} GB", memory / 1024)), 3)
            .with(ChoiceChip::new("Fullscreen", data.input == Input::Fullscreen, &data.fullscreen, ToString::to_string), 3)
            .render(frame, area);
    }

    fn draw_footer(profile: &Profile, data: &InstanceViewData, frame: &mut Frame, area: Rect) {
        let data = &data.options;
        if data.deleting {
            Paragraph::new("Press [Enter] again to delete.").fg(TEXT)
                .block(create_border_block().title(format!("Deleting: {}", profile.name).fg(TEXT)))
                .render_widget(frame, area);
        } else {
            let text = match data.input {
                Input::None => return,
                Input::Actions => Texts::OPTIONS_ACTIONS,
                Input::Name => Texts::OPTIONS_NAME,
                Input::Memory => Texts::OPTIONS_MEMORY,
                Input::Fullscreen => Texts::OPTIONS_FULLSCREEN,
            };
            Paragraph::new(text)
                .block(create_border_block().title(Texts::OPTIONS_FOOTER.fg(TEXT)))
                .render_widget(frame, area);
        }
    }

    pub fn draw(profile: &Profile, data: &InstanceViewData, frame: &mut Frame, area: Rect) {
        if data.options.input == Input::None {
            Self::draw_body(data, frame, area);
        } else {
            let [body, footer] = Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]).areas(area);
            Self::draw_body(data, frame, body);
            Self::draw_footer(profile, data, frame, footer);
        }
    }

    pub fn on_tick(profile: &Profile, options: &mut OptionsDisplayData, tick_count: &u64, runtime: &Runtime) {
        options.name.on_tick(tick_count);
    }

    pub fn on_key_press(page: &mut Page, key: KeyCode, runtime: &Runtime) -> bool {
        let view_data = page.viewing.as_mut().unwrap();
        let profile = &mut page.instances[view_data.index];
        let options = &mut view_data.options;

        if key == KeyCode::Esc && options.input != Input::None {
            profile.edit(runtime, |profile| {
                profile.name = options.name.get().clone();
                profile.memory = options.memory.get().map(|memory| MemorySettings { maximum: memory.clone() });
                profile.force_fullscreen = options.fullscreen.get();
            });
            options.input = Input::None;
            options.actions.set(None);
            true
        } else {
            match options.input {
                Input::None => {
                    match key {
                        KeyCode::Char('a') | KeyCode::Char('A') => options.input = Input::Actions,
                        KeyCode::Char('n') | KeyCode::Char('N') => options.input = Input::Name,
                        KeyCode::Char('m') | KeyCode::Char('M') => options.input = Input::Memory,
                        KeyCode::Char('f') | KeyCode::Char('F') => options.input = Input::Fullscreen,
                        _ => return false,
                    }
                    options.deleting = false;
                    true
                }
                Input::Name => options.name.on_key_press(key),
                Input::Memory => options.memory.on_key_press(key),
                Input::Fullscreen => options.fullscreen.on_key_press(key),
                Input::Actions => {
                    let handled = options.actions.on_key_press(key);
                    if !handled && key == KeyCode::Enter {
                        if let Some(action) = options.actions.get() {
                            match action {
                                ActionType::Delete => {
                                    if options.deleting && profile.delete(runtime) {
                                        page.instances.remove(view_data.index);
                                        page.viewing = None;
                                        page.section = Section::None;
                                    } else {
                                        options.deleting = true;
                                    }
                                }
                                ActionType::UpdateLoader => {}
                                ActionType::OpenFolder => profile.open(runtime),
                                ActionType::Logs => view_data.tab = InstanceTab::Logs,
                            }
                            return true;
                        }
                    }
                    options.deleting = false;
                    handled
                }
            }
        }
    }
}
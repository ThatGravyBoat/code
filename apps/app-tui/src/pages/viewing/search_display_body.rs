use crate::pages::page::Page;
use crate::pages::viewing::mods_display_body::ModDisplayData;
use crate::pages::viewing::view_instance_section::InstanceViewData;
use crate::utils::profile_utils::ProfileExt;
use crate::utils::ui::TEXT;
use crate::widgets::bordered_block::BorderedBlock;
use crate::widgets::layouts::Layouts;
use crate::widgets::search::{SearchList, SearchState};
use crate::widgets::text_input::{TextInput, TextState};
use crossterm::event::KeyCode;
use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, Wrap};
use theseus::prelude::Profile;
use tokio::runtime::Runtime;

#[derive(Eq, PartialEq, Clone)]
enum Input { None, Search, Results }

pub struct SearchDisplayData {
    input: Input,
    search: TextState,
    results: SearchState,
}

impl SearchDisplayData {
    pub fn new() -> Self {
        Self {
            input: Input::None,
            search: TextState::new(""),
            results: SearchState::new(None),
        }
    }
}

pub struct SearchDisplayBody();
impl SearchDisplayBody {

    pub fn draw(view_data: &mut InstanceViewData, frame: &mut Frame, area: Rect) {
        let data = &mut view_data.search;

        if let Ok(files) = &view_data.mods.files {
            data.results.set_installed(
                files.iter()
                    .filter_map(|it|
                        it.metadata.as_ref().map(|it| (it.project_id.to_string(), it.version_id.to_string()))
                    )
                    .collect::<Vec<(String, String)>>()
            );
        }

        Layouts::vertical()
            .with(
                TextInput::new("Search", data.input == Input::Search, &data.search),
                3
            )
            .with(
                SearchList::new("Results", data.input == Input::Results, &data.results),
                Constraint::Fill(1)
            )
            .with_option(
                data.results.get().map(|it|
                    Paragraph::new(it.description)
                        .wrap(Wrap { trim: true })
                        .block(BorderedBlock::new()
                            .title(format!("[{}]", it.title).fg(TEXT))
                            .title("[↵ to install]".fg(TEXT).into_right_aligned_line())
                       )
                ),
                Constraint::Max(4)
            )
            .render(frame, area);
    }

    pub fn on_tick(profile: &Profile, search: &mut SearchDisplayData, tick_count: &u64, runtime: &Runtime) {
        search.search.on_tick(tick_count);

        // 0.5s
        if tick_count % 5 == 0 {
            search.results.try_search(
                runtime,
                vec![
                    "[\"project_type:mod\"]".to_string(),
                    format!("[\"categories:{}\"]", profile.loader.as_str()),
                    format!("[\"versions:{}\"]", profile.game_version),
                ],
                search.search.get()
            );
        }
    }

    pub fn on_key_press(page: &mut Page, key: KeyCode, runtime: &Runtime) -> bool {
        let index = page.viewing.as_ref().unwrap().index.clone();
        let view_data = page.viewing.as_mut().unwrap();
        let data = &mut view_data.search;
        let profile = &mut page.instances[index];

        if key == KeyCode::Esc && data.input != Input::None {
            data.input = Input::None;
            true
        } else {
            match data.input {
                Input::Search => data.search.on_key_press(key),
                Input::Results => {
                    let handled = data.results.on_key_press(key, runtime);
                    if !handled && key == KeyCode::Enter {
                        if let Some(entry) = data.results.get() {
                            profile.install(entry.project_id.to_string(), runtime);
                            view_data.mods = ModDisplayData::new(profile.path.as_str(), runtime);
                        }
                        return true;
                    }
                    handled
                },
                Input::None => {
                    match key {
                        KeyCode::Char('s') | KeyCode::Char('S') => data.input = Input::Search,
                        KeyCode::Char('r') | KeyCode::Char('R') => data.input = Input::Results,
                        _ => return false,
                    };
                    true
                },
            }
        }
    }
}
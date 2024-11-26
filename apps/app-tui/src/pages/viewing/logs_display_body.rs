use crate::pages::page::Page;
use crate::pages::viewing::view_instance_section::InstanceViewData;
use crate::utils::utils::run_or_notify;
use crate::widgets::layouts::Layouts;
use crate::widgets::select::{SelectList, SelectState};
use crate::widgets::text_input::{TextInput, TextState};
use crossterm::event::KeyCode;
use ratatui::prelude::*;
use theseus::logs;
use theseus::prelude::Profile;
use tokio::runtime::Runtime;

#[derive(Eq, PartialEq, Clone)]
enum Input { None, Search, Logs }

pub struct LogsDisplayData {
    input: Input,
    search: TextState,
    logs: SelectState<String>,

    last_search: String,
    original: String,
}

impl LogsDisplayData {
    pub fn new(path: &str, runtime: &Runtime) -> Self {
        if let Some(log) = run_or_notify(runtime, logs::get_latest_log_cursor(path, 0), ToString::to_string)  {
            let logs = log.output.0.to_string().replace("\t", "    ");
            Self {
                input: Input::None,
                search: TextState::new(""),
                logs: SelectState::new(None, logs.to_string().lines().map(ToString::to_string)),
                last_search: "".to_string(),
                original:  logs.to_string(),
            }
        } else {
            Self {
                input: Input::None,
                search: TextState::new(""),
                logs: SelectState::new(None, vec![]),
                last_search: "".to_string(),
                original: "".to_string(),
            }
        }
    }
}

pub struct LogsDisplayBody();
impl LogsDisplayBody {

    pub fn draw(view_data: &mut InstanceViewData, frame: &mut Frame, area: Rect) {
        let data = &mut view_data.logs;

        Layouts::vertical()
            .with(
                TextInput::new("Search", data.input == Input::Search, &data.search),
                3
            )
            .with(
                SelectList::new("Logs", data.input == Input::Logs, &data.logs, ToString::to_string),
                Constraint::Fill(1)
            )
            .render(frame, area);
    }

    pub fn on_tick(profile: &Profile, logs: &mut LogsDisplayData, tick_count: &u64, runtime: &Runtime) {
        logs.search.on_tick(tick_count);

        let search = logs.search.get();

        // 0.5s
        if tick_count % 5 == 0 && search != logs.last_search {
            logs.last_search = search.to_string();

            let log_lines = if search.is_empty() {
                logs.original.lines().map(ToString::to_string).collect::<Vec<String>>()
            } else {
                logs.original.lines()
                    .filter(|line| search.is_empty() || line.to_lowercase().contains(search.as_str()))
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
            };

            logs.logs = SelectState::new(None, log_lines);
        }
    }

    pub fn on_key_press(page: &mut Page, key: KeyCode, runtime: &Runtime) -> bool {
        let view_data = page.viewing.as_mut().unwrap();
        let data = &mut view_data.logs;

        if key == KeyCode::Esc && data.input != Input::None {
            data.input = Input::None;
            true
        } else {
            match data.input {
                Input::Search => data.search.on_key_press(key),
                Input::Logs => data.logs.on_key_press(key),
                Input::None => {
                    match key {
                        KeyCode::Char('s') | KeyCode::Char('S') => data.input = Input::Search,
                        KeyCode::Char('l') | KeyCode::Char('L') => data.input = Input::Logs,
                        _ => return false,
                    };
                    true
                },
            }
        }
    }
}
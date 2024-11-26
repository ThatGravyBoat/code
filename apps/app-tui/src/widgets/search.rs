use crate::utils::extensions::search_entry_ext::SearchEntryExt;
use crate::utils::ui::{create_border_block, create_text, BRAND, SECONDARY, TEXT};
use crate::utils::utils::run_or_notify;
use crossterm::event::KeyCode;
use ratatui::prelude::*;
use ratatui::text::{Line, Text};
use ratatui::widgets::{Row, StatefulWidgetRef, Table, TableState, WidgetRef};
use theseus::cache::get_search_results;
use theseus::state::SearchEntry;
use tokio::runtime::Runtime;

#[derive(Default)]
pub struct SearchState {
    index: Option<usize>,
    results: Vec<SearchEntry>,
    installed: Vec<(String, String)>, // project, version

    page: u32,
    total: u32,

    last_facets: Vec<String>,
    last_query: String,
}

impl SearchState {

    pub fn new(index: Option<usize>) -> Self {
        Self { index, ..Self::default() }
    }

    pub fn on_key_press(&mut self, key: KeyCode, runtime: &Runtime) -> bool {
        match key {
            KeyCode::Left => {
                if self.page > 0 {
                    self.page -= 1;
                    self.search(runtime);
                }
            }
            KeyCode::Right => {
                if (self.page + 1) * 10 < self.total {
                    self.page += 1;
                    self.search(runtime);
                }
            }
            KeyCode::Up => {
                if let Some(index) = self.index {
                    if index > 0 {
                        self.index = Some(index - 1);
                    }
                } else if !self.results.is_empty() {
                    self.index = Some(0);
                }
            }
            KeyCode::Down => {
                if let Some(index) = self.index {
                    if index + 1 < self.results.len() {
                        self.index = Some(index + 1);
                    }
                } else if !self.results.is_empty() {
                    self.index = Some(0);
                }
            }
            KeyCode::PageUp => {
                if !self.results.is_empty() {
                    self.index = Some(0);
                }
            }
            KeyCode::PageDown => {
                if !self.results.is_empty() {
                    self.index = Some(self.results.len() - 1);
                }
            }
            _ => return false,
        }
        true
    }

    pub fn get(&self) -> Option<SearchEntry> {
        self.index.map(|index| self.results[index].clone())
    }

    fn search(&mut self, runtime: &Runtime) {
        let res = run_or_notify(
            runtime,
            get_search_results(format!(
                "?facets=[{}]&limit={}&offset={}&index={}&query={}",
                self.last_facets.join(","),
                10,
                self.page * 10,
                "relevance",
                self.last_query,
            ).as_str(), None),
            ToString::to_string,
        ).flatten();

        if let Some(res) = res {
            self.results = res.result.hits;
            self.page = res.result.offset / 10;
            self.total = (res.result.total_hits as f64 / 10.0).ceil() as u32;
            self.index = None;
        }
    }

    pub fn try_search(&mut self, runtime: &Runtime, facets: Vec<String>, query: String) {
        if facets != self.last_facets || query != self.last_query {
            self.last_facets = facets.clone();
            self.last_query = query.clone();

            self.search(runtime);
        }
    }

    pub fn set_installed(&mut self, installed: Vec<(String, String)>) {
        self.installed = installed;
    }
}

pub struct SearchList<'a> {
    offset: Option<usize>,
    table: Table<'a>,
}

impl <'a> SearchList<'a> {

    pub fn new(
        title: &'a str,
        selected: bool,
        state: &SearchState,
    ) -> Self {
        let rows: Vec<Row> = state.results.iter().map(|entry| {
            let title = entry.title.clone();
            let author = entry.author.clone();
            let modified = entry.get_modified_display().clone();
            let downloads = entry.get_download_display().clone();
            let follows = entry.get_follower_display().clone();

            let installed_version = state.installed.iter()
                .find(|(project, _)| project == &entry.project_id)
                .map(|(_, version)| version);

            let installed = installed_version.is_some();

            let installed_text = if installed {
                Span::from("Installed").fg(BRAND)
            } else {
                Span::from("Not Installed").fg(TEXT)
            };

            Row::new([
                Text::from(vec![
                    Line::from(vec![
                        Span::from(title).fg(TEXT).bold(),
                        Span::from(" "),
                        Span::from(format!("by {}", author)).fg(SECONDARY).underlined(),
                    ]),
                    Line::from(vec![
                        Span::from("[").fg(TEXT), installed_text, Span::from("]").fg(TEXT),
                        Span::from(" | "),
                        Span::from(format!("⭮ Updated {}", modified)).fg(SECONDARY),
                    ]),
                ]).left_aligned(),
                Text::from(vec![
                    Line::from(vec![
                        Span::from(format!("🢱 {}", downloads)).fg(TEXT).bold(),
                        Span::from(" downloads").fg(SECONDARY),
                    ]),
                    Line::from(vec![
                        Span::from(format!("♥ {}", follows)).fg(TEXT).bold(),
                        Span::from(" followers").fg(SECONDARY),
                    ]),
                ]).right_aligned(),
            ]).height(3)
        }).collect();

        let current_page = state.page + 1;
        let total_pages = if state.total > 10 {
            (state.total as f64 / 10.0).ceil() as u32 + 1
        } else {
            1
        };


        let block = create_border_block()
            .title_top(create_text(title, selected))
            .title_top("[<][>]".fg(TEXT).into_right_aligned_line())
            .title_bottom(format!("[{}/{}]", current_page, total_pages).fg(TEXT).into_right_aligned_line());

        Self {
            table: Table::new(rows, [Constraint::Fill(1), Constraint::Length(22)])
                .row_highlight_style(Style::new().fg(BRAND))
                .block(block),
            offset: state.index.clone()
        }
    }
}

impl WidgetRef for SearchList<'_> {

    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        StatefulWidgetRef::render_ref(
            &self.table,
            area,
            buf,
            &mut TableState::new().with_selected_cell(self.offset.map(|it| (it, 0)))
        )
    }
}
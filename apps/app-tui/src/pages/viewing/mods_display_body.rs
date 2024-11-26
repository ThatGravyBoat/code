use crate::pages::page::Page;
use crate::pages::viewing::view_instance_section::{InstanceTab, InstanceViewData};
use crate::utils::extensions::stylize_ext::*;
use crate::utils::profile_files_utils::{remove_mod, toggle_mod, ProfileFileExt};
use crate::utils::ui::{center_layout, create_border_block, BRAND, DANGER, TEXT};
use crate::utils::utils::{get_last, get_next};
use crossterm::event::KeyCode;
use dashmap::DashMap;
use ratatui::prelude::*;
use ratatui::widgets::*;
use theseus::data::{CacheBehaviour, ModLoader, ProfileFile};
use theseus::{profile, Error};
use tokio::runtime::Runtime;
use theseus::prelude::Profile;

pub struct ModDisplayData {
    pub files: Result<DashMap<String, ProfileFile>, Error>,
    pub selected_project: Option<String>, // Project hash
    pub deleting: bool,
}

impl ModDisplayData {
    pub fn new(path: &str, runtime: &Runtime) -> Self {
        Self {
            files: runtime.block_on(profile::get_projects(path, Some(CacheBehaviour::StaleWhileRevalidate))),
            selected_project: None,
            deleting: false,
        }
    }
}

pub struct ModDisplayBody();
impl ModDisplayBody {

    fn sort_files(files: &DashMap<String, ProfileFile>) -> Vec<ProfileFile> {
        let mut vec = Vec::from_iter(files.iter().map(|entry| entry.value().clone()));
        vec.sort_by_key(|file| file.file_name.to_string());
        vec
    }

    fn draw_footer(data: &InstanceViewData, file: Option<&ProfileFile>, frame: &mut Frame, area: Rect) {
        if data.mods.deleting {
            if let Some(file) = file {
                frame.render_widget(
                    Paragraph::new("Press [Del] again to delete.").fg(TEXT)
                        .block(create_border_block().title(format!("Deleting: {}", file.get_display_name()).fg(TEXT))),
                    area
                );
            }
        } else {
            frame.render_widget(
                Paragraph::new("[↵] to enable/disable mod | [Del] to remove mod").fg(TEXT).block(create_border_block().title("Controls".fg(TEXT))),
                area
            );
        }
    }

    fn draw_body(profile: &Profile, files: &DashMap<String, ProfileFile>, data: &InstanceViewData, frame: &mut Frame, area: Rect) {
        let block = if profile.loader != ModLoader::Vanilla {
            create_border_block().title("Mods".fg(TEXT)).title("[+]".fg(TEXT).into_right_aligned_line())
        } else {
            create_border_block().title("Mods".fg(TEXT))
        };

        if files.is_empty() {
            block.render_widget(frame, area);

            Line::from("No Mods Found...")
                .fg(TEXT)
                .centered()
                .render_widget(frame, center_layout(area, 40, 1));
        } else {
            let [table_area, info_area] = Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]).areas(area);

            let files = Self::sort_files(files);
            let selected = data.mods.selected_project.as_ref().map(|hash| files.iter().find(|file| &file.hash == hash)).flatten();

            let selected_index = files.iter().position(|file|
                data.mods.selected_project.as_ref().map(|project| project == &file.hash).unwrap_or(false)
            );

            let rows: Vec<Row> = files.iter().map(|file| Row::new([
                if file.is_disabled() { "[✕]".fg(DANGER) } else { "[✓]".fg(BRAND) }.into_left_aligned_line(),
                file.get_display_name().fg(TEXT).into_left_aligned_line(),
                file.get_display_type().fg(TEXT).into_right_aligned_line(),
                file.get_display_size().fg(TEXT).into_right_aligned_line(),
            ])).collect();

            let table = Table::new(rows, [Constraint::Length(3), Constraint::Fill(1), Constraint::Length(10), Constraint::Length(20)])
                .header(Row::new([
                    "".fg(TEXT).into_left_aligned_line(),
                    "File Name".fg(TEXT).into_left_aligned_line(),
                    "Type".fg(TEXT).into_right_aligned_line(),
                    "Size".fg(TEXT).into_right_aligned_line(),
                ]).underlined())
                .block(block)
                .row_highlight_style(Style::new().fg(BRAND));

            frame.render_stateful_widget(
                table,
                table_area,
                &mut TableState::new().with_selected_cell(selected_index.map(|index| (index, 0)))
            );
            Self::draw_footer(data, selected, frame, info_area);
        }
    }

    fn draw_error(error: &Error, frame: &mut Frame, area: Rect) {
        create_border_block()
            .title("Mods".fg(TEXT))
            .render_widget(frame, area);

        Paragraph::new(format!("Error Occurred:\n{}", error))
            .wrap(Wrap::default())
            .fg(DANGER)
            .centered()
            .render_widget(frame, center_layout(area, 40, 5));
    }

    pub fn draw(profile: &Profile, data: &InstanceViewData, frame: &mut Frame, area: Rect) {
        match &data.mods.files {
            Ok(files) => Self::draw_body(profile, files, data, frame, area),
            Err(error) => Self::draw_error(error, frame, area),
        }
    }

    pub fn on_key_press(page: &mut Page, key: KeyCode, runtime: &Runtime) -> bool {
        let view_data = page.viewing.as_mut().unwrap();
        let profile = &page.instances[view_data.index];
        let mods = &mut view_data.mods;

        if let Ok(projects) = &mods.files {
            let files = Self::sort_files(projects);
            match key {
                KeyCode::Char('=') | KeyCode::Char('+') => {
                    if profile.loader != ModLoader::Vanilla {
                        view_data.tab = InstanceTab::Search;
                    }
                }
                KeyCode::Up => {
                    if let Some(selected_project) = &mods.selected_project {
                        let file = get_last(&files, |file| &file.hash == selected_project);
                        mods.selected_project = Some(file.hash.to_string());
                    } else if !files.is_empty() {
                        mods.selected_project = Some(files[0].hash.to_string());
                    }
                    mods.deleting = false;
                },
                KeyCode::Down => {
                    if let Some(selected_project) = &mods.selected_project {
                        let file = get_next(&files, |file| &file.hash == selected_project);
                        mods.selected_project = Some(file.hash.to_string());
                    } else if !files.is_empty() {
                        mods.selected_project = Some(files[0].hash.to_string());
                    }
                }
                KeyCode::Enter => {
                    if let Some(selected_project) = &mods.selected_project {
                        toggle_mod(profile, files, selected_project, projects, runtime);
                    }
                }
                KeyCode::Delete => {
                    if let Some(selected_project) = &mods.selected_project {
                        if mods.deleting {
                            remove_mod(profile, files, selected_project, projects, runtime);
                            mods.deleting = false;
                            mods.selected_project = None;
                        } else {
                            mods.deleting = true;
                        }
                    }
                }
                _ => return false
            }
            return true
        }
        false
    }
}
use crate::pages::page::Page;
use crate::pages::viewing::mods_display_body::*;
use crate::pages::viewing::options_display_body::{OptionsDisplayBody, OptionsDisplayData};
use crate::pages::viewing::search_display_body::{SearchDisplayBody, SearchDisplayData};
use crate::utils::extensions::stylize_ext::WidgetExt;
use crate::utils::profile_utils::ProfileExt;
use crate::utils::ui::{create_border_block, create_text, create_title, BRAND, TEXT};
use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::*;
use ratatui::text::Line;
use ratatui::widgets::block::Title;
use ratatui::Frame;
use theseus::state::Profile;
use tokio::runtime::Runtime;
use crate::pages::viewing::logs_display_body::{LogsDisplayBody, LogsDisplayData};

#[derive(Eq, PartialEq, Clone)]
pub(crate) enum InstanceTab {
    Mods,
    Search,
    Options,
    Logs,
}

pub struct InstanceViewData {
    pub index: usize,
    pub is_playing: bool,
    pub(crate) tab: InstanceTab,
    pub mods: ModDisplayData,
    pub options: OptionsDisplayData,
    pub search: SearchDisplayData,
    pub logs: LogsDisplayData,
}

impl InstanceViewData {

    pub fn new(index: usize, page: &Page, runtime: &Runtime) -> Self {
        let profile = &page.instances[index];
        Self {
            index,
            is_playing: false,
            tab: InstanceTab::Mods,
            logs: LogsDisplayData::new(profile.path.as_str(), runtime),
            mods: ModDisplayData::new(profile.path.as_str(), runtime),
            options: OptionsDisplayData::new(profile),
            search: SearchDisplayData::new(),
        }
    }
}

pub struct ViewInstanceSection();
impl ViewInstanceSection {

    fn draw_header(profile: &Profile, frame: &mut Frame, area: Rect) {
        let version_text = if let Some(loader_version) = &profile.loader_version {
            format!("{} {} ({})", profile.game_version, profile.loader.as_str(), loader_version)
        } else {
            format!("{} {}", profile.game_version, profile.loader.as_str())
        };

        create_border_block().render_widget(frame, area);
        let [text] = Layout::vertical([Constraint::Length(1)]).margin(1).areas(area);

        Line::from(version_text).right_aligned().render_widget(frame, text);
        Line::from(profile.name.to_string().fg(TEXT)).left_aligned().render_widget(frame, text);
    }

    pub fn draw(data: &mut Page, frame: &mut Frame, area: Rect) {
        if let Some(view_data) = data.viewing.as_mut() {
            let profile = &data.instances[view_data.index];

            create_border_block()
                .title(Title::from(vec![
                    "[".fg(TEXT),
                    format!("Viewing: {}", profile.name).fg(BRAND),
                    "]".fg(TEXT),
                ]))
                .title(create_title(vec![
                    create_text("Play", view_data.is_playing),
                    Line::from(" "),
                    create_text("Options", view_data.tab == InstanceTab::Options),
                ]).right_aligned())
                .render_widget(frame, area);

            let [header, content] = Layout::vertical([
                Constraint::Length(3), Constraint::Fill(1)
            ]).margin(1).areas(area);

            Self::draw_header(profile, frame, header);

            match view_data.tab {
                InstanceTab::Mods => ModDisplayBody::draw(profile, &view_data, frame, content),
                InstanceTab::Search => SearchDisplayBody::draw(view_data, frame, content),
                InstanceTab::Options => OptionsDisplayBody::draw(profile, &view_data, frame, content),
                InstanceTab::Logs => LogsDisplayBody::draw(view_data, frame, content),
            }
        } else {
            create_border_block().render_widget(frame, area);
        }
    }

    pub fn on_tick(data: &mut Page, tick_count: &u64, runtime: &Runtime) {
        // 3s
        if let Some(view_data) = &mut data.viewing {
            let profile = &data.instances[view_data.index];
            if tick_count % 30 == 0 && view_data.is_playing {
                view_data.is_playing = profile.is_playing(runtime);
            }

            match view_data.tab {
                InstanceTab::Search => SearchDisplayBody::on_tick(profile, &mut view_data.search, tick_count, runtime),
                InstanceTab::Options => OptionsDisplayBody::on_tick(profile, &mut view_data.options, tick_count, runtime),
                InstanceTab::Logs => LogsDisplayBody::on_tick(profile, &mut view_data.logs, tick_count, runtime),
                _ => {}
            }
        }
    }

    pub fn on_key_press(data: &mut Page, key: KeyCode, runtime: &Runtime) -> bool {
        let handled = if let Some(tab) = data.viewing.as_ref().map(|it| it.tab.clone()) {
            match tab {
                InstanceTab::Mods => ModDisplayBody::on_key_press(data, key, runtime),
                InstanceTab::Search => SearchDisplayBody::on_key_press(data, key, runtime),
                InstanceTab::Options => OptionsDisplayBody::on_key_press(data, key, runtime),
                InstanceTab::Logs => LogsDisplayBody::on_key_press(data, key, runtime),
            }
        } else {
            false
        };

        if let Some(view_data) = data.viewing.as_mut() {
            let profile = &mut data.instances[view_data.index];

            if handled {
                return true;
            } else if key == KeyCode::Esc && view_data.tab != InstanceTab::Mods {
                view_data.tab = InstanceTab::Mods;
                return true;
            } else if let KeyCode::Char(char) = key {
                if (char == 'p' || char == 'P') && !view_data.is_playing {
                    view_data.is_playing = profile.run(runtime);
                    return true;
                } else if char == 'o' || char == 'O' {
                    view_data.tab = InstanceTab::Options;
                    return true;
                }
            }
        }
        false
    }
}

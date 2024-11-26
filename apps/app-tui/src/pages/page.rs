use crate::pages::accounts::accounts_display::draw_accounts;
use crate::pages::creating::data::InstanceCreationData;
use crate::pages::creating::instance_creation_section::InstanceCreationSection;
use crate::pages::creating::instance_installing_section::*;
use crate::pages::viewing::instances_section::*;
use crate::pages::viewing::view_instance_section::*;
use crate::utils::extensions::stylize_ext::WidgetExt;
use crate::utils::ui::BRAND;
use crate::widgets::bordered_block::BorderedBlock;
use crate::widgets::text::CenteredText;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Stylize;
use ratatui::Frame;
use theseus::data::Credentials;
use theseus::profile::Profile;
use theseus::{minecraft_auth, profile};
use tokio::runtime::Runtime;

#[derive(Eq, PartialEq)]
pub enum Section {
    None,
    Accounts,
    Instances,
    InstanceCreation,
    InstanceInstalling,
    InstanceViewing,
}

pub struct Page {
    pub loading: bool,
    pub accounts: Vec<Credentials>,
    pub instances: Vec<Profile>,
    pub viewing: Option<InstanceViewData>,

    pub section: Section,

    pub creation_data: InstanceCreationData,
}

impl Page {

    pub(crate) fn new() -> Self {
        Self {
            loading: true,
            accounts: vec![],
            instances: vec![],
            viewing: None,

            section: Section::None,
            creation_data: InstanceCreationData::new(),
        }
    }

    pub fn load(&mut self, runtime: &Runtime) {
        if let Ok(users) = runtime.block_on(minecraft_auth::users()) {
            self.accounts = users;
        }
        if let Ok(profiles) = runtime.block_on(profile::list()) {
            self.instances = profiles;
        }
        self.creation_data.load(runtime);
        self.loading = false;
    }

    pub fn tick(&mut self, tick_count: &u64, runtime: &Runtime) {
        match self.section {
            Section::InstanceViewing => ViewInstanceSection::on_tick(self, tick_count, runtime),
            _ => {}
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        if self.loading {
            CenteredText::new("Loading...")
                .block(BorderedBlock::new())
                .render_widget(frame, area);
        } else {
            let [sidebar, main] = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(3)]).areas(area);
            let [instances, accounts] = Layout::vertical([Constraint::Fill(4), Constraint::Fill(1)]).areas(sidebar);

            InstancesSection::draw(self, frame, instances);
            draw_accounts(self, frame, accounts);

            match self.section {
                Section::InstanceCreation => InstanceCreationSection::draw(self, frame, main),
                Section::InstanceInstalling => InstanceInstallingSection::draw(self, frame, main),
                Section::InstanceViewing => ViewInstanceSection::draw(self, frame, main),
                _ => CenteredText::new("Select or Create Instance".fg(BRAND))
                    .block(BorderedBlock::new())
                    .render_widget(frame, main),
            }
        }
    }

    pub fn handle(&mut self, event: KeyEvent, runtime: &Runtime) -> bool {
        if event.kind == KeyEventKind::Press {
            let code = event.code;
            let section_handle = match self.section {
                Section::Instances => InstancesSection::on_key_press(self, code, runtime),
                Section::InstanceCreation => InstanceCreationSection::on_key_press(self, code, runtime),
                Section::InstanceInstalling => InstanceInstallingSection::on_key_press(self, code, runtime),
                Section::InstanceViewing => ViewInstanceSection::on_key_press(self, code, runtime),
                Section::None => {
                    match code {
                        KeyCode::Char('a') | KeyCode::Char('A') => self.section = Section::Accounts,
                        KeyCode::Char('i') | KeyCode::Char('I') => self.section = Section::Instances,
                        _ => return false,
                    }
                    return true
                }
                _ => false,
            };

            return if section_handle {
                true
            } else if code == KeyCode::Esc && self.section != Section::None {
                self.section = Section::None;
                true
            } else {
                false
            }
        }
        false
    }


}
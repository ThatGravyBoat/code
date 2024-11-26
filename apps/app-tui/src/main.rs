#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::pages::notifications::renderer::render_notifications;
use crate::pages::page::Page;
use crate::utils::extensions::stylize_ext::WidgetExt;
use crate::utils::profile_utils::install_mrpack;
use crate::utils::ui::create_border_block;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use ratatui::DefaultTerminal;
use std::env;
use std::time::*;
use theseus::*;
use tokio::runtime::Runtime;

mod pages;
mod utils;
mod data;
mod widgets;

static TICK_DURATION: Duration = Duration::from_millis(100);

struct App {
    runtime: Runtime,
    terminal: DefaultTerminal,
    page: Page,
    needs_loading: bool,

    last_tick: Instant,
    tick_count: u64,
}

impl App {

    fn new() -> Self {
        Self {
            runtime: tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap(),
            terminal: ratatui::init(),
            page: Page::new(),
            needs_loading: true,

            last_tick: Instant::now(),
            tick_count: 0
        }
    }

    fn tick_renderer(&mut self) {
        self.terminal.draw(|frame| {
            let [header, body] = Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(frame.area());
            Paragraph::new("Modrinth Launcher")
                .left_aligned()
                .block(create_border_block())
                .render_widget(frame, header);
            Paragraph::new("[Ctrl + C] to close.")
                .right_aligned()
                .block(create_border_block())
                .render_widget(frame, header);

            self.page.draw(frame, body);
            render_notifications(frame);
        }).expect("Failed to draw");
    }

    fn tick_page(&mut self) {
        self.tick_count += 1;
        self.last_tick = Instant::now();
        self.page.tick(&self.tick_count, &self.runtime);
    }

    // Polls events at a rate of 100ms
    fn poll_events(&mut self) -> bool {
        if event::poll(TICK_DURATION).expect("Failed to poll event") {
            self.tick_page();

            if let Event::Key(key) = event::read().expect("Failed to read event") {
                let pressing = key.kind == KeyEventKind::Press;
                if pressing && key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    return true;
                } else if !self.page.handle(key, &self.runtime) && pressing {

                }
            }
        } else {
            self.tick_page();
        }
        false
    }

    fn tick(&mut self) -> bool {
        self.tick_renderer();

        if self.needs_loading {
            self.needs_loading = false;
            self.page.load(&self.runtime);
            self.tick_renderer();
        }

        self.poll_events()
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();
    start_logger();

    let mut app = App::new();

    app.runtime.block_on(State::init()).expect("State could not be initialized");
    app.runtime.block_on(EventState::init()).expect("Event State could not be initialized");

    if args.len() > 1 {
        if args[1] == "install" {
            app.runtime.block_on(install_mrpack(&args[2])).expect("Failed to install modpack.");
        }
    }

    loop {
        if app.tick() {
            break;
        }
    }

    ratatui::restore();
}

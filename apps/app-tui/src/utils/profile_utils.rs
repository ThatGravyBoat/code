use std::path::PathBuf;
use crate::utils::utils::run_or_notify;
use http::Method;
use memoize::memoize;
use std::time::Duration;
use ratatui::prelude::Color;
use theseus::data::{ProjectType, Version};
use theseus::fetch::{fetch, fetch_json};
use theseus::prelude::Profile;
use theseus::profile::{edit, remove, run};
use theseus::ErrorKind::LauncherError;
use theseus::{process, profile, Error, ErrorKind, State};
use tokio::runtime::Runtime;
use theseus::pack::install_from::{get_profile_from_pack, CreatePackLocation};
use theseus::pack::install_mrpack::install_zipped_mrpack;
use theseus::prelude::create::profile_create;
use crate::pages::notifications::notification_handler::NotificationHandler;
use crate::utils::ui::DANGER;

#[memoize(TimeToLive: Duration::from_secs(10), Ignore: runtime)]
fn is_playing(path: String, runtime: &Runtime) -> bool {
    runtime.block_on(process::get_by_profile_path(path.as_str()))
        .map(|it| !it.is_empty())
        .unwrap_or(false)
}

// TODO clean up this shit
async fn install_project(path: &str, version: &str, loader: &str, project: &str) -> Result<bool, Error> {
    let state = &State::get().await?;
    let versions = fetch_json::<Vec<Version>>(
        Method::GET,
        format!(
            "https://api.modrinth.com/v2/project/{}/version?game_versions=[\"{}\"]&loaders=[\"{}\"]",
            project,
            version,
            loader
        ).as_str(),
        None,
        None,
        &state.fetch_semaphore,
        &state.pool
    ).await?;

    if version.is_empty() {
        return Err(ErrorKind::OtherError("No versions returned!".to_string()).as_error())
    }

    let latest = &versions[0];
    if let Some(file) = latest.files.iter().find(|file| file.primary).or(latest.files.first()) {
        let bytes = fetch(
            &file.url,
            file.hashes.get("sha1").map(|x| &**x),
            &state.fetch_semaphore, &state.pool
        ).await?;

        Profile::add_project_bytes(
            path,
            &file.filename,
            bytes,
            file.hashes.get("sha1").map(|x| &**x),
            ProjectType::get_from_loaders(vec![loader.to_string()]),
            &state.io_semaphore,
            &state.pool
        ).await.map(|it| true)
    } else {
        Err(ErrorKind::OtherError("Failed to get latest version".to_string()).as_error())
    }
}

pub async fn install_mrpack(path: &str) -> Result<String, Error> {
    let location = CreatePackLocation::FromFile {
        path: PathBuf::from(path)
    };
    let creation = get_profile_from_pack(location.clone());
    let profile = profile_create(
        creation.name.trim().to_string(),
        creation.game_version,
        creation.modloader,
        creation.loader_version,
        None,
        None,
        Some(true)
    ).await;
    install_zipped_mrpack(location.clone(), profile?).await
}

pub trait ProfileExt {
    fn is_playing(&self, runtime: &Runtime) -> bool;
    fn run(&self, runtime: &Runtime) -> bool;
    fn edit(&mut self, runtime: &Runtime, action: impl Fn(&mut Profile)) -> bool;
    fn delete(&mut self, runtime: &Runtime) -> bool;
    fn install(&mut self, id: String, runtime: &Runtime) -> bool;
    fn open(&self, runtime: &Runtime);
}

impl ProfileExt for Profile {

    fn is_playing(&self, runtime: &Runtime) -> bool {
        is_playing(self.path.to_string(), runtime)
    }

    fn run(&self, runtime: &Runtime) -> bool {
        run_or_notify(
            runtime,
            run(self.path.as_str()),
            |kind| {
                if let LauncherError(text) = kind {
                    return text.to_string();
                }
                "".to_string()
            }
        ).is_some()
    }

    fn edit(&mut self, runtime: &Runtime, action: impl Fn(&mut Profile)) -> bool {
        let is_successful = run_or_notify(
            runtime,
            edit(self.path.as_str(), |profile| {
                action(profile);
                async { Ok(()) }
            }),
            ToString::to_string
        ).is_some();

        if is_successful {
            action(self);
            true
        } else {
            false
        }
    }

    fn delete(&mut self, runtime: &Runtime) -> bool {
        run_or_notify(
            runtime,
            remove(self.path.as_str()),
            |kind| format!("{}", kind)
        ).is_some()
    }

    fn install(&mut self, project_id: String, runtime: &Runtime) -> bool {
        run_or_notify(
            runtime,
            install_project(
                self.path.as_str(),
                self.game_version.as_str(),
                self.loader.as_str(),
                project_id.as_str()
            ),
            |kind| format!("{}", kind)
        ).is_some()
    }

    fn open(&self, runtime: &Runtime) {
        if let Some(path) = run_or_notify(runtime, profile::get_full_path(&self.path), ToString::to_string) {
            if open::that(path.as_os_str()).is_err() {
                NotificationHandler::push(
                    "Failed to open instance",
                    None,
                    (Color::White, DANGER),
                    Duration::from_secs(2)
                );
            }
        }
    }
}
use dashmap::DashMap;
use theseus::data::{ProfileFile, ProjectType};
use theseus::prelude::Profile;
use theseus::profile;
use tokio::runtime::Runtime;

const KB: f64 = 1000.0;
const MB: f64 = KB * 1000.0;
const GB: f64 = MB * 1000.0;

pub trait ProfileFileExt {
    fn is_disabled(&self) -> bool;
    fn get_display_name(&self) -> String;
    fn get_display_size(&self) -> String;
    fn get_display_type(&self) -> String;
}

impl ProfileFileExt for ProfileFile {
    fn is_disabled(&self) -> bool {
        self.file_name.ends_with(".disabled")
    }

    fn get_display_name(&self) -> String {
        strip(strip(self.file_name.as_str(), ".disabled"), ".jar").to_string()
    }

    fn get_display_size(&self) -> String {
        let size = self.size as f64;
        if size >= GB {
            format!("{:.2} gb", size / GB)
        } else if size >= MB {
            format!("{:.2} mb", size / MB)
        } else {
            format!("{:.2} kb", size / KB)
        }
    }

    fn get_display_type(&self) -> String {
        match self.project_type {
            ProjectType::Mod => "Mod".to_string(),
            ProjectType::DataPack => "Datapack".to_string(),
            ProjectType::ResourcePack => "Resourcepack".to_string(),
            ProjectType::ShaderPack => "Shaderpack".to_string(),
        }
    }
}

fn strip<'a>(text: &'a str, suffix: &'a str) -> &'a str {
    text.strip_suffix(suffix).unwrap_or(text)
}

pub(crate) fn toggle_mod(profile: &Profile, files: Vec<ProfileFile>, hash: &String, projects: &DashMap<String, ProfileFile>, runtime: &Runtime) {
    if let Some(file) = &mut files.iter().find(|file| &file.hash == hash) {
        let path = &format!("{}{}", file.project_type.get_folder_with_suffix(), file.file_name);
        let new_file = runtime.block_on(profile::toggle_disable_project(&profile.path, path)).expect("Failed");
        let new_file_name= &new_file.strip_prefix(file.project_type.get_folder_with_suffix()).unwrap_or(new_file.as_str());

        projects.remove(path);
        projects.insert(new_file.to_string(), ProfileFile {
            hash: file.hash.to_string(),
            file_name: new_file_name.to_string(),
            size: file.size,
            metadata: file.metadata.clone(),
            update_version_id: file.update_version_id.clone(),
            project_type: file.project_type,
        });
    }
}

pub(crate) fn remove_mod(profile: &Profile, files: Vec<ProfileFile>, hash: &String, projects: &DashMap<String, ProfileFile>, runtime: &Runtime) {
    if let Some(file) = &mut files.iter().find(|file| &file.hash == hash) {
        let path = &format!("{}{}", file.project_type.get_folder_with_suffix(), file.file_name);
        runtime.block_on(profile::remove_project(&profile.path, path)).expect("Failed");
        projects.remove(path);
    }
}
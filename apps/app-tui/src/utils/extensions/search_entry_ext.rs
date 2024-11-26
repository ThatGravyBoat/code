use crate::utils::utils::format_elapsed_time;
use theseus::state::SearchEntry;

pub trait SearchEntryExt {
    fn get_download_display(&self) -> String;
    fn get_follower_display(&self) -> String;
    fn get_modified_display(&self) -> String;
}

impl SearchEntryExt for SearchEntry {
    fn get_download_display(&self) -> String {
        let downloads = self.downloads as f64;
        if downloads >= 1_000_000.0 {
            format!("{:.1}M", downloads / 1_000_000.0)
        } else if downloads >= 1000.0 {
            format!("{:.1}K", downloads / 1000.0)
        } else {
            format!("{}", self.downloads)
        }
    }

    fn get_follower_display(&self) -> String {
        let follows = self.follows as f64;
        if follows >= 1_000_000.0 {
            format!("{:.1}M", follows / 1_000_000.0)
        } else if follows >= 1000.0 {
            format!("{:.1}K", follows / 1000.0)
        } else {
            format!("{}", self.follows)
        }
    }

    fn get_modified_display(&self) -> String {
        format_elapsed_time(self.date_modified)
    }
}
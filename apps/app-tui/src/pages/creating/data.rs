use tokio::runtime::Runtime;
use tokio::task::JoinHandle;
use theseus::data::ModLoader;
use theseus::{tags, Error};
use crate::widgets::chip::ChipState;
use crate::widgets::select::SelectState;
use crate::widgets::text_input::TextState;

pub(crate) const LOADERS: [ModLoader; 5] = [
    ModLoader::Vanilla,
    ModLoader::Forge,
    ModLoader::Fabric,
    ModLoader::Quilt,
    ModLoader::NeoForge,
];
pub(crate) const RELEASE_TYPE: &str = "release";

#[derive(Eq, PartialEq)]
pub(crate) enum SelectedInput {
    None,
    Name,
    Loader,
    Version,
}

pub struct InstanceCreationData {
    pub(crate) name: TextState,
    pub(crate) loader: ChipState<ModLoader>,
    pub(crate) game_version: SelectState<String>,
    pub(crate) input: SelectedInput,
    pub(crate) installer: Option<JoinHandle<Result<String, Error>>>,
}

impl InstanceCreationData {

    pub fn new() -> Self {
        Self {
            name: TextState::new(""),
            game_version: SelectState::new(None, vec![]),
            loader: ChipState::new(0, LOADERS),
            input: SelectedInput::None,
            installer: None,
        }
    }

    pub fn load(&mut self, runtime: &Runtime) {
        let game_versions = runtime.block_on(tags::get_game_version_tags())
            .map(|versions| versions.iter()
                .filter(|version| version.version_type == RELEASE_TYPE)
                .map(|version| version.version.clone())
                .collect::<Vec<String>>()
            )
            .unwrap_or(vec![]);
        self.game_version = SelectState::new(None, game_versions);
    }
}
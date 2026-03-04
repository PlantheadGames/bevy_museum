use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_seedling::prelude::*;
use bevy_skein::SkeinPlugin;

use assets::*;
use menu::*;

mod assets;
mod menu;

fn main() {
    App::new()
        .add_plugins((
                DefaultPlugins,
                PhysicsPlugins::default(),
                SkeinPlugin::default(),
                MeshPickingPlugin,
                SeedlingPlugin::default(),
))
        .init_state::<AssetLoadingState>()
        .init_state::<LevelState>()
        .add_plugins(MenuPlugin)
        //loading state  systems
        .add_loading_state(
            LoadingState::new(AssetLoadingState::LoadingGlb)
            .continue_to_state(AssetLoadingState::LoadingImage)
            .load_collection::<GlbAssets>(),
        )
        .add_loading_state(
            LoadingState::new(AssetLoadingState::LoadingImage)
            .continue_to_state(AssetLoadingState::LoadingSound)
            .load_collection::<ImageAssets>(),
        )
        .add_loading_state(
            LoadingState::new(AssetLoadingState::LoadingSound)
            .continue_to_state(AssetLoadingState::Done)
            .load_collection::<SoundAssets>(),
        )
        .run();
    }


#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum LevelState {
    #[default]
    StartUp,
    Menu,
    Level,
    End,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum AssetLoadingState {
    #[default]
    LoadingGlb,
    LoadingImage,
    LoadingSound,
    Done,
}



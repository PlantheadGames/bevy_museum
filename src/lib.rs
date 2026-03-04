
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_seedling::prelude::*;
use bevy_skein::SkeinPlugin;

use assets::*;
use menu::*;

mod assets;
mod menu;


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


#[cfg(test)]
mod test{
    use super::*;
#[test]
pub fn test(){
    assert_eq!(2, 1+1);
}
}

pub fn criterion_test(){
    assert_eq!(2, 1+1);
}


pub fn criterion_main_bench() {
    App::new()
        .add_plugins((
                DefaultPlugins,
                PhysicsPlugins::default(),
                SkeinPlugin::default(),
                MeshPickingPlugin,
                SeedlingPlugin::default(),
))
       .run();
    }



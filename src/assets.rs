use crate::*;

#[derive(AssetCollection, Resource)]
pub struct GlbAssets {
    #[asset(path = "test.glb#Scene0")]
    pub main_scene: Handle<Scene>,
}


#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "test.png")]
    pub test: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct SoundAssets {
    #[asset(path = "sound.wav")]
    pub test: Handle<AudioSample>,
}

use bevy::prelude::*;
use bevy::window::*;
use crate::*;

pub struct CustomPointerPlugin;

impl Plugin for CustomPointerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Level), setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<Entity, With<Window>>,
    mut cursor: Single<&mut CursorOptions, With<PrimaryWindow>>,
    images: Res<ImageAssets>,
) {
    cursor.grab_mode = CursorGrabMode::Locked;
    commands
        .entity(*window)
        .insert((CursorIcon::Custom(CustomCursor::Image(CustomCursorImage {
            handle: images.cursor.clone(),
            ..default()
        })),));
}

use crate::*;
use bevy::post_process::bloom::Bloom;

pub struct MuseumLayoutPlugin;

#[derive(Component)]
struct MuseumLayoutAsset;

impl Plugin for MuseumLayoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Level), setup);
    }
}

fn setup(
    mut commands: Commands,
    glb: Res<GlbAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        DirectionalLight {
            illuminance: 1000.0,
            ..default()
        },
        MuseumLayoutAsset,
    ));
    commands.spawn((SceneRoot(glb.test_map.clone()),
    RigidBody::Static,
    ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
//   Bloom::OLD_SCHOOL,
    ));
    
}



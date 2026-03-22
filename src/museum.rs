use crate::*;

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
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        DirectionalLight {
            illuminance: 1000.0,
            ..default()
        },
        MuseumLayoutAsset,
    ));

    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(100.0, 1.0, 100.0),
        Mesh3d(meshes.add(Cuboid::new(100.0, 1.0, 100.0))),
        MeshMaterial3d(materials.add(StandardMaterial
                {base_color_texture: Some(images.add(uv_debug_texture())),
            ..default()}
        )),
        Transform::from_xyz(0.0, -1.0, 0.0),
        MuseumLayoutAsset,
    ));

    commands
        .spawn((
            RigidBody::Dynamic,
            Collider::cuboid(1.0, 1.0, 1.0),
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(images.add(uv_debug_texture())),
                ..default()
            })),
            Transform::from_xyz(3.0, 5.0, 3.0),
            Pickable::default(),
        ));
}


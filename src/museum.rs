use crate::*;

pub struct MuseumPlugin;

#[derive(Component)]
struct MuseumLayoutAsset;

impl Plugin for MuseumPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(OnEnter(LevelState::Level), museum_setup);
    }
}

fn museum_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,){

    commands.spawn((
           DirectionalLight {
                illuminance: 1000.0,
                ..default()
            },
            MuseumLayoutAsset
            ));


     commands.spawn((
            RigidBody::Static,
            Collider::cuboid(100.0, 1.0, 100.0),
            Mesh3d(meshes.add(Cuboid::new(100.0,1.0,100.0))),
            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
            Transform::from_xyz(0.0, -1.0, 0.0),
    ));


    commands.spawn((
            RigidBody::Static,
            Collider::cuboid(1.0, 1.0, 1.0),
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
            Transform::from_xyz(3.0, 5.0, 3.0),
    ));

}

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
            Camera3d::default(),
            PlayerCamera,
            CollisionEventsEnabled,
            Mesh3d(meshes.add(Capsule3d::new(0.4, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
            Transform::from_xyz(0.0, 1.5, 0.0).looking_at(Vec3::new(-8.5, 3.0,5.5), Vec3::Y),
            CharacterControllerBundle::new(Collider::capsule(0.4, 1.0)).with_movement(
                0.92,
                7.0,
                (30.0 as avian3d::math::Scalar).to_radians(),
            ),
            Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            GravityScale(2.0),
    ));

    commands.spawn((
           DirectionalLight {
                illuminance: 1000.0,
                ..default()
            },
            MuseumLayoutAsset
            ));

    commands.spawn((
            RigidBody::Static,
            Collider::cuboid(1.0, 1.0, 1.0),
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
            Transform::from_xyz(3.0, 5.0, 3.0),
    ));

}

use crate::*;

pub struct LightPlugin;

impl Plugin for LightPlugin{
    fn build(&self, app:&mut App){
        app.add_systems(OnEnter(LevelState::Level), (ambient_light_setup, directional_light_setup));
    }
}

fn ambient_light_setup(
    mut ambient_light: ResMut<GlobalAmbientLight>
    ){
        ambient_light.color = Color::srgb(0.8,0.2,0.1);
        ambient_light.brightness= 1000.0;
        ambient_light.affects_lightmapped_meshes= true;
}

fn directional_light_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ){

    commands
        .spawn((
            Transform::from_xyz(3.0, 5.0, 100.0),
            DirectionalLight{
                color:  Color::srgb(0.8,0.1,0.1),
                illuminance: 10000.0,
                ..default()
            }
        ));

    commands
        .spawn((
            RigidBody::Dynamic,
            Collider::cuboid(1.0, 1.0, 1.0),
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(materials.add(StandardMaterial {
            ..default()
            })),
            Transform::from_xyz(3.0, 5.0, 3.0),
            Pickable::default(),
            DirectionalLight{
                color:  Color::srgb(0.8,0.1,0.1),
                illuminance: 10000.0,
                ..default()
            }
        ));
}

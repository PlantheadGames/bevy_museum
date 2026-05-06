use crate::*;
use bevy::light::CascadeShadowConfig;
use bevy::camera::visibility::NoFrustumCulling;
use bevy::post_process::bloom::Bloom;

pub struct LightPlugin;

impl Plugin for LightPlugin{
    fn build(&self, app:&mut App){
        app.add_systems(OnEnter(LevelState::Level), (ambient_light_setup, directional_light_setup, csm_setup, point_light_setup, spot_light_setup));
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


fn csm_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
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
                CascadeShadowConfig{
                    minimum_distance: 1000.0,
                    ..default()}
        ));
}

fn point_light_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
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
                PointLight{
                    shadows_enabled: true,
                    affects_lightmapped_mesh_diffuse: true,
                    ..default()
                },
                NoFrustumCulling,
        ));
    commands
        .spawn((
                Transform::from_xyz(3.0, 5.0, 3.0),
                PointLight{
                    shadows_enabled: true,
                    affects_lightmapped_mesh_diffuse: true,
                    ..default()
                }
        ));

}

fn spot_light_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    commands
        .spawn((
                RigidBody::Dynamic,
                Collider::cuboid(1.0, 1.0, 1.0),
                Mesh3d(meshes.add(Cuboid::default())),
                MeshMaterial3d(materials.add(StandardMaterial {
                    emissive: LinearRgba::new(0.0, 4000.0, 0.0, 0.0),
                    ..default()
                })),
                Transform::from_xyz(3.0, 5.0, 3.0),
                Pickable::default(),
                PointLight{
                    shadows_enabled: true,
                    affects_lightmapped_mesh_diffuse: true,
                    ..default()
                },

                Bloom {
                    intensity: 0.8,
                    ..Bloom::NATURAL
                },
        ));


    commands
        .spawn((
                Transform::from_xyz(3.0, 5.0, 3.0),
                SpotLight{
                    shadows_enabled: true,
                    affects_lightmapped_mesh_diffuse: true,
                    ..default()
                }
        ));

}


/*
fn gltf_light_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut glb: Res<Assets<Gltf>>
){
    commands.spawn((     
            Mesh3d(meshes.add(GltfAssetLabel::Mesh(1))),
            MeshMaterial3d(materials.add(StandardMaterial {
                emissive: LinearRgba::new(0.0, 4000.0, 0.0, 0.0),
                ..default()
            })),
            Transform::from_xyz(3.0, 5.0, 3.0),
            Pickable::default(),
            PointLight{
                shadows_enabled: true,
                affects_lightmapped_mesh_diffuse: true,
                ..default()
            },
    ));
}
*/

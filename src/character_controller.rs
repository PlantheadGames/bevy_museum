use crate::*;

use EulerRot::YXZ;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::window::PrimaryWindow;

const SPEED: f32 = 5.0;



#[derive(Component,Deref, DerefMut)]
pub struct Velocity(Vec3);

#[derive(Component)]
pub struct PlayerCam;

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Level), setup);
        app.add_systems(Update, move_camera.run_if(in_state(LevelState::Level)));
    }
}
fn move_camera(
    mut transform: Single<&mut Transform, With<PlayerCam>>,
    input: Res<ButtonInput<KeyCode>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    window: Single<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    if !window.focused {
        return;
    }

    let dt = time.delta_secs();
    //this sensitivity work around is in place as opening with terminal fullscreen made width and
    //height 0  so was dividing by zero
    let sensitivity: f32;
    if window.focused {
        sensitivity = 0.1;
    } else {
        sensitivity = 0.1 / window.width().min(window.height());
    }
        let (mut yaw, mut pitch, _roll) = transform.rotation.to_euler(YXZ);
        pitch -= mouse_motion.delta.y * dt * sensitivity;
        yaw -= mouse_motion.delta.x * dt * sensitivity;
        pitch = pitch.clamp(-1.57, 1.57);
        transform.rotation = Quat::from_euler(YXZ, yaw, pitch, 0.0);
    let mut delta = Vec3::ZERO;
    for key in input.get_pressed() {
        match key {
            KeyCode::KeyA => delta.x += -1.0,
            KeyCode::KeyD => delta.x += 1.0,
            KeyCode::KeyW => delta.z += 1.0,
            KeyCode::KeyS => delta.z += -1.0,
//            KeyCode::ShiftLeft => delta.y += -1.0,
//            KeyCode::Space => delta.y += 1.0,
            _ => (),
        }
    }
    let forward = transform.forward().as_vec3() *  delta.z;
    let right = transform.right().as_vec3() * delta.x;
    let up = transform.up().as_vec3() * delta.y;
    let mut to_move = forward + right + up;
    to_move = to_move.normalize_or_zero();
    transform.translation += to_move * time.delta_secs() * SPEED;
    transform.translation.y = 0.0;
    }

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    PlayerCam,
    Collider::cuboid(1.0,1.0,1.0), 
    RigidBody::Kinematic, 
    CollisionEventsEnabled, 
    Sensor,
    Velocity(Vec3::ZERO),
    LockedAxes::ROTATION_LOCKED,
           AmbientLight{
                brightness: 1000.0,
                ..default()
            },
  
    ));
}

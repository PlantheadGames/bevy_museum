use crate::*;

use avian3d::{math::*, prelude::*};
use EulerRot::YXZ;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::window::PrimaryWindow;

const SPEED: f32 = 300.0;
const JUMP_IMPULSE:f32 = 7.0;

#[derive(Resource)]
struct DoubleJumpCounter(u8);

#[derive(Message)]
enum MovementAction{
    Move(Vector3),
    Jump,
}

#[derive(Component,Deref, DerefMut)]
pub struct Velocity(Vec3);

#[derive(Component)]
pub struct PlayerCam;

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin{
    fn build(&self, app: &mut App) {
        app.insert_resource(DoubleJumpCounter(0));
        app.add_message::<MovementAction>();
        app.add_systems(OnEnter(LevelState::Level), setup);
        app.add_systems(Update, (move_camera,gravity,input).chain().run_if(in_state(LevelState::Level)));
    }
}

fn gravity(mut linear_velocity: Single<&mut LinearVelocity, With<PlayerCam>>,
    mut transform: Single<&mut Transform, With<PlayerCam>>,
    time: Res<Time>,
    mut jump_counter: ResMut<DoubleJumpCounter>
){
    linear_velocity.y -= 9.81 *time.delta_secs();
        if transform.translation.y < 0.0 {
            transform.translation.y = 0.0;
            linear_velocity.y = 0.0;
            jump_counter.0 = 0;
        }
}
fn move_camera(
    mut transform: Single<&mut Transform, With<PlayerCam>>,
    input: Res<ButtonInput<KeyCode>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    window: Single<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    mut linear_velocity: Query<&mut LinearVelocity, With<PlayerCam>>,
    mut movement_writer: MessageWriter<MovementAction>,
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
    let direction = to_move.normalize_or_zero();
    for mut linear_velocity in linear_velocity{
        linear_velocity.x = direction.x * time.delta_secs() * SPEED;
        linear_velocity.z = direction.z * time.delta_secs() * SPEED;
    
    }
    if direction != Vector3::ZERO {
        movement_writer.write(MovementAction::Move(direction));
    }

    if input.just_pressed(KeyCode::Space) {
        movement_writer.write(MovementAction::Jump);
    }
}

fn input(
    mut movement_reader: MessageReader<MovementAction>,
    mut controllers: Query<(&mut LinearVelocity, &Transform), With<PlayerCam>>,
    mut jump_counter: ResMut<DoubleJumpCounter>,
){
    for event in movement_reader.read() {
        for (mut linear_velocity, transform) in &mut controllers {
            match event {
                MovementAction::Jump => {
                    println!("{:#?}, {:#?}", linear_velocity.y, jump_counter.0);
                    if jump_counter.0 < 2 {
                        jump_counter.0 += 1;
                        linear_velocity.y = 0.0;
                        linear_velocity.y += JUMP_IMPULSE;
                    }
                }
                MovementAction::Move(direction) => {()}
                /*                    let world_direction =
                                      transform.rotation * Vec3::new(direction.x, 0.0, direction.y);
                                      let world_velocity = world_direction.normalize_or_zero() * SPEED;
                                      linear_velocity.x = world_velocity.x;
                                      linear_velocity.z = world_velocity.z;
                                      }
                                      */
        }
    }
}
}
fn setup(mut commands: Commands) {
    commands.spawn((
            Camera3d::default(),
            Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            PlayerCam,
            Collider::cuboid(1.0,1.0,1.0), 
            RigidBody::Kinematic, 
            Velocity(Vec3::ZERO),
            LockedAxes::ROTATION_LOCKED,
            AmbientLight{
                brightness: 3000.0,
                ..default()
            },

    ));
}

use crate::*;

use EulerRot::YXZ;
use avian3d::math::*;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::picking::Pickable;
use bevy::window::PrimaryWindow;

const SPEED: f32 = 300.0;
const JUMP_IMPULSE: f32 = 7.0;
const FOV: f32 = 90.0;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

#[derive(Component)]
pub struct MaxSlopeAngle(Scalar);

#[derive(Resource)]
struct DoubleJumpCounter(u8);

#[derive(Message)]
enum MovementAction {
    Move(Vector3),
    Jump,
}

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec3);

#[derive(Component)]
pub struct PlayerCam;

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DoubleJumpCounter(0));
        app.add_message::<MovementAction>();
        app.add_systems(OnEnter(LevelState::Level), setup);
        app.add_systems(
            Update,
            (move_camera, gravity, movement_action, update_grounded, drag_dragable)
            .chain()
            .run_if(in_state(LevelState::Level)),
        );
    }
}


fn update_grounded(
    mut commands: Commands,
    mut jump_counter: ResMut<DoubleJumpCounter>,
    mut query: Query<
    (Entity, &ShapeHits, &Rotation, Option<&MaxSlopeAngle>),
    With<PlayerCam>,
    >,
) {
    for (entity, hits, rotation, max_slope_angle) in &mut query {
        // The character is grounded if the shape caster has a hit with a normal
        // that isn't too steep.
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = max_slope_angle {
                (rotation * -hit.normal2).angle_between(Vector::Y).abs() <= angle.0
            } else {
                true
            }
        });
        println!("{:#?}", is_grounded);
        if is_grounded {
            jump_counter.0 = 0;
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

fn gravity(
    mut linear_velocity: Single<&mut LinearVelocity, With<PlayerCam>>,
    mut transform: Single<&mut Transform, With<PlayerCam>>,
    time: Res<Time>,
    mut jump_counter: ResMut<DoubleJumpCounter>,
) {
    linear_velocity.y -= 9.81 * time.delta_secs();
    if transform.translation.y < 0.1 {
        transform.translation.y = 0.0;
    }
    if linear_velocity.y == 0.0 {
        jump_counter.0 = 0;
    }
}


fn move_camera(
    mut transform: Single<&mut Transform, With<PlayerCam>>,
    input: Res<ButtonInput<KeyCode>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    window: Single<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    movement_writer: MessageWriter<MovementAction>,
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

    movement_direction(transform, input, movement_writer);
}

fn movement_direction(
    transform: Single<&mut Transform, With<PlayerCam>>,
    input: Res<ButtonInput<KeyCode>>,
    mut movement_writer: MessageWriter<MovementAction>,
) -> Vec3 {
    let mut delta = Vec3::ZERO;
    for key in input.get_pressed() {
        match key {
            KeyCode::KeyA => delta.x += -1.0,
            KeyCode::KeyD => delta.x += 1.0,
            KeyCode::KeyW => delta.z += 1.0,
            KeyCode::KeyS => delta.z += -1.0,
            _ => (),
        }
    }

    let forward = transform.forward().as_vec3() * delta.z;
    let right = transform.right().as_vec3() * delta.x;
    let up = transform.up().as_vec3() * delta.y;
    let to_move = forward + right + up;
    let direction = to_move.normalize_or_zero();
    if direction != Vector3::ZERO {
        movement_writer.write(MovementAction::Move(direction));
    }

    if input.just_pressed(KeyCode::Space) {
        movement_writer.write(MovementAction::Jump);
    }
    direction
}
fn movement_action(
    mut movement_reader: MessageReader<MovementAction>,
    mut controllers: Query<&mut LinearVelocity, With<PlayerCam>>,
    mut jump_counter: ResMut<DoubleJumpCounter>,
    time: Res<Time>,
) {
    for event in movement_reader.read() {
        for mut linear_velocity in &mut controllers {
            match event {
                MovementAction::Jump => {
                    println!("{:#?}, {:#?}", linear_velocity.y, jump_counter.0);
                    if jump_counter.0 < 2 {
                        jump_counter.0 += 1;
                        linear_velocity.y = 0.0;
                        linear_velocity.y += JUMP_IMPULSE;
                    }
                }
                MovementAction::Move(direction) => {
                    linear_velocity.x = direction.x * time.delta_secs() * SPEED;
                    linear_velocity.z = direction.z * time.delta_secs() * SPEED;
                }
            }
        }
    }
}
fn setup(mut commands: Commands) {
    commands.spawn((
            Camera3d::default(),
            Projection::from(PerspectiveProjection {
                fov: FOV.to_radians(),
                ..default()
            }),
            Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            PlayerCam,
            Collider::cuboid(1.0, 1.0, 1.0),
            LinearDamping(0.9),
            RigidBody::Dynamic,
            Velocity(Vec3::ZERO),
            LockedAxes::ROTATION_LOCKED,
            AmbientLight {
                brightness: 3000.0,
                ..default()
            },
            ShapeCaster::new(
                Collider::cuboid(1.0, 1.0, 1.0),
                Vector::ZERO,
                Quaternion::default(),
                Dir3::NEG_Y,
            ).with_max_distance(0.1),
                MaxSlopeAngle(PI * 0.45),
                ));
}
fn drag_dragable(
    transforms: Query<(&Transform, &mut LinearVelocity), (With<Pickable>, Without<PlayerCam>)>,
    player_transform: Single<&mut Transform, With<PlayerCam>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    for (transform, mut linear_velocity) in transforms {
        let direction = transform.translation - player_transform.translation;

        let direction_to_object = direction.normalize();
        let forward = player_transform.forward();
        let target_simularity_percent = 0.8;
        let in_view = forward.dot(direction_to_object) > target_simularity_percent;

        if player_transform.translation.distance(transform.translation) < 4.0
            && mouse.pressed(MouseButton::Left)
                && in_view
        {
            let distance_in_front = 3.5;
            //       let forward = player_transform.forward();
            let target_position = player_transform.translation + (forward * distance_in_front);
            let hold_direction = target_position - transform.translation;
            let speed = 15.0;
            linear_velocity.0 = hold_direction * speed;
        }
    }
}

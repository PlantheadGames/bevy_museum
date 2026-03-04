use EulerRot::YXZ;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct CameraPlugin;

#[derive(Component)]
pub struct PlayerCamera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_camera);
    }
}

fn move_camera(
    mut transform: Query<&mut Transform, With<PlayerCamera>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    window: Single<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let sensitivity;
    if window.focused {
        sensitivity = 0.1;
    } else {
        sensitivity = 0.1 / window.width().min(window.height());
    }

    let dt = time.delta_secs();
    for mut transform in transform.iter_mut() {
        let (mut yaw, mut pitch, _roll) = transform.rotation.to_euler(YXZ);
        pitch -= mouse_motion.delta.y * dt * sensitivity;
        yaw -= mouse_motion.delta.x * dt * sensitivity;
        pitch = pitch.clamp(-1.57, 1.57);
        transform.rotation = Quat::from_euler(YXZ, yaw, pitch, 0.0);
    }
}


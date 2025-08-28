use crate::structs::Player;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

const PLAYER_SPEED: f32 = 200.0;
const MOUSE_SENSITIVITY: f32 = 0.003;

/// System for ZQSD movement and mouse-based camera orientation (FPS style)
pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    // Shoot on left mouse click (MouseButton::Left just pressed)
    if mouse_buttons.just_pressed(MouseButton::Left) {
        // Replace this with actual shooting logic (spawn projectile, send event, etc.)
        println!("Bang! Player shot a projectile.");
    }
    let mut direction = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyZ) || keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyQ) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    // Mouse look: only when right mouse button is held (optional, can be always-on)
    let mut yaw_delta = 0.0;
    if mouse_buttons.pressed(MouseButton::Right) {
        for ev in mouse_motion_events.read() {
            yaw_delta += ev.delta.x;
        }
    }

    for mut transform in &mut query {
        // FPS-style mouse look (rotate around Z axis for 2D, or Y for 3D)
        if yaw_delta != 0.0 {
            // For 2D top-down, rotate around Z
            transform.rotation = Quat::from_rotation_z(
                transform.rotation.to_euler(EulerRot::XYZ).2 - yaw_delta * MOUSE_SENSITIVITY,
            );
        }

        // Move in facing direction (relative to rotation)
        if direction != Vec2::ZERO {
            let direction = direction.normalize();
            let speed = PLAYER_SPEED * time.delta_secs();
            // Rotate movement vector by player rotation
            let rot = transform.rotation.to_euler(EulerRot::XYZ).2;
            let move_dir = Vec2::from_angle(rot) * direction.y
                + Vec2::from_angle(rot + std::f32::consts::FRAC_PI_2) * direction.x;
            transform.translation.x += move_dir.x * speed;
            transform.translation.y += move_dir.y * speed;
        }
    }
}

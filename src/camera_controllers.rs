use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit},
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use std::{f32::consts::*, ops::Range};

/// Camera controller [`Component`].
#[derive(Component)]
pub struct CameraController {
    pub orbit_distance: f32,
    pub pitch_speed: f32,
    pub pitch_range: Range<f32>,
    pub yaw_speed: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        let max_pitch_limit = -0.1; // Maximum downward angle (keep camera above plane)
        let min_pitch_limit = -(FRAC_PI_2 - 0.01); // Maximum upward angle
        Self {
            orbit_distance: 20.0,
            pitch_speed: 0.003,
            pitch_range: min_pitch_limit..max_pitch_limit,
            yaw_speed: 0.004,
        }
    }
}

pub(crate) fn orbit(
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    accumulated_mouse_scroll: Res<AccumulatedMouseScroll>,
    mut windows: Query<(&Window, &mut CursorOptions)>,
    mut cursor_grabbed: Local<bool>,
) {
    let Ok((mut transform, mut controller)) = query.single_mut() else {
        return;
    };

    // Handle mouse wheel for orbit distance
    let scroll_amount = match accumulated_mouse_scroll.unit {
        MouseScrollUnit::Line => accumulated_mouse_scroll.delta.y,
        MouseScrollUnit::Pixel => accumulated_mouse_scroll.delta.y / 16.0,
    };
    if scroll_amount != 0.0 {
        controller.orbit_distance =
            (controller.orbit_distance - scroll_amount * 2.0).clamp(2.0, 50.0);
    }

    // Handle mouse capture
    if mouse_buttons.just_pressed(MouseButton::Right) {
        *cursor_grabbed = true;
        for (window, mut cursor_options) in &mut windows {
            if window.focused {
                cursor_options.grab_mode = CursorGrabMode::Locked;
                cursor_options.visible = false;
            }
        }
    }

    if mouse_buttons.just_released(MouseButton::Right) {
        *cursor_grabbed = false;
        for (_, mut cursor_options) in &mut windows {
            cursor_options.grab_mode = CursorGrabMode::None;
            cursor_options.visible = true;
        }
    }

    // Only rotate when mouse is captured
    if !*cursor_grabbed && scroll_amount == 0. {
        return;
    }

    let delta = mouse_motion.delta;

    // Mouse motion is one of the few inputs that should not be multiplied by delta time,
    // as we are already receiving the full movement since the last frame was rendered. Multiplying
    // by delta time here would make the movement slower that it should be.
    let delta_pitch = delta.y * controller.pitch_speed;
    let delta_yaw = delta.x * controller.yaw_speed;

    // Obtain the existing pitch, yaw, and roll values from the transform.
    let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);

    // Establish the new yaw and pitch, preventing the pitch value from exceeding our limits.
    let pitch =
        (pitch + delta_pitch).clamp(controller.pitch_range.start, controller.pitch_range.end);
    let yaw = yaw + delta_yaw;
    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

    // Adjust the translation to maintain the correct orientation toward the orbit target.
    // In our example it's a static target, but this could easily be customized.
    let target = Vec3::ZERO;
    transform.translation = target - transform.forward() * controller.orbit_distance;
}

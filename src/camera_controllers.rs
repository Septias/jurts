//! A freecam-style camera controller plugin.
//! To use in your own application:
//! - Copy the code for the [`CameraControllerPlugin`] and add the plugin to your App.
//! - Attach the [`CameraController`] component to an entity with a [`Camera3d`].
//!
//! Unlike other examples, which demonstrate an application, this demonstrates a plugin library.

use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit},
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use std::{f32::consts::*, fmt, ops::Range};

/// A freecam-style camera controller plugin.
pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, run_camera_controller);
    }
}

/// Based on Valorant's default sensitivity, not entirely sure why it is exactly 1.0 / 180.0,
/// but I'm guessing it is a misunderstanding between degrees/radians and then sticking with
/// it because it felt nice.
pub const RADIANS_PER_DOT: f32 = 1.0 / 180.0;

/// Camera controller [`Component`].
#[derive(Component)]
pub struct CameraController {
    /// Enables this [`CameraController`] when `true`.
    pub enabled: bool,
    /// Indicates if this controller has been initialized by the [`CameraControllerPlugin`].
    pub initialized: bool,
    /// Multiplier for pitch and yaw rotation speed.
    pub sensitivity: f32,
    /// [`KeyCode`] for forward translation.
    pub key_forward: KeyCode,
    /// [`KeyCode`] for backward translation.
    pub key_back: KeyCode,
    /// [`KeyCode`] for left translation.
    pub key_left: KeyCode,
    /// [`KeyCode`] for right translation.
    pub key_right: KeyCode,
    /// [`KeyCode`] for up translation.
    pub key_up: KeyCode,
    /// [`KeyCode`] for down translation.
    pub key_down: KeyCode,
    /// [`KeyCode`] to use [`run_speed`](CameraController::run_speed) instead of
    /// [`walk_speed`](CameraController::walk_speed) for translation.
    pub key_run: KeyCode,
    /// [`MouseButton`] for grabbing the mouse focus.
    pub mouse_key_cursor_grab: MouseButton,
    /// [`KeyCode`] for grabbing the keyboard focus.
    pub keyboard_key_toggle_cursor_grab: KeyCode,
    /// Multiplier for unmodified translation speed.
    pub walk_speed: f32,
    /// Multiplier for running translation speed.
    pub run_speed: f32,
    /// Multiplier for how the mouse scroll wheel modifies [`walk_speed`](CameraController::walk_speed)
    /// and [`run_speed`](CameraController::run_speed).
    pub scroll_factor: f32,
    /// Friction factor used to exponentially decay [`velocity`](CameraController::velocity) over time.
    pub friction: f32,
    /// This [`CameraController`]'s pitch rotation.
    pub pitch: f32,
    /// This [`CameraController`]'s yaw rotation.
    pub yaw: f32,
    /// This [`CameraController`]'s translation velocity.
    pub velocity: Vec3,
    pub orbit_distance: f32,
    pub pitch_speed: f32,
    // Clamp pitch to this range
    pub pitch_range: Range<f32>,
    pub roll_speed: f32,
    pub yaw_speed: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        let pitch_limit = FRAC_PI_2 - 0.01;
        Self {
            enabled: true,
            initialized: false,
            sensitivity: 1.0,
            key_forward: KeyCode::ArrowUp,
            key_back: KeyCode::ArrowDown,
            key_left: KeyCode::ArrowLeft,
            key_right: KeyCode::ArrowRight,
            key_up: KeyCode::Space,
            key_down: KeyCode::KeyQ,
            key_run: KeyCode::ShiftLeft,
            mouse_key_cursor_grab: MouseButton::Left,
            keyboard_key_toggle_cursor_grab: KeyCode::KeyM,
            walk_speed: 5.0,
            run_speed: 15.0,
            scroll_factor: 0.1,
            friction: 0.5,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
            orbit_distance: 20.0,
            pitch_speed: 0.003,
            pitch_range: -pitch_limit..pitch_limit,
            roll_speed: 1.0,
            yaw_speed: 0.004,
        }
    }
}

impl fmt::Display for CameraController {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "
Freecam Controls:
    Mouse\t- Move camera orientation
    Scroll\t- Adjust movement speed
    {:?}\t- Hold to grab cursor
    {:?}\t- Toggle cursor grab
    {:?} & {:?}\t- Fly forward & backwards
    {:?} & {:?}\t- Fly sideways left & right
    {:?} & {:?}\t- Fly up & down
    {:?}\t- Fly faster while held",
            self.mouse_key_cursor_grab,
            self.keyboard_key_toggle_cursor_grab,
            self.key_forward,
            self.key_back,
            self.key_left,
            self.key_right,
            self.key_up,
            self.key_down,
            self.key_run,
        )
    }
}

fn run_camera_controller(
    time: Res<Time<Real>>,
    mut windows: Query<(&Window, &mut CursorOptions)>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    accumulated_mouse_scroll: Res<AccumulatedMouseScroll>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut toggle_cursor_grab: Local<bool>,
    mut mouse_cursor_grab: Local<bool>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
) {
    let dt = time.delta_secs();

    let Ok((mut transform, mut controller)) = query.single_mut() else {
        return;
    };

    if !controller.initialized {
        let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
        controller.yaw = yaw;
        controller.pitch = pitch;
        controller.initialized = true;
        info!("{}", *controller);
    }
    if !controller.enabled {
        return;
    }

    let mut scroll = 0.0;

    let amount = match accumulated_mouse_scroll.unit {
        MouseScrollUnit::Line => accumulated_mouse_scroll.delta.y,
        MouseScrollUnit::Pixel => accumulated_mouse_scroll.delta.y / 16.0,
    };
    scroll += amount;
    controller.walk_speed += scroll * controller.scroll_factor * controller.walk_speed;
    controller.run_speed = controller.walk_speed * 3.0;

    // Handle key input
    let mut axis_input = Vec3::ZERO;
    if key_input.pressed(controller.key_forward) {
        axis_input.z += 1.0;
    }
    if key_input.pressed(controller.key_back) {
        axis_input.z -= 1.0;
    }
    if key_input.pressed(controller.key_right) {
        axis_input.x += 1.0;
    }
    if key_input.pressed(controller.key_left) {
        axis_input.x -= 1.0;
    }
    if key_input.pressed(controller.key_up) {
        axis_input.y += 1.0;
    }
    if key_input.pressed(controller.key_down) {
        axis_input.y -= 1.0;
    }

    let mut cursor_grab_change = false;
    if key_input.just_pressed(controller.keyboard_key_toggle_cursor_grab) {
        *toggle_cursor_grab = !*toggle_cursor_grab;
        cursor_grab_change = true;
    }
    if mouse_button_input.just_pressed(controller.mouse_key_cursor_grab) {
        *mouse_cursor_grab = true;
        cursor_grab_change = true;
    }
    if mouse_button_input.just_released(controller.mouse_key_cursor_grab) {
        *mouse_cursor_grab = false;
        cursor_grab_change = true;
    }
    let cursor_grab = *mouse_cursor_grab || *toggle_cursor_grab;

    // Update velocity
    if axis_input != Vec3::ZERO {
        let max_speed = if key_input.pressed(controller.key_run) {
            controller.run_speed
        } else {
            controller.walk_speed
        };
        controller.velocity = axis_input.normalize() * max_speed;
    } else {
        let friction = controller.friction.clamp(0.0, 1.0);
        controller.velocity *= 1.0 - friction;
        if controller.velocity.length_squared() < 1e-6 {
            controller.velocity = Vec3::ZERO;
        }
    }

    // Apply movement update
    if controller.velocity != Vec3::ZERO {
        let forward = *transform.forward();
        let right = *transform.right();
        transform.translation += controller.velocity.x * dt * right
            + controller.velocity.y * dt * Vec3::Y
            + controller.velocity.z * dt * forward;
    }

    // Handle cursor grab
    if cursor_grab_change {
        if cursor_grab {
            for (window, mut cursor_options) in &mut windows {
                if !window.focused {
                    continue;
                }

                cursor_options.grab_mode = CursorGrabMode::Locked;
                cursor_options.visible = false;
            }
        } else {
            for (_, mut cursor_options) in &mut windows {
                cursor_options.grab_mode = CursorGrabMode::None;
                cursor_options.visible = true;
            }
        }
    }

    // Handle mouse input
    if accumulated_mouse_motion.delta != Vec2::ZERO && cursor_grab {
        // Apply look update
        controller.pitch = (controller.pitch
            - accumulated_mouse_motion.delta.y * RADIANS_PER_DOT * controller.sensitivity)
            .clamp(-PI / 2., PI / 2.);
        controller.yaw -=
            accumulated_mouse_motion.delta.x * RADIANS_PER_DOT * controller.sensitivity;
        transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, controller.yaw, controller.pitch);
    }
}

pub(crate) fn orbit(
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
) {
    let delta = mouse_motion.delta;
    let mut delta_roll = 0.0;

    let Ok((mut transform, controller)) = query.single_mut() else {
        return;
    };

    if mouse_buttons.pressed(MouseButton::Left) {
        delta_roll -= 1.0;
    }
    if mouse_buttons.pressed(MouseButton::Right) {
        delta_roll += 1.0;
    }

    // Mouse motion is one of the few inputs that should not be multiplied by delta time,
    // as we are already receiving the full movement since the last frame was rendered. Multiplying
    // by delta time here would make the movement slower that it should be.
    let delta_pitch = delta.y * controller.pitch_speed;
    let delta_yaw = delta.x * controller.yaw_speed;

    // Conversely, we DO need to factor in delta time for mouse button inputs.
    delta_roll *= controller.roll_speed * time.delta_secs();

    // Obtain the existing pitch, yaw, and roll values from the transform.
    let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);

    // Establish the new yaw and pitch, preventing the pitch value from exceeding our limits.
    let pitch =
        (pitch + delta_pitch).clamp(controller.pitch_range.start, controller.pitch_range.end);
    let roll = roll + delta_roll;
    let yaw = yaw + delta_yaw;
    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

    // Adjust the translation to maintain the correct orientation toward the orbit target.
    // In our example it's a static target, but this could easily be customized.
    let target = Vec3::ZERO;
    transform.translation = target - transform.forward() * controller.orbit_distance;
}

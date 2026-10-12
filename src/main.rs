//! Usta's first desktop/3D technology spike.
//!
//! This is intentionally a disposable prototype, not the final engine architecture.

use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
};
use std::f32::consts::FRAC_PI_2;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Usta — Scene Workbench (prototype)".into(),
                resolution: (1440, 900).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.035, 0.045, 0.065)))
        .insert_resource(OrbitCamera::default())
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (control_camera, draw_ground_grid))
        .run();
}

#[derive(Resource)]
struct OrbitCamera {
    target: Vec3,
    yaw: f32,
    pitch: f32,
    distance: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            target: Vec3::new(0.0, 0.45, 0.0),
            yaw: 0.65,
            // Negative pitch places the camera above the target in this convention.
            pitch: -0.42,
            distance: 9.0,
        }
    }
}

impl OrbitCamera {
    fn position(&self) -> Vec3 {
        let rotation = Quat::from_rotation_y(self.yaw) * Quat::from_rotation_x(self.pitch);
        self.target + rotation * Vec3::new(0.0, 0.0, self.distance)
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Viewport camera. Camera movement is handled separately from scene data.
    commands.spawn((
        Name::new("Editor Camera"),
        Camera3d::default(),
        Transform::from_xyz(7.0, 5.0, 8.0).looking_at(Vec3::new(0.0, 0.45, 0.0), Vec3::Y),
    ));

    // Neutral ground surface to give the grid and object a clear visual reference.
    commands.spawn((
        Name::new("Ground"),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(40.0, 40.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.075, 0.09, 0.12))),
        Transform::from_xyz(0.0, -0.025, 0.0),
    ));

    // The first editable-object placeholder: one standard cube.
    commands.spawn((
        Name::new("Cube"),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.20, 0.54, 0.92))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    commands.spawn((
        Name::new("Key Light"),
        DirectionalLight::default(),
        Transform::from_xyz(4.0, 8.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Heads-up controls panel. This panel is informational; it is not yet an
    // interactive toolbar or a persistent scene hierarchy.
    commands.spawn((
        Name::new("Prototype Help"),
        Text::new(
            "USTA  /  SCENE WORKBENCH\n\n             VIEWPORT CONTROLS\n             Right-drag   Orbit\n             Middle-drag  Pan\n             Mouse wheel  Zoom\n             Home         Reset view\n\n             PROTOTYPE SCOPE\n             • One cube\n             • Ground grid\n             • Orbit camera",
        ),
        TextColor(Color::srgb(0.88, 0.91, 0.96)),
        Node {
            position_type: PositionType::Absolute,
            top: px(16),
            left: px(16),
            padding: UiRect::all(px(14)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.055, 0.07, 0.095, 0.92)),
    ));

    commands.spawn((
        Name::new("Prototype Outliner"),
        Text::new(
            "OUTLINER\n\n             Scene\n             ├─ Ground\n             ├─ Cube\n             ├─ Key Light\n             └─ Editor Camera\n\n             INSPECTOR\n             Object editing is not yet implemented in this spike.",
        ),
        TextColor(Color::srgb(0.76, 0.81, 0.88)),
        Node {
            position_type: PositionType::Absolute,
            top: px(16),
            right: px(16),
            padding: UiRect::all(px(14)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.055, 0.07, 0.095, 0.92)),
    ));
}

fn draw_ground_grid(mut gizmos: Gizmos) {
    // Gizmo grids are drawn in the XY plane by default; rotate onto the XZ ground.
    gizmos
        .grid(
            Isometry3d::new(
                Vec3::new(0.0, 0.002, 0.0),
                Quat::from_rotation_x(FRAC_PI_2),
            ),
            UVec2::splat(24),
            Vec2::splat(1.0),
            Color::srgb(0.22, 0.27, 0.34),
        )
        .outer_edges();

    // A small colored axis cross helps establish the world origin.
    gizmos.line(
        Vec3::new(-1.5, 0.012, 0.0),
        Vec3::new(1.5, 0.012, 0.0),
        Color::srgb(0.85, 0.25, 0.25),
    );
    gizmos.line(
        Vec3::new(0.0, 0.012, -1.5),
        Vec3::new(0.0, 0.012, 1.5),
        Color::srgb(0.25, 0.55, 0.92),
    );
}

fn control_camera(
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    mut orbit: ResMut<OrbitCamera>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Home) {
        *orbit = OrbitCamera::default();
    }

    let delta = mouse_motion.delta;

    if mouse_buttons.pressed(MouseButton::Right) {
        orbit.yaw -= delta.x * 0.006;
        orbit.pitch = (orbit.pitch + delta.y * 0.006).clamp(-1.40, 1.40);
    }

    if mouse_buttons.pressed(MouseButton::Middle) {
        let pan_scale = orbit.distance * 0.0015;
        let right = Vec3::new(orbit.yaw.cos(), 0.0, -orbit.yaw.sin());
        orbit.target -= right * delta.x * pan_scale;
        orbit.target += Vec3::Y * delta.y * pan_scale;
    }

    if mouse_scroll.delta.y != 0.0 {
        let scroll_amount = mouse_scroll.delta.y.clamp(-10.0, 10.0);
        orbit.distance = (orbit.distance * (-scroll_amount * 0.08).exp()).clamp(2.0, 40.0);
    }

    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };

    camera_transform.translation = orbit.position();
    camera_transform.look_at(orbit.target, Vec3::Y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_starts_at_the_configured_orbit_distance() {
        let orbit = OrbitCamera::default();
        let actual_distance = orbit.position().distance(orbit.target);
        assert!((actual_distance - orbit.distance).abs() < 1.0e-4);
    }

    #[test]
    fn reset_restores_a_valid_camera_distance() {
        let mut orbit = OrbitCamera::default();
        orbit.distance = 100.0;
        orbit.yaw = 100.0;

        orbit = OrbitCamera::default();

        assert!((2.0..=40.0).contains(&orbit.distance));
        assert!((orbit.position().distance(orbit.target) - orbit.distance).abs() < 1.0e-4);
    }
}

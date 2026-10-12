//! Usta's first desktop/3D technology spike.
//!
//! This is intentionally a disposable prototype, not the final engine architecture.

mod document;

use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
};
use document::{PrimitiveKind, SceneDocument, SceneObject};
use std::path::Path;

const SCENE_FILE: &str = "usta_scene.json";

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Usta — Scene Workbench (prototype)".into(),
                    resolution: (1440, 900).into(),
                    ..default()
                }),
                ..default()
            }),
            MeshPickingPlugin,
        ))
        .insert_resource(ClearColor(Color::srgb(0.035, 0.045, 0.065)))
        .insert_resource(OrbitCamera::default())
        .insert_resource(SelectedObject::default())
        .insert_resource(EditorStatus::default())
        .add_systems(Startup, setup_scene)
        .add_systems(
            Update,
            (
                control_camera,
                draw_ground_grid,
                edit_selected_object,
                update_inspector,
                handle_document_shortcuts,
                update_status_panel,
            ),
        )
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
            pitch: -0.42,
            distance: 9.0,
        }
    }
}

impl OrbitCamera {
    fn reset(&mut self) {
        *self = Self::default();
    }

    fn position(&self) -> Vec3 {
        let rotation = Quat::from_rotation_y(self.yaw) * Quat::from_rotation_x(self.pitch);
        self.target + rotation * Vec3::new(0.0, 0.0, self.distance)
    }
}

#[derive(Component)]
struct EditableObject {
    /// Stable ID that belongs to the document, not to Bevy's ECS.
    id: String,
    kind: PrimitiveKind,
}

#[derive(Component)]
struct InspectorPanel;

#[derive(Component)]
struct StatusPanel;

#[derive(Resource, Default)]
struct SelectedObject(Option<Entity>);

#[derive(Resource, Default)]
struct EditorStatus(String);

#[derive(Clone, Copy)]
enum TransformCommand {
    Translate(Vec3),
    RotateY(f32),
}

fn apply_transform_command(transform: &mut Transform, command: TransformCommand) {
    match command {
        TransformCommand::Translate(offset) => transform.translation += offset,
        TransformCommand::RotateY(radians) => transform.rotate_y(radians),
    }
}

fn select_object(click: On<PointerClick>, mut selected: ResMut<SelectedObject>) {
    selected.0 = Some(click.entity);
}

fn clear_selection(_click: On<PointerClick>, mut selected: ResMut<SelectedObject>) {
    selected.0 = None;
}

fn make_scene_object(
    id: impl Into<String>,
    name: impl Into<String>,
    transform: &Transform,
) -> SceneObject {
    SceneObject {
        id: id.into(),
        name: name.into(),
        kind: PrimitiveKind::Cube,
        translation: transform.translation.to_array(),
        rotation_xyzw: transform.rotation.to_array(),
        scale: transform.scale.to_array(),
    }
}

fn spawn_scene_object(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    object: SceneObject,
) {
    // Current MVP supports cube primitives. The enum in the document format makes this
    // explicit and lets unsupported kinds be rejected instead of silently remapped.
    match object.kind {
        PrimitiveKind::Cube => {
            let transform = Transform {
                translation: Vec3::from_array(object.translation),
                rotation: Quat::from_array(object.rotation_xyzw).normalize(),
                scale: Vec3::from_array(object.scale),
            };
            commands
                .spawn((
                    Name::new(object.name.clone()),
                    EditableObject {
                        id: object.id,
                        kind: object.kind,
                    },
                    Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                    MeshMaterial3d(materials.add(Color::srgb(0.20, 0.54, 0.92))),
                    transform,
                ))
                .observe(select_object);
        }
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Editor Camera"),
        Camera3d::default(),
        Transform::from_xyz(7.0, 5.0, 8.0).looking_at(Vec3::new(0.0, 0.45, 0.0), Vec3::Y),
    ));

    commands
        .spawn((
            Name::new("Ground"),
            Mesh3d(meshes.add(Plane3d::default().mesh().size(40.0, 40.0))),
            MeshMaterial3d(materials.add(Color::srgb(0.075, 0.09, 0.12))),
            Transform::from_xyz(0.0, -0.025, 0.0),
        ))
        .observe(clear_selection);

    let default_cube = SceneObject::cube("cube-0001", "Cube", [0.0, 0.5, 0.0]);
    spawn_scene_object(&mut commands, &mut meshes, &mut materials, default_cube);

    commands.spawn((
        Name::new("Key Light"),
        DirectionalLight::default(),
        Transform::from_xyz(4.0, 8.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Name::new("Prototype Help"),
        Text::new(
            "USTA  /  SCENE WORKBENCH\n\n             VIEWPORT\n             Left-click cube  Select\n             Right-drag       Orbit\n             Middle-drag      Pan\n             Mouse wheel      Zoom\n             Home             Reset view\n\n             EDIT SELECTED CUBE\n             Arrow keys       Move X / Z\n             Page Up / Down   Move Y\n             Q / E            Rotate Y\n             Hold Shift       Larger step\n\n             PROJECT FILE\n             Ctrl+S            Save scene\n             Ctrl+O            Load scene",
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
        Name::new("Prototype Inspector"),
        InspectorPanel,
        Text::new(inspector_text(None, &[String::from("Cube")])),
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

    commands.spawn((
        Name::new("Document Status"),
        StatusPanel,
        Text::new("Ready — Ctrl+S saves, Ctrl+O loads usta_scene.json."),
        TextColor(Color::srgb(0.76, 0.92, 0.82)),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(16),
            left: px(16),
            padding: UiRect::all(px(10)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.055, 0.07, 0.095, 0.92)),
    ));
}

fn inspector_text(
    selection: Option<(&EditableObject, &Name, &Transform)>,
    object_names: &[String],
) -> String {
    let selected_name = selection.map(|(_, name, _)| name.as_str());
    let mut panel = String::from("OUTLINER\n\nScene\n  Ground\n");
    for name in object_names {
        let marker = if Some(name.as_str()) == selected_name {
            "> "
        } else {
            "  "
        };
        panel.push_str(&format!("  {marker}{name}\n"));
    }
    panel.push_str("  Key Light\n  Editor Camera\n\nINSPECTOR\n");

    match selection {
        Some((object, name, transform)) => panel.push_str(&format!(
            "Selected: {} ({})\n\nPOSITION\nX  {:>7.2}\nY  {:>7.2}\nZ  {:>7.2}\n\n             ROTATION Y\n{:>7.2} rad\n\nSCALE\nX  {:>7.2}\nY  {:>7.2}\nZ  {:>7.2}",
            name.as_str(),
            object.id,
            transform.translation.x,
            transform.translation.y,
            transform.translation.z,
            transform.rotation.to_euler(EulerRot::YXZ).0,
            transform.scale.x,
            transform.scale.y,
            transform.scale.z,
        )),
        None => panel.push_str(
            "No object selected.\n\nClick a cube to inspect and edit it.",
        ),
    }

    panel
}

fn update_inspector(
    selected: Res<SelectedObject>,
    objects: Query<(&EditableObject, &Name, &Transform)>,
    mut panels: Query<&mut Text, With<InspectorPanel>>,
) {
    let selection = selected.0.and_then(|entity| objects.get(entity).ok());
    let object_names = objects
        .iter()
        .map(|(_, name, _)| name.as_str().to_owned())
        .collect::<Vec<_>>();
    let Ok(mut panel) = panels.single_mut() else {
        return;
    };
    *panel = Text::new(inspector_text(selection, &object_names));
}

fn update_status_panel(status: Res<EditorStatus>, mut panels: Query<&mut Text, With<StatusPanel>>) {
    if !status.is_changed() || status.0.is_empty() {
        return;
    }
    let Ok(mut panel) = panels.single_mut() else {
        return;
    };
    *panel = Text::new(status.0.clone());
}

fn draw_ground_grid(
    mut gizmos: Gizmos,
    selected: Res<SelectedObject>,
    objects: Query<&Transform, With<EditableObject>>,
) {
    gizmos
        .grid(
            Isometry3d::new(
                Vec3::new(0.0, 0.002, 0.0),
                Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
            ),
            UVec2::splat(24),
            Vec2::splat(1.0),
            Color::srgb(0.22, 0.27, 0.34),
        )
        .outer_edges();

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

    if let Some(entity) = selected.0
        && let Ok(transform) = objects.get(entity)
    {
        let outline = Transform::from_translation(transform.translation)
            .with_rotation(transform.rotation)
            .with_scale(transform.scale * 1.08);
        gizmos.cube(outline, Color::srgb(0.30, 1.0, 0.78));
    }
}

fn edit_selected_object(
    selected: Res<SelectedObject>,
    keys: Res<ButtonInput<KeyCode>>,
    mut objects: Query<&mut Transform, With<EditableObject>>,
) {
    let Some(entity) = selected.0 else {
        return;
    };
    let Ok(mut transform) = objects.get_mut(entity) else {
        return;
    };

    let step = if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
        0.5
    } else {
        0.1
    };

    if keys.just_pressed(KeyCode::ArrowLeft) {
        apply_transform_command(
            &mut transform,
            TransformCommand::Translate(Vec3::new(-step, 0.0, 0.0)),
        );
    }
    if keys.just_pressed(KeyCode::ArrowRight) {
        apply_transform_command(
            &mut transform,
            TransformCommand::Translate(Vec3::new(step, 0.0, 0.0)),
        );
    }
    if keys.just_pressed(KeyCode::ArrowUp) {
        apply_transform_command(
            &mut transform,
            TransformCommand::Translate(Vec3::new(0.0, 0.0, -step)),
        );
    }
    if keys.just_pressed(KeyCode::ArrowDown) {
        apply_transform_command(
            &mut transform,
            TransformCommand::Translate(Vec3::new(0.0, 0.0, step)),
        );
    }
    if keys.just_pressed(KeyCode::PageUp) {
        apply_transform_command(
            &mut transform,
            TransformCommand::Translate(Vec3::new(0.0, step, 0.0)),
        );
    }
    if keys.just_pressed(KeyCode::PageDown) {
        apply_transform_command(
            &mut transform,
            TransformCommand::Translate(Vec3::new(0.0, -step, 0.0)),
        );
    }
    if keys.just_pressed(KeyCode::KeyQ) {
        apply_transform_command(&mut transform, TransformCommand::RotateY(-0.1));
    }
    if keys.just_pressed(KeyCode::KeyE) {
        apply_transform_command(&mut transform, TransformCommand::RotateY(0.1));
    }
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
        orbit.reset();
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

fn handle_document_shortcuts(
    keys: Res<ButtonInput<KeyCode>>,
    mut status: ResMut<EditorStatus>,
    mut commands: Commands,
    objects: Query<(Entity, &EditableObject, &Name, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut selected: ResMut<SelectedObject>,
) {
    let control_pressed = keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight);
    if !control_pressed {
        return;
    }

    if keys.just_pressed(KeyCode::KeyS) {
        let mut scene_objects = objects
            .iter()
            .map(|(_, object, name, transform)| {
                let mut scene_object =
                    make_scene_object(object.id.clone(), name.as_str(), transform);
                scene_object.kind = object.kind;
                scene_object
            })
            .collect::<Vec<_>>();
        scene_objects.sort_by(|left, right| left.id.cmp(&right.id));

        let document = SceneDocument::new(scene_objects);
        status.0 = match document.save_to(Path::new(SCENE_FILE)) {
            Ok(()) => format!(
                "Saved {} object(s) to {}.",
                document.objects.len(),
                SCENE_FILE
            ),
            Err(error) => format!("Save failed; live scene unchanged. {error}"),
        };
    } else if keys.just_pressed(KeyCode::KeyO) {
        // Fully read, parse, and validate before scheduling any live-scene changes.
        match SceneDocument::load_from(Path::new(SCENE_FILE)) {
            Ok(document) => {
                for (entity, _, _, _) in &objects {
                    commands.entity(entity).despawn();
                }
                for object in document.objects.iter().cloned() {
                    spawn_scene_object(&mut commands, &mut meshes, &mut materials, object);
                }
                selected.0 = None;
                status.0 = format!(
                    "Loaded {} object(s) from {}.",
                    document.objects.len(),
                    SCENE_FILE
                );
            }
            Err(error) => {
                status.0 = format!("Load failed; current scene kept. {error}");
            }
        }
    }
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
        let mut orbit = OrbitCamera {
            distance: 100.0,
            yaw: 100.0,
            ..OrbitCamera::default()
        };

        orbit.reset();

        assert!((2.0..=40.0).contains(&orbit.distance));
        assert!((orbit.position().distance(orbit.target) - orbit.distance).abs() < 1.0e-4);
    }

    #[test]
    fn transform_command_moves_a_selected_object() {
        let mut transform = Transform::IDENTITY;
        apply_transform_command(
            &mut transform,
            TransformCommand::Translate(Vec3::new(1.0, 2.0, 3.0)),
        );
        assert_eq!(transform.translation, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn transform_command_rotates_around_y() {
        let mut transform = Transform::IDENTITY;
        apply_transform_command(&mut transform, TransformCommand::RotateY(0.5));
        assert!(
            transform
                .rotation
                .abs_diff_eq(Quat::from_rotation_y(0.5), 1.0e-5)
        );
    }
}

use crate::{
    camera_controllers::{CameraController, orbit},
    meshes::create_prisma,
    ui::setup_ui,
};
use bevy::{
    color::palettes::tailwind::*,
    feathers::{FeathersPlugins, dark_theme::create_dark_theme, theme::UiTheme},
    picking::pointer::PointerInteraction,
    prelude::*,
};
mod camera_controllers;
mod meshes;
mod ui;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FeathersPlugins,
            // CameraControllerPlugin,
            MeshPickingPlugin,
            // FpsOverlayPlugin::default(),
        ))
        .insert_resource(UiTheme(create_dark_theme()))
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(Update, (draw_mesh_intersections, orbit))
        .run();
}

/// A marker component for our shapes so we can query them separately from the ground plane.
#[derive(Component)]
struct Shape;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 10., 10.).looking_at(Vec3::ZERO, Vec3::Y),
        CameraController::default(),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(7.5))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::default().looking_to(-Vec3::Y, Vec3::X),
    ));
        // .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 10.0, 4.0),
    ));

    // draw centeroid jurt.
    create_prisma(commands, meshes, materials, 8, 1.65, 2., 1.0, Vec3::ZERO);
}

/// Returns an observer that updates the entity's material to the one specified.
fn update_material_on<E: EntityEvent>(
    new_material: Handle<StandardMaterial>,
) -> impl Fn(On<E>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
    // An observer closure that captures `new_material`. We do this to avoid needing to write four
    // versions of this observer, each triggered by a different event and with a different hardcoded
    // material. Instead, the event type is a generic, and the material is passed in.
    move |event, mut query| {
        if let Ok(mut material) = query.get_mut(event.event_target()) {
            material.0 = new_material.clone();
        }
    }
}
/// A system that draws hit indicators for every pointer.
fn draw_mesh_intersections(pointers: Query<&PointerInteraction>, mut gizmos: Gizmos) {
    for (point, normal) in pointers
        .iter()
        .filter_map(|interaction| interaction.get_nearest_hit())
        .filter_map(|(_entity, hit)| hit.position.zip(hit.normal))
    {
        gizmos.sphere(point, 0.05, RED_500);
        gizmos.arrow(point, point + normal.normalize() * 0.5, PINK_100);
    }
}

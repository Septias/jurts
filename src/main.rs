use crate::{
    camera_controllers::{CameraController, orbit},
    tred::meshes::{JurtBlueprint, create_jurt},
    ui::{setup_ui, update_editing_mode_button_text},
};
use bevy::{
    color::palettes::tailwind::*,
    feathers::{FeathersPlugins, dark_theme::create_dark_theme, theme::UiTheme},
    picking::pointer::PointerInteraction,
    prelude::*,
};
use ui::EditingMode;
mod camera_controllers;
mod tred;
mod twod;
mod ui;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FeathersPlugins,
            MeshPickingPlugin,
            // CameraControllerPlugin,
            // FpsOverlayPlugin::default(),
        ))
        .insert_resource(UiTheme(create_dark_theme()))
        .init_resource::<EditingMode>()
        .init_resource::<JurtMaterials>()
        .init_resource::<JurtBlueprint>()
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(
            Update,
            (
                draw_mesh_intersections,
                orbit,
                update_editing_mode_button_text,
            ),
        )
        .run();
}

#[derive(Resource)]
pub(crate) struct JurtMaterials {
    plane: Handle<StandardMaterial>,
    plane_hover: Handle<StandardMaterial>,
    rot: Handle<StandardMaterial>,
}

impl FromWorld for JurtMaterials {
    fn from_world(world: &mut World) -> Self {
        let materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        Self::default(materials)
    }
}
impl JurtMaterials {
    fn default(mut materials: Mut<Assets<StandardMaterial>>) -> Self {
        let plane = materials.add(StandardMaterial {
            base_color: SLATE_900.into(),
            cull_mode: None,
            double_sided: true,
            ..Default::default()
        });
        let plane_hover = materials.add(StandardMaterial {
            base_color: SLATE_400.into(),
            cull_mode: None,
            double_sided: true,
            ..Default::default()
        });

        let rot = materials.add(StandardMaterial {
            base_color: ZINC_400.into(),
            ..Default::default()
        });

        Self {
            plane,
            plane_hover,
            rot,
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    jurt_materials: Res<JurtMaterials>,
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
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 10.0, 4.0),
    ));
    let blueprint = JurtBlueprint {
        num_sides: 8,
        side_length: 1.65,
        side_height: 2.0,
        roof_height: 1.0,
    };

    // draw centeroid jurt.
    create_jurt(
        commands,
        meshes,
        materials,
        jurt_materials,
        blueprint,
        Vec3::new(0.0, 0.0, 0.0),
    );
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

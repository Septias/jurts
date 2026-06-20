use bevy::{
    asset::RenderAssetUsages,
    color::palettes::tailwind::{PINK_100, YELLOW_800},
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};
use itertools::Itertools;
use std::f32::consts::PI;

use crate::JurtMaterials;

#[derive(Component)]
struct Shape;

#[derive(Component)]
pub struct JurtInstance {
    corners: Vec<Vec2>,
}

#[derive(Resource, Clone)]
pub(crate) struct JurtBlueprint {
    pub(crate) num_sides: usize,
    pub(crate) side_length: f32,
    pub(crate) side_height: f32,
    pub(crate) roof_height: f32,
}

impl Default for JurtBlueprint {
    fn default() -> Self {
        Self {
            num_sides: 8,
            side_length: 1.65,
            side_height: 2.,
            roof_height: 1.0,
        }
    }
}

impl JurtBlueprint {
    fn with_sides(num_sides: usize) -> Self {
        Self {
            num_sides,
            ..Default::default()
        }
    }
    fn radius(&self) -> f32 {
        2.
    }
}

/// Creates a standard rectangular plane facing the Z axis with pos1 at (0,0,0)
fn create_standard_rect_plane(width: f32, height: f32) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-width / 2.0, 0.0, 0.0],    // bottom-left
            [width / 2.0, 0.0, 0.0],     // bottom-right
            [width / 2.0, height, 0.0],  // top-right
            [-width / 2.0, height, 0.0], // top-left
        ],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ],
    )
    .with_inserted_indices(Indices::U32(vec![0, 1, 2, 0, 2, 3]))
}

/// Creates a roof plane mesh for a trapezoid shape between outer and inner positions
fn create_roof_plane(pos1: Vec3, pos2: Vec3, pos3: Vec3, pos4: Vec3) -> (Mesh, Vec3) {
    // Calculate mesh center for local positioning
    let mesh_center = (pos1 + pos2 + pos3 + pos4) / 4.0;

    // Convert world positions to local positions relative to mesh center
    let local_pos1 = pos1 - mesh_center;
    let local_pos2 = pos2 - mesh_center;
    let local_pos3 = pos3 - mesh_center;
    let local_pos4 = pos4 - mesh_center;

    // Calculate face normal using cross product
    let edge1 = local_pos2 - local_pos1;
    let edge2 = local_pos4 - local_pos1;
    let face_normal = edge1.cross(edge2).normalize();

    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![local_pos1, local_pos2, local_pos4, local_pos3],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0.0, 1.0], [0.5, 0.0], [1.0, 0.0], [0.5, 1.0]],
    )
    // Assign proper face normal to all vertices
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            (-face_normal).to_array(),
            (-face_normal).to_array(),
            (-face_normal).to_array(),
            (-face_normal).to_array(),
        ],
    )
    // After defining all the vertices and their attributes, build each triangle using the
    // indices of the vertices that make it up in a counter-clockwise order.
    .with_inserted_indices(Indices::U32(vec![
        // First triangle
        0, 3, 1, // Second triangle
        1, 3, 2,
    ]));

    (mesh, mesh_center)
}
#[allow(unused)]
pub(crate) fn create_jurt2(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    jurt_materials: Res<JurtMaterials>,
    blueprint: JurtBlueprint,
    center: Vec3,
) {
    let JurtBlueprint {
        num_sides,
        side_length,
        side_height,
        roof_height,
    } = blueprint;

    let JurtMaterials {
        plane: plane_mat,
        plane_hover,
        rot: rot_material,
    } = jurt_materials.into_inner();

    let removed = (2.0 * PI) * 0.3;
    let angle_step = (2.0 * PI - removed) / num_sides as f32;
    let radius = side_length / (2.0 * (PI / num_sides as f32).sin());
    let trunk_height = side_height + roof_height + 0.5;

    let sides = (0..num_sides + 1)
        .map(|i| {
            let angle = i as f32 * angle_step;
            let pos1 = Vec3::new(radius * angle.cos(), 0.0, radius * angle.sin());
            (pos1, angle)
        })
        .collect::<Vec<_>>();
}

/// Spawns a prisma with [num_sides] sides that have a length of [side_length].
pub(crate) fn create_jurt(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    jurt_materials: Res<JurtMaterials>,
    blueprint: JurtBlueprint,
    center: Vec3,
) {
    let JurtBlueprint {
        num_sides,
        side_length,
        side_height,
        roof_height,
    } = blueprint;

    let JurtMaterials {
        plane: plane_mat,
        plane_hover,
        rot: rot_material,
    } = jurt_materials.into_inner();

    let angle_step = 2.0 * PI / num_sides as f32;
    let radius = side_length / (2.0 * (PI / num_sides as f32).sin());
    let trunk_height = side_height + roof_height + 0.5;

    let sides = (0..num_sides + 1)
        .map(|i| {
            let angle = i as f32 * angle_step;
            let pos1 = Vec3::new(radius * angle.cos(), 0.0, radius * angle.sin());
            (pos1, angle)
        })
        .collect::<Vec<_>>();

    // spawn center trunk
    commands
        .spawn((
            Transform::from_translation(center),
            Visibility::default(),
            JurtInstance {
                corners: sides.iter().map(|(v1, _)| v1.xz()).collect(),
            },
        ))
        .with_children(|commands| {
            commands.spawn((
                Mesh3d(meshes.add(Cylinder::new(0.05, trunk_height))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: YELLOW_800.into(),
                    ..Default::default()
                })),
                Transform::from_translation(Vec3::new(0., trunk_height / 2., 0.)),
                Shape,
            ));

            // spawn planes and side rods
            for ((pos1, angle), (pos2, next_angle)) in sides.iter().tuple_windows() {
                // spawn side rod
                commands.spawn((
                    Mesh3d(meshes.add(Cylinder::new(0.02, side_height))),
                    MeshMaterial3d(rot_material.clone()),
                    Transform::from_translation(pos1 + Vec3::new(0., side_height / 2.0, 0.)),
                    Shape,
                ));

                let plane_center = (pos1 + pos2) / 2.0;
                let side_length_actual = pos1.distance(pos2.clone());

                let up = Vec3::Y;
                let forward = plane_center.normalize(); // Face inward toward center
                let right = up.cross(forward).normalize();
                let corrected_up = forward.cross(right);

                let rotation_matrix = Mat3::from_cols(right, corrected_up, forward);
                let rotation = Quat::from_mat3(&rotation_matrix);

                let transform = Transform {
                    translation: plane_center,
                    rotation,
                    scale: Vec3::ONE,
                };

                // spawn side plane
                commands
                    .spawn((
                        Mesh3d(
                            meshes.add(create_standard_rect_plane(side_length_actual, side_height)),
                        ),
                        MeshMaterial3d(plane_mat.clone()),
                        Shape,
                        transform,
                    ))
                    .observe(on_plane_hover_single::<Pointer<Over>>(plane_hover.clone()))
                    .observe(on_plane_hover_single::<Pointer<Out>>(plane_mat.clone()));

                // spawn head plane
                let pos1 = Vec3::new(radius * angle.cos(), side_height, radius * angle.sin());
                let pos2 = Vec3::new(
                    radius * next_angle.cos(),
                    side_height,
                    radius * next_angle.sin(),
                );
                let radius_inner = radius * 0.2;

                let pos3 = Vec3::new(
                    radius_inner * angle.cos(),
                    side_height + roof_height,
                    radius_inner * angle.sin(),
                );
                let pos4 = Vec3::new(
                    radius_inner * next_angle.cos(),
                    side_height + roof_height,
                    radius_inner * next_angle.sin(),
                );

                let (roof_mesh, mesh_center) = create_roof_plane(pos1, pos2, pos3, pos4);

                commands
                    .spawn((
                        Mesh3d(meshes.add(roof_mesh)),
                        MeshMaterial3d(plane_mat.clone()),
                        Shape,
                        Transform::from_translation(mesh_center),
                    ))
                    .observe(on_plane_hover_single::<Pointer<Over>>(plane_hover.clone()))
                    .observe(on_plane_hover_single::<Pointer<Out>>(plane_mat.clone()));
            }
        })
        .observe(on_jurt_hover::<Pointer<Out>>(plane_mat.clone()))
        .observe(on_jurt_hover::<Pointer<Over>>(plane_hover.clone()));
}

// Returns the best center position for a new jurt with mouse position pos.
fn get_jurt_center(
    query: Query<(&Transform, &JurtInstance)>,
    adding: &Res<JurtBlueprint>,
    pos: Vec2,
) -> Vec2 {
    let hyp = adding.radius();
    let best = (Vec2::default(), Vec2::default(), Entity::PLACEHOLDER);

    for (trans, jurt) in query.iter() {
        for (a, b) in jurt.corners.iter().tuple_windows() {
            let center = trans.translation.xz();
            let (a, b) = (a + center, b + center);
            let middle = (a + b) / 2.0;
            let opp = (a - middle).length();
            let adj_len = (hyp * hyp - opp * opp).max(0.0).sqrt();
            let new_center = (a + middle) + (middle.perp().normalize() * adj_len);
        }
    }

    let (a, b, _) = best;
    let middle = (a - b) / 2.0;
    let opp = (a - middle).length();
    let adj_len = (hyp * hyp - opp * opp).max(0.0).sqrt();
    let center = middle + (middle.perp().normalize() * adj_len);
    center
}

// pub(crate) fn debug_jurt_extension(
//     mut cursor_events: MessageReader<CursorMoved>,
//     query: Query<(&Transform, &JurtInstance)>,
//     adding: Res<JurtBlueprint>,
//     mut gizmos: Gizmos,
// ) {
//     for event in cursor_events.read() {
//         let pos = event.position;
//         get_jurt_center(query, &adding, pos, &mut gizmos);
//     }
// }

mod hover {
    use bevy::prelude::*;

    /// Returns an observer that updates the entity's material to the one specified.
    pub(crate) fn on_plane_hover_single<E: EntityEvent>(
        new_material: Handle<StandardMaterial>,
    ) -> impl Fn(On<E>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
        move |event, mut query| {
            if let Ok(mut material) = query.get_mut(event.event_target()) {
                material.0 = new_material.clone();
            }
        }
    }

    /// Returns an observer that updates the entity's material to the one specified.
    pub(crate) fn on_jurt_hover<E: EntityEvent>(
        new_material: Handle<StandardMaterial>,
    ) -> impl Fn(
        On<E>,
        (
            Query<&Children>,
            Query<&mut MeshMaterial3d<StandardMaterial>>,
        ),
    ) {
        move |event, query| {
            let (children, mut materials) = query;
            if let Ok(children) = children.get(event.event_target()) {
                children.iter().for_each(|child| {
                    if let Ok(mut material) = materials.get_mut(child) {
                        material.0 = new_material.clone();
                    };
                });
            }
        }
    }
}

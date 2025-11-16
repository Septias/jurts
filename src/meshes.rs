use bevy::{
    asset::RenderAssetUsages,
    color::palettes::tailwind::{RED_700, SLATE_400, SLATE_900, YELLOW_800, ZINC_400},
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};
use std::f32::consts::PI;

use crate::{
    Shape,
    meshes::hover::{on_jurt_hover, on_plane_hover_single},
};

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

/// Spawns a prisma with [num_sides] sides that have a length of [side_length].
pub(crate) fn create_prisma(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    num_sides: usize,
    side_length: f32,
    side_height: f32,
    roof_height: f32,
    center: Vec3,
) {
    let angle_step = 2.0 * PI / num_sides as f32;
    let radius = side_length / (2.0 * (PI / num_sides as f32).sin());
    let trunk_height = side_height + roof_height + 0.5;
    let hover_mat = materials.add(StandardMaterial {
        base_color: SLATE_400.into(),
        cull_mode: None,
        double_sided: true,
        ..Default::default()
    });

    let standart_mat = materials.add(StandardMaterial {
        base_color: SLATE_900.into(),
        cull_mode: None,
        double_sided: true,
        ..Default::default()
    });

    let full_hover_mat = materials.add(StandardMaterial {
        base_color: RED_700.into(),
        cull_mode: None,
        double_sided: true,
        ..Default::default()
    });

    let rot_material = materials.add(StandardMaterial {
        base_color: ZINC_400.into(),
        ..Default::default()
    });

    // spawn center trunk
    commands
        .spawn((Transform::from_translation(center),))
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
            for i in 0..num_sides {
                let angle = i as f32 * angle_step;
                let next_angle = ((i + 1) % num_sides) as f32 * angle_step;

                let pos1 = Vec3::new(radius * angle.cos(), 0.0, radius * angle.sin());
                let pos2 = Vec3::new(radius * next_angle.cos(), 0.0, radius * next_angle.sin());

                // spawn side rod
                commands.spawn((
                    Mesh3d(meshes.add(Cylinder::new(0.02, side_height))),
                    MeshMaterial3d(rot_material.clone()),
                    Transform::from_translation(pos1 + Vec3::new(0., side_height / 2.0, 0.)),
                    Shape,
                ));

                let plane_center = (pos1 + pos2) / 2.0;
                let side_length_actual = pos1.distance(pos2);

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
                        MeshMaterial3d(standart_mat.clone()),
                        Shape,
                        transform,
                    ))
                    .observe(on_plane_hover_single::<Pointer<Over>>(hover_mat.clone()))
                    .observe(on_plane_hover_single::<Pointer<Out>>(standart_mat.clone()));

                // spawn head plane
                let pos1 = Vec3::new(radius * angle.cos(), side_height, radius * angle.sin());
                let pos2 = Vec3::new(
                    radius * next_angle.cos(),
                    side_height,
                    radius * next_angle.sin(),
                );
                let radius_inner = radius * 0.2; // Make inner radius 20% of outer radius

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
                        MeshMaterial3d(standart_mat.clone()),
                        Shape,
                        Transform::from_translation(mesh_center),
                    ))
                    .observe(on_plane_hover_single::<Pointer<Over>>(hover_mat.clone()))
                    .observe(on_plane_hover_single::<Pointer<Out>>(standart_mat.clone()));
            }
        })
        .observe(on_jurt_hover::<Pointer<Out>>(full_hover_mat));
}

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

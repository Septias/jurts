use bevy::{
    asset::RenderAssetUsages,
    color::palettes::tailwind::{SLATE_900, YELLOW_800, ZINC_400},
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};
use std::f32::consts::PI;

use crate::Shape;

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

    // spawn center trunk
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.05, trunk_height))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: YELLOW_800.into(),
            ..Default::default()
        })),
        Shape,
        Transform::from_translation(center + Vec3::new(0., trunk_height / 2., 0.)),
    ));

    // spawn planes and side rods
    for i in 0..num_sides {
        let angle = i as f32 * angle_step;
        let next_angle = ((i + 1) % num_sides) as f32 * angle_step;

        let pos1 = Vec3::new(radius * angle.cos(), center.y, radius * angle.sin());
        let pos2 = Vec3::new(
            radius * next_angle.cos(),
            center.y,
            radius * next_angle.sin(),
        );

        // spawn side rod
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(0.02, side_height))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: ZINC_400.into(),
                ..Default::default()
            })),
            Shape,
            Transform::from_translation(pos1 + Vec3::new(0., side_height / 2.0, 0.)),
        ));

        let plane_center = (pos1 + pos2) / 2.0;
        let face_normal = center - plane_center; // TODO: revert this as soon as picking works from both sides

        let mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                pos1,
                pos2,
                pos2 + Vec3::new(0., side_height, 0.),
                pos1 + Vec3::new(0., side_height, 0.),
            ],
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            vec![[0.0, 1.0], [0.5, 0.0], [1.0, 0.0], [0.5, 1.0]],
        )
        // Assign proper face normal to all vertices
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            vec![
                face_normal.to_array(),
                face_normal.to_array(),
                face_normal.to_array(),
                face_normal.to_array(),
            ],
        )
        .with_inserted_indices(Indices::U32(vec![0, 3, 1, 1, 3, 2]));

        // spawn side plane
        commands.spawn((
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: SLATE_900.into(),
                cull_mode: None,
                double_sided: true,
                ..Default::default()
            })),
            Shape,
        ));

        // spawn head plane
        let pos1 = Vec3::new(radius * angle.cos(), side_height, radius * angle.sin());
        let pos2 = Vec3::new(
            radius * next_angle.cos(),
            side_height,
            radius * next_angle.sin(),
        );
        let radius_inner = radius * 0.2; // Make inner radius 70% of outer radius

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

        commands.spawn((
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: SLATE_900.into(),
                cull_mode: None,
                double_sided: true,
                ..Default::default()
            })),
            Shape,
            Transform::from_translation(mesh_center),
        ));
    }
}

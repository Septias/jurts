use bevy::{
    color::palettes::tailwind::{RED_300, SLATE_900, YELLOW_800, ZINC_400},
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
    let trunk_height = side_length + roof_height + 0.5;

    // spawn center trunk
    commands
        .spawn((
            Mesh3d(meshes.add(Cylinder::new(0.05, trunk_height))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: YELLOW_800.into(),
                ..Default::default()
            })),
            Shape,
            Transform::from_translation(center + Vec3::new(0., trunk_height / 2., 0.)),
        ))
        .with_children(|parent| {
            let center = Vec3::new(0., -trunk_height / 2. + side_height / 2., 0.);
            parent.spawn((
                Mesh3d(meshes.add(Sphere::new(0.1))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: RED_300.into(),
                    ..Default::default()
                })),
                Shape,
                Transform::from_translation(center),
            ));
            for i in 0..num_sides {
                let angle = i as f32 * angle_step;
                let next_angle = ((i + 1) % num_sides) as f32 * angle_step;

                let pos1 = Vec3::new(radius * angle.cos(), center.y, radius * angle.sin());
                let pos2 = Vec3::new(
                    radius * next_angle.cos(),
                    center.y,
                    radius * next_angle.sin(),
                );

                // spawn side rods
                parent.spawn((
                    Mesh3d(meshes.add(Cylinder::new(0.02, side_length))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: ZINC_400.into(),
                        ..Default::default()
                    })),
                    Shape,
                    Transform::from_translation(pos1),
                ));

                let plane_center = (pos1 + pos2) / 2.0;
                let face_normal = center - plane_center; // TODO: revert this as soon as picking works from both sides

                // spawn side planes
                parent.spawn((
                    Mesh3d(meshes.add(Plane3d::new(Vec3::Z, Vec2::splat(side_length / 2.0)))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: SLATE_900.into(),
                        cull_mode: None,
                        double_sided: true,
                        ..Default::default()
                    })),
                    Shape,
                    Transform::from_translation(plane_center)
                        .looking_at(plane_center + face_normal, Vec3::Y),
                ));
            }
        });
}

use bevy::{
    color::palettes::tailwind::{GRAY_700, GREEN_300},
    prelude::*,
};

/// Spawns a prisma with [num_sides] sides that have a length of [side_length].
pub(crate) fn create_prisma(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    num_sides: usize,
    side_length: f32,
    side_height: f32,
    center: Vec3,
) {
    use std::f32::consts::PI;

    let angle_step = 2.0 * PI / num_sides as f32;
    let radius = side_length / (2.0 * (PI / num_sides as f32).sin());

    let center = center + Vec3::new(0., side_height / 2., 0.);
    for i in 0..num_sides {
        let angle = i as f32 * angle_step;
        let next_angle = ((i + 1) % num_sides) as f32 * angle_step;

        let pos1 = Vec3::new(radius * angle.cos(), center.y, radius * angle.sin());
        let pos2 = Vec3::new(
            radius * next_angle.cos(),
            center.y,
            radius * next_angle.sin(),
        );

        // spawn sides
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(0.05, side_length))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: GRAY_700.into(),
                ..Default::default()
            })),
            Transform::from_translation(pos1),
        ));

        let plane_center = (pos1 + pos2) / 2.0;

        // Spawn small sphere at center
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.1))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.0, 0.0),
                ..Default::default()
            })),
            Transform::from_translation(plane_center),
        ));

        let face_normal = (plane_center - center);

        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.1))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: GREEN_300.into(),
                ..Default::default()
            })),
            Transform::from_translation(plane_center + face_normal),
        ));

        commands.spawn((
            Mesh3d(meshes.add(Plane3d::new(Vec3::Z, Vec2::splat(side_length / 2.0)))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.8, 0.2, 0.6),
                cull_mode: None,
                double_sided: true,
                ..Default::default()
            })),
            Transform::from_translation(plane_center)
                .looking_at(plane_center + face_normal, Vec3::Y),
        ));
    }
}

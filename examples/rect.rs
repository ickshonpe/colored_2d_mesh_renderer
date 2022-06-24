use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::sprite::Mesh2dHandle;
use colored_2d_mesh_renderer::*;

pub(crate) fn an_rect(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let mut rect_mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList
    );

    let min = 0.0;
    let max = 50.0;
    let z = 0.0;
    let vertices = vec![
        [min, min, z],
        [min, max, z],
        [max, max, z],
        [max, min, z],
    ];
    let indices = Indices::U32(vec![0, 2, 1, 0, 3, 2]);
    let color = Color::RED.as_rgba_u32();
    let colors = vec![color; 4];

    rect_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    rect_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    rect_mesh.set_indices(Some(indices));
    
    commands.spawn_bundle((
        ColoredMesh2d::default(),
        Transform::from_translation(900.0 * Vec3::Z),
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
        Mesh2dHandle(meshes.add(rect_mesh.clone()))
    ));

    let mut rect_mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::LineList
    );

    let min = 0.0;
    let max = 100.0;
    let z = 0.0;
    let vertices = vec![
        [min, min, z],
        [max, max, z],
        [min, max, z],
        [max, min, z],
    ];
    let indices = Indices::U32(vec![0, 1, 2, 3]);
    let color = Color::RED.as_rgba_u32();
    let colors = vec![color; 4];

    rect_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    rect_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    rect_mesh.set_indices(Some(indices));

    commands.spawn_bundle((
        ColoredMesh2d::default(),
        Transform::from_translation(-120.0 * Vec3::X + 1.0 * Vec3::Z),
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
        Mesh2dHandle(meshes.add(rect_mesh))
    ));
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(ColoredMesh2dPlugin)
    .add_startup_system(an_rect)
    .run();
}
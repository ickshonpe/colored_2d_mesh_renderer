use bevy::math::vec2;
use bevy::prelude::*;
use colored_2d_mesh_renderer::*;
use colored_2d_mesh_renderer::helpers::*;

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_quads(
    mut helper: ColoredMesh2dBuilder,
) {
    let mut quads = QuadBuffer::default();
    quads.push_rect(Vec2::ZERO, 200.0 * Vec2::ONE, 100.0, Color::RED);
    quads.push_rect(vec2(-100.0, 0.0), vec2(0.0, 100.0), 100.0, Color::GREEN);
    quads.push_rect(100.0 * Vec2::X, -100. * Vec2::Y,  100.0, Color::BLUE);
    quads.push_rect(Vec2::ZERO, -200.0 * Vec2::ONE, 100.0, Color::YELLOW);
    helper.spawn_quads(quads, Transform::default());
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(ColoredMesh2dPlugin)
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_quads)
    .run();
}
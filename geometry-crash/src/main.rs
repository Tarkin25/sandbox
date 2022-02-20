mod display_diagnostics_plugin;
mod player;
mod tile;

use bevy::prelude::*;
use bevy::render::camera::{DepthCalculation, ScalingMode};
use bevy::window::WindowMode;
use bevy_rapier2d::prelude::*;
use crate::display_diagnostics_plugin::DisplayDiagnosticsPlugin;
use crate::player::PlayerPlugin;
use crate::tile::TileBundle;

fn main() {
    dotenv::dotenv().unwrap();

    App::new()
        .insert_resource(WindowDescriptor {
            title: "Geometry Crash".to_string(),
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(DisplayDiagnosticsPlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system(add_cameras)
        .add_startup_system(setup_floor)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn add_cameras(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection = OrthographicProjection {
        far: 1000.0,
        depth_calculation: DepthCalculation::ZDifference,
        scaling_mode: ScalingMode::FixedHorizontal,
        ..Default::default()
    };
    camera.transform.scale = Vec3::new(10., 10., 1.);

    commands.spawn_bundle(camera);
    commands.spawn_bundle(UiCameraBundle::default());
}

fn setup_floor(mut commands: Commands) {
    commands.spawn_bundle(TileBundle::new(10., 1., Vec2::new(0., -2.)));
}

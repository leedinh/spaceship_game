mod asset_loader;
mod asteroids;
mod camera;
mod collision_detection;
mod debug;
mod despawn;
mod health;
mod movement;
mod schedule;
mod spaceship;
mod state;

use asset_loader::AssetLoaderPlugin;
use asteroids::AsteroidPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

fn main() {
    App::new()
        // Bevy built-ins.
        .insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 2000.,
        })
        .add_plugins(DefaultPlugins)
        // User defined plugins.
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(MovementPlugin)
        // .add_plugins(DebugPlugin)
        .add_plugins((SpaceshipPlugin, AsteroidPlugin))
        .add_plugins(collision_detection::CollisionDetectionPlugin)
        .add_plugins(despawn::DespawnPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(schedule::SchedulePlugin)
        .add_plugins(state::GameStatePlugin)
        .run();
}

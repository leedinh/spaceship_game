mod asset_loader;
mod asteroids;
mod camera;
mod debug;
mod movement;
mod spaceship;

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
        .add_plugins(CameraPlugin)
        .run();
}

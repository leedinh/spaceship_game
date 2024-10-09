use std::ops::Range;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    asset_loader::SceneAssets,
    movement::{Acceleration, MovingObjectBundle, Velocity},
};

const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 1.0;
const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..50.0;

#[derive(Component, Debug)]
pub struct AsteroidPlugin;

#[derive(Component, Debug)]
pub struct Asteroid;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        });
        app.add_systems(Update, (spawn_asteroid, roll_asteroid));
    }
}

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    pub timer: Timer,
}

fn spawn_asteroid(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    scene_asssets: Res<SceneAssets>,
) {
    timer.timer.tick(time.delta());
    if !timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    let translation = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X.clone()),
        0.0,
        rng.gen_range(SPAWN_RANGE_Z.clone()),
    );

    let mut random_unit_vector =
        || Vec3::new(rng.gen_range(-1.0..1.0), 0.0, rng.gen_range(-1.0..1.0)).normalize_or_zero();
    let velocity = random_unit_vector() * VELOCITY_SCALAR;
    let acceleration = random_unit_vector() * ACCELERATION_SCALAR;

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity { value: velocity },
            acceleration: Acceleration::new(acceleration),
            model: SceneBundle {
                scene: scene_asssets.asteroid.clone(),
                transform: Transform::from_translation(translation),
                ..Default::default()
            },
        },
        Asteroid,
    ));
}

fn roll_asteroid(mut query: Query<(&Velocity, &mut Transform), With<Asteroid>>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        let roll = velocity.value.x * time.delta_seconds();
        transform.rotate(Quat::from_rotation_y(roll));
    }
}

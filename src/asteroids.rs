use std::ops::Range;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    asset_loader::SceneAssets,
    collision_detection::{Collider, CollisionDamage},
    health::Health,
    movement::{Acceleration, MovingObjectBundle, Velocity},
    schedule::InGameSet,
};

const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 1.0;
const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..50.0;
const ASTEROID_ROTATION_SPEED: f32 = 2.5;
const ASTEROID_RADIUS: f32 = 2.5;
const HEALTH: f32 = 80.0;
const COLLISION_DAMAGE: f32 = 35.0;

#[derive(Component, Debug)]
pub struct AsteroidPlugin;

#[derive(Component, Debug)]
pub struct Asteroid;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        });
        app.add_systems(
            Update,
            (spawn_asteroid, rotation_asteroid)
                .chain()
                .in_set(InGameSet::EntityUpdates),
        );
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
            collider: Collider::new(ASTEROID_RADIUS),
            model: SceneBundle {
                scene: scene_asssets.asteroid.clone(),
                transform: Transform::from_translation(translation),
                ..Default::default()
            },
        },
        Asteroid,
        Health::new(HEALTH),
        CollisionDamage::new(COLLISION_DAMAGE),
    ));
}

fn rotation_asteroid(mut query: Query<&mut Transform, With<Asteroid>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate_local_z(ASTEROID_ROTATION_SPEED * time.delta_seconds());
        transform.rotate_local_y(ASTEROID_ROTATION_SPEED * time.delta_seconds());
    }
}

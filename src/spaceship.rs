use bevy::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    collision_detection::{Collider, CollisionDamage},
    health::Health,
    movement::{Acceleration, MovingObjectBundle, Velocity},
    schedule::InGameSet,
};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const SPACESHIP_RADIUS: f32 = 3.0;
const SPACESHIP_HEALTH: f32 = 100.0;
const SPACESHIP_COLLISION_DAMAGE: f32 = 100.0;
const BULLET_SPEED: f32 = 50.0;
const BULLET_FORWARD_SPAWN_SCALAR: f32 = 5.5;
const BULLET_RADIUS: f32 = 1.0;
const BULLET_HEALTH: f32 = 1.0;
const BULLET_COLLISION_DAMAGE: f32 = 5.0;

#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct Bullet;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship).add_systems(
            Update,
            (spaceship_movement_controls, space_weapons_controls)
                .chain()
                .in_set(InGameSet::UserInput),
        );
    }
}

fn spawn_spaceship(mut commands: Commands, scene_asssets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            collider: Collider::new(SPACESHIP_RADIUS),
            model: SceneBundle {
                scene: scene_asssets.spaceship.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION)
                    .with_scale(Vec3::splat(2.))
                    .with_rotation(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2)),
                ..default()
            },
        },
        Spaceship,
        Health::new(SPACESHIP_HEALTH),
        CollisionDamage::new(SPACESHIP_COLLISION_DAMAGE),
    ));
}

fn spaceship_movement_controls(
    mut query: Query<(&mut Velocity, &mut Transform), With<Spaceship>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut velocity, mut transform)) = query.get_single_mut() else {
        return;
    };
    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    if input.pressed(KeyCode::KeyW) {
        movement = SPACESHIP_SPEED;
    } else if input.pressed(KeyCode::KeyS) {
        movement = -SPACESHIP_SPEED;
    }

    if input.pressed(KeyCode::ShiftLeft) {
        roll -= SPACESHIP_ROLL_SPEED * time.delta_seconds();
    } else if input.pressed(KeyCode::ControlLeft) {
        roll += SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }

    if input.pressed(KeyCode::KeyA) {
        rotation += SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    } else if input.pressed(KeyCode::KeyD) {
        rotation -= SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    }

    transform.rotate_y(rotation);
    transform.rotate_local_z(roll);

    velocity.value = -transform.left() * movement;
}

fn space_weapons_controls(
    mut commands: Commands,
    query: Query<&Transform, With<Spaceship>>,
    input: Res<ButtonInput<KeyCode>>,
    scene_asssets: Res<SceneAssets>,
) {
    let Ok(spaceship_transform) = query.get_single() else {
        info!("Spaceship not found.");
        return;
    };

    if input.just_pressed(KeyCode::Space) {
        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(-spaceship_transform.left() * BULLET_SPEED),
                acceleration: Acceleration::new(Vec3::ZERO),
                collider: Collider::new(BULLET_RADIUS),
                model: SceneBundle {
                    scene: scene_asssets.missiles.clone(),
                    transform: Transform::from_translation(
                        spaceship_transform.translation
                            + -spaceship_transform.left() * BULLET_FORWARD_SPAWN_SCALAR,
                    )
                    .with_scale(Vec3::splat(3.)),
                    ..Default::default()
                },
            },
            Bullet,
            Health::new(BULLET_HEALTH),
            CollisionDamage::new(BULLET_COLLISION_DAMAGE),
        ));
    }
}

use bevy::{prelude::*, utils::HashMap};

#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub collding_entities: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            collding_entities: Vec::new(),
        }
    }
}

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_detection);
    }
}

fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    // Detect collisions.
    for (entity_a, transform_a, collider_a) in query.iter() {
        for (entity_b, transform_b, collider_b) in query.iter() {
            if entity_a == entity_b {
                continue;
            }

            let distance = transform_a
                .translation()
                .distance(transform_b.translation());
            let combined_radius = collider_a.radius + collider_b.radius;

            if distance < combined_radius {
                colliding_entities
                    .entry(entity_a)
                    .or_insert_with(Vec::new)
                    .push(entity_b);
            }
        }
    }

    // Update colliding entities.
    for (entity, _, mut collider) in query.iter_mut() {
        collider.collding_entities.clear();
        if let Some(collisions) = colliding_entities.get(&entity) {
            collider.collding_entities.extend(collisions);
        }
    }
}

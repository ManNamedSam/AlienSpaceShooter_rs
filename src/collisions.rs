use bevy::{prelude::*, utils::HashMap};

use crate::{
    fighter::{IsBullet, Team},
    scene::Size,
    AppState,
};

#[derive(Component, Debug)]
pub struct Collider {
    pub size: Size,
    pub colliding_entities: Vec<Entity>,
}

impl Collider {
    pub fn new(size: Size) -> Self {
        Self {
            size,
            colliding_entities: vec![],
        }
    }
}

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_detection.run_if(in_state(AppState::Game)));
    }
}

fn collision_detection(
    mut query: Query<(Entity, &GlobalTransform, &mut Collider, &Team, &IsBullet)>,
) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    for (entiity_a, transform_a, collider_a, team_a, is_bullet_a) in query.iter() {
        for (entity_b, transform_b, collider_b, team_b, is_bullet_b) in query.iter() {
            if entiity_a != entity_b
                && team_a.value != team_b.value
                && is_bullet_a.value != is_bullet_b.value
            {
                let distance = (transform_a.translation() - transform_b.translation()).abs();

                // let distance = transform_a
                //     .translation()
                //     .distance(transform_b.translation());
                if distance.x < collider_a.size.value.x / 2.0 + collider_b.size.value.x / 2.0
                    && distance.y < collider_a.size.value.y / 2.0 + collider_b.size.value.y / 2.0
                {
                    colliding_entities
                        .entry(entiity_a)
                        .or_insert_with(Vec::new)
                        .push(entity_b);
                }
            }
        }
    }

    for (entity, _, mut collider, _, _) in query.iter_mut() {
        collider.colliding_entities.clear();
        if let Some(collisions) = colliding_entities.get(&entity) {
            collider
                .colliding_entities
                .extend(collisions.iter().copied());
        }
    }
}

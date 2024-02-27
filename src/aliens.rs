use bevy::prelude::*;
use rand::Rng;

use crate::{
    collisions::Collider,
    fighter::{Player, Reload, Team},
    movement::{Position, Velocity},
    scene::SceneAssets,
};

#[derive(Resource, Debug, Default)]
pub struct AlienRespawnTimer {
    value: f32,
}

#[derive(Component, Debug)]
pub struct Alien;

#[derive(Component, Debug)]
pub struct AlienBullet;

pub struct AliensPlugin;

impl Plugin for AliensPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AlienRespawnTimer>().add_systems(
            Update,
            (spawn_aliens, spawn_alien_bullets, handle_alien_collisions),
        );
    }
}

fn spawn_aliens(
    time: Res<Time>,
    mut commands: Commands,
    window: Query<&Window>,
    mut spawn_timer: ResMut<AlienRespawnTimer>,
    scene_assets: Res<SceneAssets>,
) {
    spawn_timer.value -= 60.0 * time.delta_seconds();
    if spawn_timer.value <= 0.0 {
        let window = window.single();
        let alien_pos_x = window.width() / 2.0;
        let alien_pos_x = window.width() / 2.0;
        let alien_pos_y: f32 = (rand::random::<f32>() * window.height()) - window.height() / 2.0;
        let alien_pos_y: f32 = (rand::random::<f32>() * window.height()) - window.height() / 2.0;
        let alien_speed: f32 = (rand::random::<f32>() * 250.) + 100.;
        commands.spawn((
            SpriteBundle {
                texture: scene_assets.alien.clone_weak(),
                transform: Transform::from_xyz(alien_pos_x, alien_pos_y, 0.0),
                ..default()
            },
            Position::new(Vec3::new(alien_pos_x, alien_pos_y, 0.)),
            Velocity::new(Vec3::new(-alien_speed, 0.0, 0.0)),
            Reload::new(rand::random::<f32>() * 120.0),
            Alien,
            Collider::new(100.0),
            Team::new(0),
        ));
        spawn_timer.value = rand::random::<f32>() * 150.0;
    }
}

const ALIEN_BULLET_SPEED: f32 = 300.0;

fn spawn_alien_bullets(
    time: Res<Time>,
    mut commands: Commands,
    player_query: Query<&Position, With<Player>>,
    mut aliens_query: Query<(&Position, &mut Reload), With<Alien>>,
    scene_assets: Res<SceneAssets>,
) {
    let player_position = player_query.single();
    for (position, mut reload) in aliens_query.iter_mut() {
        reload.value -= 60.0 * time.delta_seconds();

        if reload.value <= 0.0 {
            commands.spawn((
                SpriteBundle {
                    texture: scene_assets.alien_bullet.clone_weak(),
                    transform: Transform::from_xyz(
                        position.value.x,
                        position.value.y,
                        position.value.z,
                    ),
                    ..default()
                },
                AlienBullet,
                Velocity::new(
                    calculate_slope(&position.value, &player_position.value) * ALIEN_BULLET_SPEED,
                ),
                Position::new(position.value),
                Collider::new(10.0),
                Team::new(0),
            ));
            reload.value = rand::random::<f32>() * 180.0;
        }
    }
}

fn calculate_slope(position_from: &Vec3, position_to: &Vec3) -> Vec3 {
    let steps = (position_to.x - position_from.x)
        .abs()
        .max((position_to.y - position_from.y).abs());
    if steps == 0.0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    (position_to.clone() - position_from.clone()) / steps
}

fn handle_alien_collisions(mut commands: Commands, query: Query<(Entity, &Collider), With<Alien>>) {
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_entities.iter() {
            // Asteroid collided with another asteroid.
            if query.get(collided_entity).is_ok() {
                continue;
            }
            // Despawn the asteroid.
            commands.entity(entity).despawn_recursive();
        }
    }
}

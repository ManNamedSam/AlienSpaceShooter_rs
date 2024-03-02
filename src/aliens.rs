use bevy::prelude::*;

use crate::{
    collisions::Collider,
    explosions::Explosion,
    fighter::{IsBullet, Player, Reload, Team},
    hud::CurrentScore,
    movement::{Position, Velocity},
    scene::{SceneAssets, SceneSounds, Size},
    AppState,
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
            (
                spawn_aliens,
                spawn_alien_bullets,
                handle_alien_collisions,
                handle_alien_bullet_collisions,
            )
                .run_if(in_state(AppState::Game)),
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
        let alien_pos_y: f32 = (rand::random::<f32>() * window.height()) - window.height() / 2.0;
        let alien_speed: f32 = (rand::random::<f32>() * 250.) + 100.;
        commands.spawn((
            SpriteBundle {
                texture: scene_assets.alien.image.clone_weak(),
                transform: Transform::from_xyz(alien_pos_x, alien_pos_y, 0.0),
                ..default()
            },
            Position::new(Vec3::new(alien_pos_x, alien_pos_y, 0.)),
            Velocity::new(Vec3::new(-alien_speed, 0.0, 0.0)),
            Reload::new(rand::random::<f32>() * 120.0),
            Alien,
            Collider::new(Size::new(scene_assets.alien.dimensions)),
            Size::new(scene_assets.alien.dimensions),
            Team::new(0),
            IsBullet::new(false),
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
    scene_sounds: Res<SceneSounds>,
) {
    if let Ok(player_position) = player_query.get_single() {
        for (position, mut reload) in aliens_query.iter_mut() {
            reload.value -= 60.0 * time.delta_seconds();

            if reload.value <= 0.0 {
                commands.spawn((
                    SpriteBundle {
                        texture: scene_assets.alien_bullet.image.clone_weak(),
                        transform: Transform::from_xyz(
                            position.value.x,
                            position.value.y,
                            position.value.z,
                        ),
                        ..default()
                    },
                    AlienBullet,
                    Velocity::new(
                        calculate_slope(&position.value, &player_position.value)
                            * ALIEN_BULLET_SPEED,
                    ),
                    Position::new(position.value),
                    Collider::new(Size::new(scene_assets.alien_bullet.dimensions)),
                    Size::new(scene_assets.alien_bullet.dimensions),
                    Team::new(0),
                    IsBullet::new(true),
                ));
                reload.value = rand::random::<f32>() * 180.0;

                commands.spawn(AudioBundle {
                    source: scene_sounds.alien_fire.clone(),
                    settings: PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Remove,
                        ..default()
                    },
                    ..default()
                });
            }
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

fn handle_alien_collisions(
    mut commands: Commands,
    query: Query<(Entity, &Collider, &Position), With<Alien>>,
    scene_sounds: Res<SceneSounds>,
    scene_assets: Res<SceneAssets>,
    mut score: ResMut<CurrentScore>,
) {
    for (entity, collider, position) in query.iter() {
        for &collided_entity in collider.colliding_entities.iter() {
            // Asteroid collided with another asteroid.
            if query.get(collided_entity).is_ok() {
                continue;
            }

            for _ in 0..30 {
                let explosion = Explosion::new(position.value.x, position.value.y);

                let image = scene_assets.explosion.image.clone();

                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::Rgba {
                                red: explosion.r,
                                green: explosion.g,
                                blue: explosion.b,
                                alpha: explosion.a,
                            },
                            ..default()
                        },
                        texture: image,
                        transform: Transform::from_xyz(explosion.x, explosion.y, 0.0),
                        ..default()
                    },
                    Velocity::new(Vec3::new(explosion.dx, explosion.dy, 0.0)),
                    Position::new(Vec3::new(explosion.x, explosion.y, 0.0)),
                    explosion,
                ));
            }
            // Despawn the alien.
            commands.entity(entity).despawn_recursive();

            commands.spawn(AudioBundle {
                source: scene_sounds.alien_dies.clone(),
                settings: PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Remove,
                    ..default()
                },
                ..default()
            });
            score.value += 1;
        }
    }
}

fn handle_alien_bullet_collisions(
    mut commands: Commands,
    query: Query<(Entity, &Collider), With<AlienBullet>>,
) {
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

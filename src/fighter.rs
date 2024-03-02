use bevy::prelude::*;

use crate::{
    collisions::Collider,
    explosions::Explosion,
    hud::CurrentScore,
    movement::{Position, Velocity},
    scene::{SceneAssets, SceneSounds, Size},
    AppState,
};

const PLAYER_SPEED: f32 = 250.0;
const PLAYER_RELOAD: f32 = 12.0;
const PLAYER_BULLET_SPEED: f32 = 500.0;

#[derive(Component, Debug)]
pub struct Reload {
    pub value: f32,
}

impl Reload {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct PlayerBullet;

pub struct FighterPlugin;

#[derive(Component, Debug)]
pub struct Team {
    pub value: u8,
}

impl Team {
    pub fn new(team: u8) -> Self {
        Self { value: team }
    }
}

#[derive(Component, Debug)]
pub struct IsBullet {
    pub value: bool,
}

impl IsBullet {
    pub fn new(is_bullet: bool) -> Self {
        Self { value: is_bullet }
    }
}

#[derive(Component, Debug)]
pub struct GameOverCountdown {
    pub value: f32,
}

impl GameOverCountdown {
    pub fn new() -> Self {
        Self { value: 2.0 }
    }
}

impl Plugin for FighterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(
                Update,
                (
                    player_movement,
                    spawn_player_bullet,
                    handle_player_collisions,
                    handle_player_bullet_collisions,
                )
                    .run_if(in_state(AppState::Game)),
            );
    }
}

fn spawn_player(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: scene_assets.player.image.clone_weak(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        Player,
        Position::new(Vec3::new(0., 0., 0.)),
        Reload::new(PLAYER_RELOAD),
        Collider::new(Size::new(scene_assets.player.dimensions)),
        Size::new(scene_assets.player.dimensions),
        Team::new(1),
        IsBullet::new(false),
    ));
}

fn player_movement(
    time: Res<Time>,
    window: Query<&Window>,
    mut query: Query<(&mut Transform, &mut Position), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    scene_assets: Res<SceneAssets>,
) {
    if let Ok((mut transform, mut position)) = query.get_single_mut() {
        let mut direction_x = 0.0;
        let mut direction_y = 0.0;

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction_x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction_x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction_y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction_y += 1.0;
        }

        position.value.x += direction_x * PLAYER_SPEED * time.delta_seconds();
        position.value.y += direction_y * PLAYER_SPEED * time.delta_seconds();
        let window = window.single();
        if position.value.x
            < -(window.width() / 2.0 - scene_assets.player.dimensions.0 as f32 / 2.0)
        {
            position.value.x =
                -(window.width() / 2.0 - scene_assets.player.dimensions.0 as f32 / 2.0);
        }

        if position.value.x > 0.0 {
            position.value.x = 0.0;
        }

        if position.value.y
            < -(window.height() / 2.0 - scene_assets.player.dimensions.1 as f32 / 2.0)
        {
            position.value.y =
                -(window.height() / 2.0 - scene_assets.player.dimensions.1 as f32 / 2.0);
        }

        if position.value.y
            > (window.height() / 2.0 - scene_assets.player.dimensions.1 as f32 / 2.0)
        {
            position.value.y =
                (window.height() / 2.0 - scene_assets.player.dimensions.1 as f32 / 2.0);
        }

        transform.translation = position.value;
    }
}

fn spawn_player_bullet(
    time: Res<Time>,
    mut query: Query<(&Position, &mut Reload), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    scene_sounds: Res<SceneSounds>,
) {
    if let Ok((position, mut reload)) = query.get_single_mut() {
        reload.value -= 60.0 * time.delta_seconds();

        if keyboard_input.pressed(KeyCode::KeyF) && reload.value <= 0.0 {
            commands.spawn((
                SpriteBundle {
                    texture: scene_assets.player_bullet.image.clone_weak(),
                    transform: Transform::from_translation(position.value),
                    ..default()
                },
                PlayerBullet,
                Velocity::new(Vec3::new(PLAYER_BULLET_SPEED, 0.0, 0.0)),
                Position::new(position.value),
                Collider::new(Size::new(scene_assets.player_bullet.dimensions)),
                Size::new(scene_assets.player_bullet.dimensions),
                Team::new(1),
                IsBullet::new(true),
            ));
            reload.value = PLAYER_RELOAD;

            commands.spawn(AudioBundle {
                source: scene_sounds.player_fire.clone(),
                settings: PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Remove,
                    ..default()
                },
                ..default()
            });
        }
    }
}

fn handle_player_collisions(
    mut commands: Commands,
    query: Query<(Entity, &Collider, &Position), With<Player>>,
    scene_sounds: Res<SceneSounds>,
    scene_assets: Res<SceneAssets>,
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

            // Despawn the player.
            commands.entity(entity).despawn_recursive();
            commands.spawn(AudioBundle {
                source: scene_sounds.player_dies.clone(),
                settings: PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Remove,
                    ..default()
                },
                ..default()
            });
            commands.spawn(GameOverCountdown::new());
        }
    }
}

fn handle_player_bullet_collisions(
    mut commands: Commands,
    query: Query<(Entity, &Collider), With<PlayerBullet>>,
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

use bevy::prelude::*;

use crate::{
    movement::{Position, Velocity},
    scene::SceneAssets,
};

const PLAYER_SPEED: f32 = 250.0;
const PLAYER_RELOAD: f32 = 12.0;

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

impl Plugin for FighterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_movement, spawn_player_bullet));
    }
}

fn spawn_player(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: scene_assets.player.clone_weak(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Player,
        Position::new(Vec3::new(0., 0., 0.)),
        Reload::new(PLAYER_RELOAD),
    ));
}

fn player_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Position), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let (mut transform, mut position) = query.single_mut();
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

    transform.translation = position.value;
}

fn spawn_player_bullet(
    time: Res<Time>,
    mut query: Query<(&Position, &mut Reload), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
) {
    let (position, mut reload) = query.single_mut();

    reload.value -= 60.0 * time.delta_seconds();

    if (keyboard_input.pressed(KeyCode::ControlRight)
        || keyboard_input.pressed(KeyCode::ControlLeft))
        && reload.value <= 0.0
    {
        commands.spawn((
            SpriteBundle {
                texture: scene_assets.player_bullet.clone_weak(),
                transform: Transform::from_translation(position.value),
                ..default()
            },
            PlayerBullet,
            Velocity::new(Vec3::new(500.0, 0.0, 0.0)),
            Position::new(position.value),
        ));
        reload.value = PLAYER_RELOAD;
    }
}

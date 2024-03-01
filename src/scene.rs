use std::path::Path;

use bevy::{
    prelude::*,
    render::view::window,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::EnabledButtons,
};
use image::io::Reader;

use crate::{
    aliens::{Alien, AlienBullet},
    fighter::{GameOverCountdown, PlayerBullet},
    movement::{Position, Velocity},
    AppState,
};

const BACKGROUND_SCROLL_SPEED: f32 = 100.0;
const MAX_STARS: u32 = 500;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub player: ImageBox,
    pub player_bullet: ImageBox,
    pub background: ImageBox,
    pub alien: ImageBox,
    pub alien_bullet: ImageBox,
}

#[derive(Debug, Default)]
pub struct ImageBox {
    pub image: Handle<Image>,
    pub dimensions: (u32, u32),
}

#[derive(Component, Debug)]
pub struct Size {
    pub value: Vec3,
}

#[derive(Component, Debug)]
pub struct Background;

impl Size {
    pub fn new(size: (u32, u32)) -> Self {
        let value = Vec3::new(size.0 as f32, size.1 as f32, 0.0);
        Self { value }
    }
}

#[derive(Component, Debug)]
pub struct Star;

pub struct SceneLoaderPlugin;

impl Plugin for SceneLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, (load_assets, load_background, load_stars).chain())
            .add_systems(Update, (scroll_background, handle_stars))
            .add_systems(
                Update,
                game_over_countdown_timer.run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(AppState::Game), despawn_scene);
    }
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_assets: ResMut<SceneAssets>,
) {
    commands.spawn(Camera2dBundle::default());
    let player = ImageBox {
        image: asset_server.load("craft.png"),
        dimensions: Reader::open(Path::new("assets/craft.png"))
            .unwrap()
            .into_dimensions()
            .unwrap(),
    };
    let player_bullet = ImageBox {
        image: asset_server.load("playerBullet.png"),
        dimensions: Reader::open(Path::new("assets/playerBullet.png"))
            .unwrap()
            .into_dimensions()
            .unwrap(),
    };
    let background = ImageBox {
        image: asset_server.load("background.png"),
        dimensions: Reader::open(Path::new("assets/background.png"))
            .unwrap()
            .into_dimensions()
            .unwrap(),
    };
    let alien = ImageBox {
        image: asset_server.load("alien.png"),
        dimensions: Reader::open(Path::new("assets/alien.png"))
            .unwrap()
            .into_dimensions()
            .unwrap(),
    };

    let alien_bullet = ImageBox {
        image: asset_server.load("alienBullet.png"),
        dimensions: Reader::open(Path::new("assets/alienBullet.png"))
            .unwrap()
            .into_dimensions()
            .unwrap(),
    };

    *scene_assets = SceneAssets {
        player,
        player_bullet,
        background,
        alien,
        alien_bullet,
    };
}

fn load_background(
    mut commands: Commands,
    mut window: Query<&mut Window>,
    scene_assets: Res<SceneAssets>,
) {
    let mut window = window.single_mut();
    window.enabled_buttons = EnabledButtons {
        maximize: false,
        ..default()
    };
    window.resizable = false;
    for i in 0..2 {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(window.width(), window.height())),
                    ..default()
                },
                texture: scene_assets.background.image.clone_weak(),
                transform: Transform::from_xyz(window.width() * i as f32, 0.0, -1.0),
                ..default()
            },
            Background,
        ));
    }
}

fn load_stars(
    mut commands: Commands,
    mut window: Query<&mut Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window.single();
    for i in 0..MAX_STARS {
        let velocity = rand::random::<f32>() * 8.0;
        let color = 30 * velocity as u8;
        let velocity = Velocity::new(Vec3::new(-velocity, 0.0, 0.0));
        let position = Position {
            value: Vec3::new(
                rand::random::<f32>() * window.width() - window.width() / 2.0,
                rand::random::<f32>() * window.height() - window.height() / 2.0,
                0.0,
            ),
        };
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(1.0, 3.0))),
                material: materials.add(Color::rgb_u8(color, color, color)),
                transform: Transform::from_xyz(position.value.x, position.value.y, 0.0),
                ..default()
            },
            position,
            velocity,
            Star,
        ));
    }
}

fn handle_stars(
    time: Res<Time>,
    window: Query<&Window>,
    mut query: Query<(&mut Position, &mut Transform, &Velocity), With<Star>>,
) {
    let window = window.single();

    for (mut position, mut transform, velocity) in query.iter_mut() {
        position.value += velocity.value * 60.0 * time.delta_seconds();
        transform.translation += velocity.value * 60.0 * time.delta_seconds();

        if position.value.x < -window.width() / 2.0 {
            position.value.x += window.width();
            transform.translation.x += window.width();
        }
    }
}

fn scroll_background(
    time: Res<Time>,
    window: Query<&Window>,
    mut query: Query<(&mut Transform, &mut Sprite), With<Background>>,
) {
    let window = window.single();

    let scroll_amount = BACKGROUND_SCROLL_SPEED * time.delta_seconds();

    for (mut transform, mut sprite) in query.iter_mut() {
        transform.translation.x -= scroll_amount;

        if sprite.custom_size.unwrap().x != window.width() {
            sprite.custom_size.unwrap().x = window.width();
            sprite.custom_size.unwrap().y = window.height();

            let bg_idx = (transform.translation.x / window.width()).round();
            transform.translation.x = bg_idx * window.width();
        }

        if transform.translation.x <= -window.width() {
            transform.translation.x += 2.0 * window.width();
        }
    }
}

fn game_over_countdown_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut next_state: ResMut<NextState<AppState>>,
    mut query: Query<(Entity, &mut GameOverCountdown)>,
) {
    if let Ok((entity, mut timer)) = query.get_single_mut() {
        timer.value -= time.delta_seconds();

        if timer.value < 0.0 {
            next_state.set(AppState::IntroScreen);
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_scene(
    mut commands: Commands,
    player_bullets_query: Query<Entity, With<PlayerBullet>>,
    aliens_query: Query<Entity, With<Alien>>,
    alien_bullets_query: Query<Entity, With<AlienBullet>>,
) {
    for entity in player_bullets_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in aliens_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in alien_bullets_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

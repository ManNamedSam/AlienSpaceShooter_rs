use bevy::{
    audio::Volume,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::EnabledButtons,
};

use crate::{
    aliens::{Alien, AlienBullet},
    fighter::{GameOverCountdown, PlayerBullet},
    hud::CurrentScore,
    movement::{Position, Velocity},
    AppState,
};

const BACKGROUND_SCROLL_SPEED: f32 = 100.0;
const MAX_STARS: u32 = 500;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub player: ImageBox,
    pub player_bullet: ImageBox,
    // pub background: ImageBox,
    pub alien: ImageBox,
    pub alien_bullet: ImageBox,
    pub explosion: ImageBox,
}

#[derive(Resource, Debug, Default)]
pub struct SceneSounds {
    pub player_fire: Handle<AudioSource>,
    pub player_dies: Handle<AudioSource>,
    pub alien_fire: Handle<AudioSource>,
    pub alien_dies: Handle<AudioSource>,
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
            .init_resource::<SceneSounds>()
            .add_systems(Startup, (load_assets, load_background, load_stars, music))
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
    mut scene_sounds: ResMut<SceneSounds>,
) {
    commands.spawn(Camera2dBundle::default());
    let player = ImageBox {
        image: asset_server.load("craft.png"),
        dimensions: (75, 33),
    };
    let player_bullet = ImageBox {
        image: asset_server.load("playerBullet.png"),
        dimensions: (26, 9),
    };
    // let background = ImageBox {
    //     image: asset_server.load("background_space.png"),
    //     dimensions: (1104, 1021),
    // };
    let alien = ImageBox {
        image: asset_server.load("alien.png"),
        dimensions: (72, 34),
    };

    let alien_bullet = ImageBox {
        image: asset_server.load("alienBullet.png"),
        dimensions: (11, 11),
    };
    let explosion = ImageBox {
        image: asset_server.load("explosion.png"),
        dimensions: (96, 96),
    };

    *scene_assets = SceneAssets {
        player,
        player_bullet,
        // background,
        alien,
        alien_bullet,
        explosion,
    };

    let player_fire: Handle<AudioSource> = asset_server.load("sounds/playerFire.ogg");
    let player_dies: Handle<AudioSource> = asset_server.load("sounds/playerDies.ogg");
    let alien_fire: Handle<AudioSource> = asset_server.load("sounds/alienFire.ogg");
    let alien_dies: Handle<AudioSource> = asset_server.load("sounds/alienDies.ogg");

    *scene_sounds = SceneSounds {
        player_fire,
        player_dies,
        alien_fire,
        alien_dies,
    }
}

fn load_background(
    mut commands: Commands,
    mut window: Query<&mut Window>,
    // scene_assets: Res<SceneAssets>,
    asset_server: Res<AssetServer>,
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
                texture: asset_server.load("background_space.png"),
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
    mut score: ResMut<CurrentScore>,
) {
    if let Ok((entity, mut timer)) = query.get_single_mut() {
        timer.value -= time.delta_seconds();

        if timer.value < 0.0 {
            next_state.set(AppState::IntroScreen);
            commands.entity(entity).despawn_recursive();
            score.reset();
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

fn music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AudioBundle {
        source: asset_server.load("music/alienSpaceShooter.ogg"),
        settings: PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            volume: Volume::new(0.5),
            ..default()
        },
        ..default()
    });
}

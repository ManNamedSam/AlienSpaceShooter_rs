use std::path::Path;

use bevy::{prelude::*, render::view::window, window::EnabledButtons};
use image::io::Reader;

const BACKGROUND_SCROLL_SPEED: f32 = 100.0;

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

pub struct SceneLoaderPlugin;

impl Plugin for SceneLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, (load_assets, load_background).chain())
            .add_systems(Update, (scroll_background));
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

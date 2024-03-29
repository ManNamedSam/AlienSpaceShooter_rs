mod aliens;
mod collisions;
mod explosions;
mod fighter;
mod hud;
mod intro_screen;
mod movement;
mod scene;

use bevy::{
    audio::{AudioPlugin, SpatialScale, Volume},
    prelude::*,
};

use aliens::AliensPlugin;
use collisions::CollisionDetectionPlugin;
use explosions::ExplosionsPlugin;
use fighter::FighterPlugin;
use hud::HudPlugin;
use intro_screen::IntroScreenPlugin;
use movement::MovementPlugin;
use scene::SceneLoaderPlugin;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_app() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    main();
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_plugins(IntroScreenPlugin)
        .add_plugins(SceneLoaderPlugin)
        .add_plugins(HudPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(FighterPlugin)
        .add_plugins(AliensPlugin)
        .add_plugins(ExplosionsPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .run();
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    IntroScreen,
    Highscores,
    Game,
}

mod fighter;
mod movement;
mod scene;

use bevy::prelude::*;

use fighter::FighterPlugin;
use movement::MovementPlugin;
use scene::SceneLoaderPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SceneLoaderPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(FighterPlugin)
        .run();
}

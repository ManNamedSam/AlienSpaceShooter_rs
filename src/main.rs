mod aliens;
mod fighter;
mod movement;
mod scene;

use bevy::prelude::*;

use aliens::AliensPlugin;
use fighter::FighterPlugin;
use movement::MovementPlugin;
use scene::SceneLoaderPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SceneLoaderPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(FighterPlugin)
        .add_plugins(AliensPlugin)
        .run();
}

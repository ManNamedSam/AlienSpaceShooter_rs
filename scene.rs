use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub player: Handle<Image>,
    pub player_bullet: Handle<Image>,
}

pub struct SceneLoaderPlugin;

impl Plugin for SceneLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_assets: ResMut<SceneAssets>,
) {
    commands.spawn(Camera2dBundle::default());
    *scene_assets = SceneAssets {
        player: asset_server.load("craft.png"),
        player_bullet: asset_server.load("playerBullet.png"),
    };
}

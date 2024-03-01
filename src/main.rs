mod aliens;
mod collisions;
mod fighter;
mod intro_screen;
mod movement;
mod scene;

use bevy::prelude::*;

use aliens::AliensPlugin;
use collisions::CollisionDetectionPlugin;
use fighter::FighterPlugin;
use intro_screen::IntroScreenPlugin;
use movement::MovementPlugin;
use scene::SceneLoaderPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_plugins(IntroScreenPlugin)
        .add_plugins(SceneLoaderPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(FighterPlugin)
        .add_plugins(AliensPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .run();
}

// fn intro_screen_test(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     asset_server: Res<AssetServer>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let texture_handle = asset_server.load("titleText.png");
//     let material_handle = materials.add(StandardMaterial {
//         base_color_texture: Some(texture_handle.clone()),
//         ..default()
//     });
//     let mut uvs = Vec::new();
//     uvs.push([0.0, 0.5]);
//     uvs.push([0.0, 0.0]);
//     uvs.push([0.5, 0.0]);
//     uvs.push([0.5, 0.5]);

//     let mut mesh = Mesh::from(Rectangle::new(100.0, 100.0));
//     mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

//     commands.spawn(PbrBundle {
//         mesh: meshes.add(mesh),
//         material: material_handle.clone(),
//         transform: Transform::from_xyz(0.0, 0.0, 1.0),
//         ..default()
//     });

//     // light
//     commands.spawn(LightBundle {
//         transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
//         ..Default::default()
//     });
//     // camera
//     commands.spawn(Camera3dBundle {
//         transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0))
//             .looking_at(Vec3::default(), Vec3::unit_y()),
//         ..Default::default()
//     });
// }

// fn intro_screen_test(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
// ) {
//     // Camera

//     let text_style = TextStyle {
//         font_size: 20.,
//         ..default()
//     };

//     let texture_handle = asset_server.load("titleText.png");
//     let texture_atlas = TextureAtlasLayout::from_grid(Vec2::new(24.0, 24.0), 1, 3, None, None);
//     let texture_atlas_handle = texture_atlases.add(texture_atlas);

//     // root node
//     commands
//         .spawn(NodeBundle {
//             style: Style {
//                 width: Val::Percent(100.0),
//                 height: Val::Percent(100.0),
//                 flex_direction: FlexDirection::Column,
//                 justify_content: JustifyContent::Center,
//                 align_items: AlignItems::Center,
//                 row_gap: Val::Px(text_style.font_size * 2.),
//                 ..default()
//             },
//             ..default()
//         })
//         .with_children(|parent| {
//             parent.spawn(AtlasImageBundle {
//                 style: Style {
//                     width: Val::Px(700.),
//                     height: Val::Px(256.),
//                     ..default()
//                 },
//                 texture_atlas: texture_atlas_handle.into(),
//                 image: UiImage::new(texture_handle),
//                 ..default()
//             });
//             parent.spawn(TextBundle::from_sections([
//                 TextSection::new("press ".to_string(), text_style.clone()),
//                 TextSection::new(
//                     "space".to_string(),
//                     TextStyle {
//                         color: Color::YELLOW,
//                         ..text_style.clone()
//                     },
//                 ),
//                 TextSection::new(" to advance frames".to_string(), text_style),
//             ]));
//         });
// }
// fn increment_atlas_index(
//     mut atlas_images: Query<&mut TextureAtlas>,
//     keyboard: Res<ButtonInput<KeyCode>>,
// ) {
//     if keyboard.just_pressed(KeyCode::Space) {
//         for mut atlas_image in &mut atlas_images {
//             atlas_image.index = (atlas_image.index + 1) % 3;
//         }
//     }
// }

// fn intro_screen_test(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let image = asset_server.load("titleText.png");

//     let slicer = TextureSlicer {
//         border: BorderRect::square(22.0),
//         center_scale_mode: SliceScaleMode::Stretch,
//         sides_scale_mode: SliceScaleMode::Stretch,
//         max_corner_scale: 1.0,
//     };
//     // ui camera
//     commands
//         .spawn(NodeBundle {
//             style: Style {
//                 width: Val::Percent(100.0),
//                 height: Val::Percent(100.0),
//                 align_items: AlignItems::Center,
//                 justify_content: JustifyContent::Center,
//                 ..default()
//             },
//             ..default()
//         })
//         .with_children(|parent| {
//             for [w, h] in [[150.0, 150.0], [300.0, 150.0], [150.0, 300.0]] {
//                 parent
//                     .spawn((
//                         ButtonBundle {
//                             style: Style {
//                                 width: Val::Px(w),
//                                 height: Val::Px(h),
//                                 // horizontally center child text
//                                 justify_content: JustifyContent::Center,
//                                 // vertically center child text
//                                 align_items: AlignItems::Center,
//                                 margin: UiRect::all(Val::Px(20.0)),
//                                 ..default()
//                             },
//                             image: image.clone().into(),
//                             ..default()
//                         },
//                         ImageScaleMode::Sliced(slicer.clone()),
//                     ))
//                     .with_children(|parent| {
//                         parent.spawn(TextBundle::from_section(
//                             "Button",
//                             TextStyle {
//                                 font_size: 40.0,
//                                 color: Color::rgb(0.9, 0.9, 0.9),
//                                 ..default()
//                             },
//                         ));
//                     });
//             }
//         });
// }

// #[derive(Component, Debug)]
// pub struct Title;

// fn intro_screen_test(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let text_style = TextStyle {
//         // font: asset_server.load("fonts/FiraMono-Medium.ttf"),
//         font_size: 20.0,
//         ..default()
//     };

//     let image = asset_server.load("titleText.png");

//     commands
//         .spawn(NodeBundle {
//             style: Style {
//                 width: Val::Percent(100.),
//                 height: Val::Percent(100.),
//                 align_items: AlignItems::Center,
//                 justify_content: JustifyContent::Center,
//                 ..Default::default()
//             },
//             // background_color: ,
//             // background_color: ANTIQUE_WHITE.into(),
//             ..Default::default()
//         })
//         .with_children(|parent| {
//             for overflow in [
//                 Overflow::visible(),
//                 Overflow::clip_x(),
//                 Overflow::clip_y(),
//                 Overflow::clip(),
//             ] {
//                 parent
//                     .spawn(NodeBundle {
//                         style: Style {
//                             flex_direction: FlexDirection::Column,
//                             align_items: AlignItems::Center,
//                             margin: UiRect::horizontal(Val::Px(25.)),
//                             ..Default::default()
//                         },
//                         ..Default::default()
//                     })
//                     .with_children(|parent| {
//                         let label = format!("{overflow:#?}");
//                         parent
//                             .spawn(NodeBundle {
//                                 style: Style {
//                                     padding: UiRect::all(Val::Px(10.)),
//                                     margin: UiRect::bottom(Val::Px(25.)),
//                                     ..Default::default()
//                                 },
//                                 background_color: Color::DARK_GRAY.into(),
//                                 // background_color: DARK_GRAY.into(),
//                                 ..Default::default()
//                             })
//                             .with_children(|parent| {
//                                 parent.spawn(TextBundle {
//                                     text: Text::from_section(label, text_style.clone()),
//                                     ..Default::default()
//                                 });
//                             });
//                         parent
//                             .spawn((
//                                 NodeBundle {
//                                     style: Style {
//                                         width: Val::Px(100.),
//                                         height: Val::Px(100.),
//                                         padding: UiRect {
//                                             left: Val::Px(25.),
//                                             top: Val::Px(25.),
//                                             ..Default::default()
//                                         },
//                                         overflow,
//                                         ..Default::default()
//                                     },
//                                     background_color: Color::GRAY.into(),
//                                     // background_color: GRAY.into(),
//                                     ..Default::default()
//                                 },
//                                 Title,
//                             ))
//                             .with_children(|parent| {
//                                 parent.spawn((
//                                     ImageBundle {
//                                         image: UiImage::new(image.clone()),
//                                         style: Style {
//                                             min_width: Val::Px(100.),
//                                             min_height: Val::Px(100.),
//                                             ..Default::default()
//                                         },
//                                         background_color: Color::WHITE.into(),

//                                         ..Default::default()
//                                     },
//                                     Interaction::default(),
//                                     Outline {
//                                         width: Val::Px(2.),
//                                         offset: Val::Px(2.),
//                                         color: Color::NONE,
//                                     },
//                                 ));
//                             });
//                     });
//             }
//         });
// }

// fn change_height(time: Res<Time>, mut query: Query<&mut Style, With<Title>>) {
//     //     for (mut style) in query.iter_mut() {
//     //         style.height = Val::Px(100.0 * time.delta_seconds());
//     //     }
// }

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    IntroScreen,
    Highscores,
    Game,
}

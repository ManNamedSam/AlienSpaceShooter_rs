use bevy::prelude::*;

use crate::AppState;

pub struct IntroScreenPlugin;

impl Plugin for IntroScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::IntroScreen), setup)
            .add_systems(Update, start_game.run_if(in_state(AppState::IntroScreen)))
            .add_systems(OnExit(AppState::IntroScreen), despawn);
    }
}

#[derive(Component, Debug)]
pub struct UiComponent;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/Orbitron-VariableFont_wght.ttf"),
        font_size: 50.0,
        ..default()
    };
    let image = asset_server.load("titleText.png");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                ..Default::default()
            },
            UiComponent,
        ))
        .with_children(|parent| {
            parent.spawn(
                (ImageBundle {
                    image: UiImage::new(image.clone()),

                    style: Style {
                        width: Val::Px(600.),
                        height: Val::Px(400.),
                        padding: UiRect::all(Val::Px(100.)),
                        ..default()
                    },
                    ..default()
                }),
            );
            parent.spawn(TextBundle {
                text: Text::from_section(
                    String::from("PRESS FIRE (F) TO PLAY!"),
                    text_style.clone(),
                ),
                ..default()
            });
        });
}

fn start_game(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::KeyF) {
        next_state.set(AppState::Game);
        println!("changed state to Game.");
    }
}

fn despawn(mut commands: Commands, query: Query<Entity, With<UiComponent>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

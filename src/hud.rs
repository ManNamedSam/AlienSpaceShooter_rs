use bevy::prelude::*;

use crate::AppState;

#[derive(Resource, Debug, Default)]
pub struct CurrentScore {
    pub value: u32,
}

impl CurrentScore {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn reset(&mut self) {
        self.value = 0;
    }
}

#[derive(Resource, Debug, Default)]
pub struct Highscore {
    pub value: u32,
}

impl Highscore {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn update(&mut self, new_highscore: u32) {
        self.value = new_highscore;
    }
}

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentScore>()
            .init_resource::<Highscore>()
            .add_systems(OnEnter(AppState::Game), setup_hud)
            .add_systems(
                Update,
                (update_current_score, update_highscore_score).run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(AppState::Game), despawn_hud);
    }
}

#[derive(Component, Debug)]
pub struct ScoreText;

#[derive(Component, Debug)]
pub struct HighscoreText;

#[derive(Component, Debug)]
pub struct Hud;

fn setup_hud(
    mut commands: Commands,
    current_score: Res<CurrentScore>,
    highscore: Res<Highscore>,
    asset_server: Res<AssetServer>,
) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/Orbitron-VariableFont_wght.ttf"),
        font_size: 32.0,
        ..default()
    };
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(15.0),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
            Hud,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new("Score: ", text_style.clone()),
                            TextSection::new(
                                format!("{}", current_score.value),
                                text_style.clone(),
                            ),
                        ],
                        ..default()
                    },
                    ..default()
                },
                ScoreText,
            ));
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new("High Score: ", text_style.clone()),
                            TextSection::new(format!("{}", highscore.value), text_style),
                        ],
                        ..default()
                    },
                    ..default()
                },
                HighscoreText,
            ));
        });
}

fn update_current_score(
    mut text_query: Query<&mut Text, With<ScoreText>>,
    score: Res<CurrentScore>,
) {
    if score.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[1].value = format!("{}", score.value.to_string());
        }
    }
}

fn update_highscore_score(
    mut text_query: Query<&mut Text, With<HighscoreText>>,
    current_score: Res<CurrentScore>,
    mut highscore: ResMut<Highscore>,
) {
    if current_score.value > highscore.value {
        highscore.value = current_score.value;
    }
    for mut text in text_query.iter_mut() {
        text.sections[1].value = format!("{}", highscore.value.to_string());
        if highscore.is_changed() {
            text.sections[1].style.color = Color::GREEN;
        }
    }
}

fn despawn_hud(mut query: Query<Entity, With<Hud>>, mut commands: Commands) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}

use bevy::{prelude::*, transform::commands};

use crate::{
    movement::{Position, Velocity},
    AppState,
};

pub struct ExplosionsPlugin;

impl Plugin for ExplosionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_explosions.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), despawn_explosions);
    }
}

#[derive(Component, Debug)]
pub struct Explosion {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub r: f32,
    pub b: f32,
    pub g: f32,
    pub a: f32,
}

impl Explosion {
    pub fn new(x: f32, y: f32) -> Self {
        let x = x + rand::random::<f32>() * 32. - rand::random::<f32>() * 32.;
        let y = y + rand::random::<f32>() * 32. as f32 - rand::random::<f32>() * 32.;
        let dx = (rand::random::<f32>()) - (rand::random::<f32>());
        let dy = (rand::random::<f32>()) - (rand::random::<f32>());
        let mut r: f32 = 0.;
        let mut g: f32 = 0.;
        let mut b: f32 = 0.;

        let seed = rand::random::<u8>() % 4;

        match seed {
            0 => {
                r = 1.0;
            }
            1 => {
                r = 1.0;
                g = 1.0;
            }
            2 => {
                r = 1.0;
                g = 0.5;
            }
            _ => {
                r = 1.0;
                g = 1.0;
                b = 1.0;
            }
        }

        Self {
            x,
            y,
            dx,
            dy,
            r,
            g,
            b,
            a: rand::random::<f32>(),
        }
    }
}

fn handle_explosions(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<
        (
            &Velocity,
            &mut Position,
            &mut Transform,
            &mut Sprite,
            Entity,
        ),
        With<Explosion>,
    >,
) {
    for (velocity, mut position, mut tranform, mut sprite, entity) in query.iter_mut() {
        position.value += velocity.value;
        tranform.translation += velocity.value;
        let new_a = sprite.color.a() - (time.delta_seconds());
        if new_a < 0.0 {
            commands.entity(entity).despawn_recursive();
        } else {
            sprite.color.set_a(new_a);
        }
    }
}

fn despawn_explosions(mut commands: Commands, mut query: Query<Entity, With<Explosion>>) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}

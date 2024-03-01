use bevy::prelude::*;

use crate::AppState;

#[derive(Component, Debug)]
pub struct Position {
    pub value: Vec3,
}

impl Position {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position.run_if(in_state(AppState::Game)));
    }
}

fn update_position(
    time: Res<Time>,
    window: Query<&Window>,
    mut query: Query<(
        &Velocity,
        &mut Position,
        &mut Transform,
        Entity,
        &Handle<Image>,
    )>,
    mut commands: Commands,
    assets: Res<Assets<Image>>,
) {
    let window = window.single();
    for (velocity, mut position, mut transform, entity, image_handle) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
        position.value = transform.translation;
        let image_size = assets.get(image_handle).unwrap().size_f32();

        if position.value.x > (window.width() / 2.0) + (image_size.x / 2.0)
            || position.value.x < (-window.width() / 2.0) - (image_size.x / 2.0)
            || position.value.y > (window.height() / 2.0) + (image_size.y / 2.0)
            || position.value.y < (-window.height() / 2.0) - (image_size.y / 2.0)
        {
            commands.entity(entity).despawn_recursive();
        }
    }
}

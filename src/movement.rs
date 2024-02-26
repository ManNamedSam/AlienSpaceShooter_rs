use bevy::prelude::*;

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
        app.add_systems(Update, update_position);
    }
}

fn update_position(
    time: Res<Time>,
    window: Query<&Window>,
    mut query: Query<(&Velocity, &mut Position, &mut Transform, Entity)>,
    mut commands: Commands,
) {
    let window = window.single();
    for (velocity, mut position, mut transform, entity) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
        position.value = transform.translation;

        if position.value.x > window.width() / 2.0
            || position.value.x < -window.width() / 2.0
            || position.value.y > window.height() / 2.0
            || position.value.y < -window.height() / 2.0
        {
            commands.entity(entity).despawn();
        }
    }
}

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_scene);
}

#[derive(Event, Debug)]
pub struct SpawnScene;

fn spawn_scene(_trigger: Trigger<SpawnScene>) {}

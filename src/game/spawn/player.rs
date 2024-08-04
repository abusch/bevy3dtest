//! Spawn the player.

use bevy::prelude::*;

use crate::{game::assets::PlayerAssets, screen::Screen};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<Player>();
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn spawn_player(_trigger: Trigger<SpawnPlayer>, assets: Res<PlayerAssets>, mut commands: Commands) {
    commands.spawn((
        Name::new("Player"),
        Player,
        SceneBundle {
            scene: assets.mesh.clone_weak(),
            ..default()
        },
        StateScoped(Screen::Playing),
    ));
}

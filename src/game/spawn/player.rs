//! Spawn the player.

use std::time::Duration;

use bevy::{animation::animate_targets, ecs::system::SystemState, prelude::*};
use bevy_asset_loader::loading_state::{
    config::{ConfigureLoadingState, LoadingStateConfig},
    LoadingStateAppExt,
};

use crate::{game::assets::PlayerAssets, screen::Screen, AppSet};

pub(super) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(Screen::Loading).init_resource::<PlayerAnimations>(),
    )
    .observe(spawn_player)
    .add_systems(
        Update,
        start_animations
            .in_set(AppSet::Update)
            .run_if(in_state(Screen::Playing))
            .before(animate_targets),
    )
    .register_type::<Player>();
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Resource)]
pub struct PlayerAnimations {
    pub graph: Handle<AnimationGraph>,
    pub animation_id: AnimationNodeIndex,
}

impl FromWorld for PlayerAnimations {
    fn from_world(world: &mut World) -> Self {
        info!("Creating PlayerAnimations resource");
        let mut system_state =
            SystemState::<(ResMut<Assets<AnimationGraph>>, Res<PlayerAssets>)>::new(world);
        let (mut graphs, player_assets) = system_state.get_mut(world);
        let (graph, index) = AnimationGraph::from_clip(player_assets.animations.clone());
        let graph = graphs.add(graph);

        Self {
            graph,
            animation_id: index,
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    player_assets: Res<PlayerAssets>,
    mut commands: Commands,
) {
    info!("Spawning player");

    let _player = commands
        .spawn((
            Name::new("Player"),
            Player,
            SceneBundle {
                scene: player_assets.mesh.clone(),
                ..default()
            },
            StateScoped(Screen::Playing),
        ))
        .id();
}

fn start_animations(
    mut commands: Commands,
    player_animations: Res<PlayerAnimations>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut player) in &mut players {
        info!("Found AnimationPlayer for entity {entity}");
        let mut transitions = AnimationTransitions::default();
        transitions
            .play(&mut player, player_animations.animation_id, Duration::ZERO)
            .repeat();
        commands
            .entity(entity)
            .insert(player_animations.graph.clone())
            .insert(transitions);
    }
}

//! Spawn the player.

use std::collections::HashMap;

use avian3d::prelude::{Collider, DebugRender, LockedAxes, RigidBody};
use bevy::{ecs::system::SystemState, prelude::*};
use bevy_asset_loader::loading_state::{
    config::{ConfigureLoadingState, LoadingStateConfig},
    LoadingStateAppExt,
};
use bevy_tnua::{
    builtins::{TnuaBuiltinCrouch, TnuaBuiltinJumpState},
    prelude::{TnuaBuiltinJump, TnuaBuiltinWalk, TnuaController, TnuaControllerBundle},
    TnuaAction, TnuaAnimatingState, TnuaAnimatingStateDirective, TnuaUserControlsSystemSet,
};
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use smooth_bevy_cameras::LookTransform;

use crate::{game::assets::CharactersAssets, screen::Screen, AppSet};

pub(super) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(Screen::Loading).init_resource::<PlayerAssets>(),
    )
    .observe(spawn_player)
    .add_systems(
        Update,
        (
            apply_controls.in_set(TnuaUserControlsSystemSet),
            prepare_animations.in_set(AppSet::Update),
            handle_animations.in_set(AppSet::Update),
            move_camera.in_set(AppSet::Update),
        )
            .run_if(in_state(Screen::Playing)),
    )
    .register_type::<PlayerParams>()
    .register_type::<Player>();
}

#[derive(Component)]
pub struct CameraTracked;

#[derive(Component, Reflect)]
pub struct PlayerParams {
    float_height: f32,
    cling_distance: f32,
    crouch_float_offset: f32,
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Resource)]
pub struct PlayerAssets {
    pub scene: Handle<Scene>,
    pub graph: Handle<AnimationGraph>,
    pub animations: HashMap<Box<str>, AnimationNodeIndex>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        info!("Creating PlayerAnimations resource");
        let mut system_state = SystemState::<(
            Res<Assets<Gltf>>,
            ResMut<Assets<AnimationGraph>>,
            Res<CharactersAssets>,
        )>::new(world);
        let (gltfs, mut graphs, characters_assets) = system_state.get_mut(world);

        let mut animation_ids: HashMap<Box<str>, AnimationNodeIndex> = HashMap::new();
        let mut graph = AnimationGraph::new();
        let gltf = gltfs
            .get(&characters_assets.male_a)
            .expect("Missing GLTF file!");
        for (name, clip) in &gltf.named_animations {
            let idx = graph.add_clip(clip.clone(), 1.0, graph.root);
            animation_ids.insert(name.clone(), idx);
        }

        let graph = graphs.add(graph);

        let scene = gltf.scenes[0].clone();

        Self {
            scene,
            graph,
            animations: animation_ids,
        }
    }
}

pub enum PlayerAnimationState {
    Standing,
    Running(f32),
    Jumping,
    Falling,
    Crouch,
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

    commands
        .spawn((
            Name::new("Player"),
            Player,
            CameraTracked,
            SpatialBundle::from_transform(Transform::from_xyz(0.0, 5.5, 0.0)),
            StateScoped(Screen::Playing),
            TnuaAnimatingState::<PlayerAnimationState>::default(),
            RigidBody::Dynamic,
            TnuaControllerBundle::default(),
            TnuaAvian3dSensorShape(Collider::cylinder(0.24, 0.0)),
            LockedAxes::ROTATION_LOCKED.unlock_rotation_y(),
            PlayerParams {
                float_height: 0.5,
                cling_distance: 0.1,
                crouch_float_offset: 0.0,
            },
            Collider::capsule(0.25, 0.1),
            DebugRender::all(),
        ))
        .with_children(|children| {
            // Spawn the actual mesh as a child to be able to align it properly with the collider,
            // which is always spawned around the origin.
            children.spawn(SceneBundle {
                scene: player_assets.scene.clone(),
                transform: Transform::from_xyz(0.0, -0.5, 0.0),
                ..default()
            });
        });
}

fn handle_animations(
    mut player_query: Query<(
        &TnuaController,
        &mut TnuaAnimatingState<PlayerAnimationState>,
    )>,
    mut animation_player_query: Query<&mut AnimationPlayer>,
    player_assets: Res<PlayerAssets>,
) {
    let Ok((controller, mut animation_state)) = player_query.get_single_mut() else {
        return;
    };
    let Ok(mut animation_player) = animation_player_query.get_single_mut() else {
        return;
    };

    let current_status_for_animating = match controller.action_name() {
        Some(TnuaBuiltinJump::NAME) => {
            // In case of jump, we want to cast it so that we can get the concrete jump state.
            let (_, jump_state) = controller
                .concrete_action::<TnuaBuiltinJump>()
                .expect("action name mismatch");
            // Depending on the state of the jump, we need to decide if we want to play the jump
            // animation or the fall animation.
            match jump_state {
                TnuaBuiltinJumpState::NoJump => return,
                TnuaBuiltinJumpState::StartingJump { .. } => PlayerAnimationState::Jumping,
                TnuaBuiltinJumpState::SlowDownTooFastSlopeJump { .. } => {
                    PlayerAnimationState::Jumping
                }
                TnuaBuiltinJumpState::MaintainingJump => PlayerAnimationState::Jumping,
                TnuaBuiltinJumpState::StoppedMaintainingJump => PlayerAnimationState::Jumping,
                TnuaBuiltinJumpState::FallSection => PlayerAnimationState::Falling,
            }
        }
        Some(TnuaBuiltinCrouch::NAME) => PlayerAnimationState::Crouch,
        Some(_) => panic!("Unknown command!"),
        None => {
            let Some((_, basis_state)) = controller.concrete_basis::<TnuaBuiltinWalk>() else {
                return;
            };
            if basis_state.standing_on_entity().is_none() {
                // Player isn't standing on an entity: it needs to fall
                PlayerAnimationState::Falling
            } else {
                let speed = basis_state.running_velocity.length();
                if 0.01 < speed {
                    PlayerAnimationState::Running(0.1 * speed)
                } else {
                    PlayerAnimationState::Standing
                }
            }
        }
    };

    let animation_directive = animation_state.update_by_discriminant(current_status_for_animating);
    match animation_directive {
        TnuaAnimatingStateDirective::Maintain { state } => {
            // We're staying in the same animation state
            // If we're running, adjust the speed though...
            if let PlayerAnimationState::Running(speed) = state {
                if let Some(animation) =
                    animation_player.animation_mut(player_assets.animations["walk"])
                {
                    animation.set_speed(*speed);
                }
            }
        }
        TnuaAnimatingStateDirective::Alter { state, .. } => {
            animation_player.stop_all();
            match state {
                PlayerAnimationState::Standing => {
                    animation_player.start(player_assets.animations["static"]);
                    animation_player
                        .start(player_assets.animations["idle"])
                        .set_speed(1.0)
                        .repeat();
                }
                PlayerAnimationState::Running(speed) => {
                    animation_player
                        .start(player_assets.animations["walk"])
                        .set_speed(*speed)
                        .repeat();
                }
                PlayerAnimationState::Falling => {
                    animation_player
                        .start(player_assets.animations["fall"])
                        .set_speed(1.0)
                        .repeat();
                }
                PlayerAnimationState::Jumping => {
                    animation_player
                        .start(player_assets.animations["jump"])
                        .set_speed(1.0);
                }
                PlayerAnimationState::Crouch => {
                    animation_player
                        .start(player_assets.animations["crouch"])
                        .set_speed(1.0);
                }
            }
        }
    }
}

fn prepare_animations(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    players: Query<Entity, Added<AnimationPlayer>>,
) {
    for entity in &players {
        info!("Found AnimationPlayer for entity {entity}");
        commands.entity(entity).insert(player_assets.graph.clone());
    }
}

fn apply_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut TnuaController, &PlayerParams)>,
) {
    let Ok((mut controller, player_params)) = query.get_single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::ArrowUp) {
        direction -= Vec3::Z;
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        direction += Vec3::Z;
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        direction -= Vec3::X;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        direction += Vec3::X;
    }

    // Feed the basis
    controller.basis(TnuaBuiltinWalk {
        desired_velocity: direction.normalize_or_zero() * 8.0,
        desired_forward: -direction.normalize_or_zero(),
        float_height: player_params.float_height,
        cling_distance: player_params.cling_distance,
        ..default()
    });

    // Dash
    if keyboard.pressed(KeyCode::ShiftLeft) {
        controller.action(TnuaBuiltinCrouch {
            float_offset: player_params.crouch_float_offset,
            ..default()
        });
    }

    // Feed the jump action every frame as long as the player holds the jump button. If the player
    // stops holding the jump button, simply stop feeding the action.
    if keyboard.pressed(KeyCode::Space) {
        controller.action(TnuaBuiltinJump {
            // The height is the only mandatory field of the jump button.
            height: 2.0,
            // `TnuaBuiltinJump` also has customization fields with sensible defaults.
            ..Default::default()
        });
    }
}

fn move_camera(
    mut camera: Query<&mut LookTransform>,
    tracked: Query<&Transform, With<CameraTracked>>,
) {
    let mut camera = camera.single_mut();
    let tracked = tracked.single();

    camera.target = tracked.translation;
    // camera.look_to(tracked.back(), Vec3::Y);
}

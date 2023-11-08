use bevy::ecs::query::Has;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;
use std::time::Duration;

use crate::{
    animation::{Animated, AnimationCharacterMap, AnimationInit, AnimationTransitionEvent},
    assets::{CharacterCache, PlayerAnimationCache},
    camera::MainCamera,
    core::{GameState, IndexPointer},
    input::{InputListenerBundle, PlayerAction},
    physics::{Direction, Grounded, Momentum, MovementBundle, Speed},
};

#[derive(Resource, Default)]
pub struct PlayerData {
    pub player_position: Vec3,
    pub held_object_position: Vec3,
    pub held_object_index: IndexPointer,
    pub distance_from_floor: f32,
    pub floor_normal: Vec3,
    pub speed: f32,
    pub defacto_speed: f32,
    pub kicked_wall: Option<Entity>,
    pub jump_stage: u8,
}

#[derive(Event)]
pub struct PlayerStateTransitionEvent {
    pub current_state: Player,
    pub new_state: Player,
}

#[derive(Component, Default, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Diving,
    BellySliding,
    #[default]
    Idle,
    Walking,
    Running,
    LongJumping,
    Rising,
    Freefall,
    Walljumping,
    Carrying,
    ButtSliding,
    Sliding,
}

fn spawn_player(
    mut commands: Commands,
    characters: Res<CharacterCache>,
    particles: Res<crate::particles::ParticleCache>,
) {
    commands.spawn((
        SceneBundle {
            scene: characters.uli.clone_weak(),
            ..default()
        },
        Player::Idle,
        Animated,
        MovementBundle {
            collider: Collider::capsule_y(0.5, 0.5),
            ..default()
        },
        InputListenerBundle::input_map(),
    ));

    use bevy_hanabi::prelude::*;

    commands.spawn(ParticleEffectBundle {
        effect: ParticleEffect::new(particles.dust.clone_weak()),
        transform: Transform::from_translation(Vec3::Y),
        ..default()
    });
}

fn update_player_data(
    mut player_data: ResMut<PlayerData>,
    player_query: Query<&Transform, With<Player>>,
) {
    for transform in &player_query {
        player_data.player_position = transform.translation;
    }
}

fn handle_grounded(
    mut commands: Commands,
    mut player_data: ResMut<PlayerData>,
    player_query: Query<(Entity, &Transform, Has<Grounded>), With<Player>>,
    rapier_context: Res<RapierContext>,
) {
    for (entity, transform, has_grounded) in &player_query {
        let ray_pos = transform.translation;
        let ray_dir = Vec3::Y * -1.0;
        let max_distance = 1.1;
        let solid = true;
        let filter = QueryFilter::exclude_dynamic().exclude_sensors();

        let ray_result =
            rapier_context.cast_ray_and_get_normal(ray_pos, ray_dir, max_distance, solid, filter);

        if let Some((_, intersection)) = ray_result {
            player_data.floor_normal = intersection.normal;
            player_data.distance_from_floor = intersection.toi;
            player_data.kicked_wall = None;
            if !has_grounded {
                commands.entity(entity).insert(Grounded);
            }
        }
    }
}

fn set_player_direction(
    mut player_query: Query<
        (
            &mut Direction,
            Option<&Grounded>,
            &ActionState<PlayerAction>,
        ),
        With<Player>,
    >,
    camera_query: Query<&Transform, With<MainCamera>>,
) {
    let camera_transform = camera_query.single();
    for (mut direction, grounded, action) in &mut player_query {
        if grounded.is_some() {
            direction.set(get_direction_in_camera_space(camera_transform, action));
        } else {
            if direction.is_any() {
                direction.set(Vec3::ZERO);
            }
        }
    }
}

fn get_direction_in_camera_space(
    camera_transform: &Transform,
    action: &ActionState<PlayerAction>,
) -> Vec3 {
    let mut x = 0.0;
    let mut z = 0.0;

    let mut forward = camera_transform.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let mut right = camera_transform.right();
    right.y = 0.0;
    right = right.normalize();

    if action.pressed(PlayerAction::Move) {
        let axis_pair = action.clamped_axis_pair(PlayerAction::Move).unwrap();
        x = axis_pair.x();
        z = axis_pair.y();
    }

    let right_vec: Vec3 = x * right;
    let forward_vec: Vec3 = z * forward;

    (right_vec + forward_vec)
}

fn play_idle_animation(
    mut commands: Commands,
    animation_map: Res<AnimationCharacterMap>,
    player_query: Query<Entity, (With<Player>, Without<AnimationInit>)>,
    mut animation_player_query: Query<&mut AnimationPlayer>,
    assets: Res<AssetServer>,
) {
    for entity in &player_query {
        if let Some(animation_entity) = animation_map.get(entity) {
            if let Ok(mut animation_player) = animation_player_query.get_mut(animation_entity) {
                animation_player
                    .play(assets.load("models/uli.glb#Animation0"))
                    .repeat();

                commands.entity(entity).insert(AnimationInit);
            }
        }
    }
}

fn handle_state_transition_events(
    mut state_events: EventWriter<PlayerStateTransitionEvent>,
    player_query: Query<&Player>,
    mut previous_state: Local<Player>,
) {
    for current_state in &player_query {
        if *current_state != *previous_state {
            state_events.send(PlayerStateTransitionEvent {
                current_state: *previous_state,
                new_state: *current_state,
            });
        }
        *previous_state = *current_state;
    }
}

fn run_to_idle(
    mut animation_transitions: EventWriter<AnimationTransitionEvent>,
    animation_cache: Res<PlayerAnimationCache>,
    player_query: Query<(Entity, &Direction, Has<Grounded>), With<Player>>,
) {
    for (entity, direction, is_grounded) in &player_query {
        if is_grounded {
            if direction.is_any() {
                animation_transitions.send(AnimationTransitionEvent {
                    entity,
                    clip: animation_cache.run.clone_weak(),
                    transition: Duration::from_secs_f32(0.2),
                });
            } else {
                animation_transitions.send(AnimationTransitionEvent {
                    entity,
                    clip: animation_cache.idle.clone_weak(),
                    transition: Duration::from_secs_f32(0.3),
                });
            }
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerData::default())
            .add_systems(OnEnter(GameState::Gameplay), spawn_player)
            .add_systems(
                Update,
                (
                    handle_grounded,
                    play_idle_animation,
                    update_player_data,
                    set_player_direction,
                    run_to_idle,
                )
                    .run_if(in_state(GameState::Gameplay)),
            );
    }
}

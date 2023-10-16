use bevy::ecs::query::Has;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    animation::{Animated, AnimationCharacterMap, AnimationInit},
    assets::CharacterCache,
    core::{GameState, IndexPointer},
    input::{InputListenerBundle, PlayerAction},
    physics::{Grounded, MovementBundle},
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

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands, characters: Res<CharacterCache>) {
    commands.spawn((
        SceneBundle {
            scene: characters.uli.clone_weak(),
            ..default()
        },
        Player,
        Animated,
        MovementBundle {
            collider: Collider::capsule_y(0.5, 0.5),
            ..default()
        },
        InputListenerBundle::input_map(),
    ));
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
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerData::default())
            .add_systems(OnEnter(GameState::Gameplay), spawn_player)
            .add_systems(
                Update,
                (handle_grounded, play_idle_animation, update_player_data)
                    .run_if(in_state(GameState::Gameplay)),
            );
    }
}

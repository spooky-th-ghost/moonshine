use std::time::Duration;

use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::core::GameState;

#[derive(Component)]
pub struct Animated;

#[derive(Component)]
pub struct AnimationInit;

#[derive(Resource, Default)]
pub struct AnimationCharacterMap(HashMap<Entity, Entity>);

impl AnimationCharacterMap {
    pub fn get(&self, key_entity: Entity) -> Option<Entity> {
        self.0.get(&key_entity).copied()
    }

    pub fn insert(&mut self, key_entity: Entity, value_entity: Entity) {
        self.0.insert(key_entity, value_entity);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Event)]
pub struct AnimationTransitionEvent {
    pub entity: Entity,
    pub clip: Handle<AnimationClip>,
    pub transition: Duration,
}

fn store_animation_relationships(
    mut commands: Commands,
    mut animation_character_map: ResMut<AnimationCharacterMap>,
    child_query: Query<(Entity, &Parent), Added<AnimationPlayer>>,
    grandparent_query: Query<(Entity, &Children), With<Animated>>,
) {
    for (grandchild_entity, grandchild_parent) in &child_query {
        for (grandparent_entity, grandparent_children) in &grandparent_query {
            if grandparent_children
                .into_iter()
                .any(|entity| *entity == grandchild_parent.get())
            {
                animation_character_map.insert(grandparent_entity, grandchild_entity);
                commands.entity(grandparent_entity).remove::<Animated>();
            }
        }
    }
}

fn handle_animation_transition_events(
    mut transition_events: EventReader<AnimationTransitionEvent>,
    mut animation_player_query: Query<&mut AnimationPlayer>,
    animation_character_map: Res<AnimationCharacterMap>,
) {
    for event in transition_events.iter() {
        if let Some(animation_player_entity) = animation_character_map.get(event.entity) {
            if let Ok(mut animation_player) =
                animation_player_query.get_mut(animation_player_entity)
            {
                animation_player.play_with_transition(event.clip.clone_weak(), event.transition);
                animation_player.repeat();
            }
        }
    }
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AnimationCharacterMap::default())
            .add_event::<AnimationTransitionEvent>()
            .add_systems(
                Update,
                (
                    store_animation_relationships,
                    handle_animation_transition_events,
                )
                    .run_if(in_state(GameState::Gameplay)),
            );
    }
}

use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::core::GameState;

#[derive(Component)]
pub struct Animated;

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

pub fn store_animation_relationships(
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

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AnimationCharacterMap::default())
            .add_systems(
                Update,
                store_animation_relationships.run_if(in_state(GameState::Gameplay)),
            );
    }
}

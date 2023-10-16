use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::core::GameState;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Preload).continue_to_state(GameState::Gameplay),
        )
        .add_collection_to_loading_state::<_, CharacterCache>(GameState::Preload)
        .add_collection_to_loading_state::<_, PlayerAnimationCache>(GameState::Preload)
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            GameState::Preload,
            "character_models.assets.ron",
        )
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            GameState::Preload,
            "player_animations.assets.ron",
        );
    }
}

#[derive(Resource, AssetCollection)]
pub struct CharacterCache {
    #[asset(key = "uli")]
    pub uli: Handle<Scene>,
}

#[derive(Resource, AssetCollection)]
pub struct PlayerAnimationCache {
    #[asset(key = "idle")]
    pub idle: Handle<AnimationClip>,
    #[asset(key = "run")]
    pub run: Handle<AnimationClip>,
}

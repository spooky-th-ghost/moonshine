use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

mod animation;
mod assets;
mod camera;
mod core;
mod input;
mod physics;
mod player;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::default(),
        ))
        .add_plugins((
            core::CorePlugin,
            camera::CameraPlugin,
            assets::AssetPlugin,
            player::PlayerPlugin,
            animation::AnimationPlugin,
            input::InputPlugin,
        ))
        .run();
}

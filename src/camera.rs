use crate::core::GameState;
use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 10.0, 5.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(MainCamera);
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gameplay), spawn_camera);
    }
}

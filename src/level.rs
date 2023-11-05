use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::assets::MaterialCache;
use crate::core::GameState;

pub fn spawn_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<MaterialCache>,
) {
    commands.spawn(DirectionalLightBundle::default());
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(10.0, 0.5, 10.0))),
            material: materials.checkerboard.clone_weak(),
            transform: Transform::from_translation(Vec3::NEG_Y * 1.0),
            ..default()
        })
        .insert(Collider::cuboid(5.0, 0.25, 5.0))
        .insert(RigidBody::Fixed);
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gameplay), spawn_level);
    }
}

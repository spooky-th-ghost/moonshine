use crate::core::GameState;
use crate::input::PlayerAction;
use crate::player::PlayerData;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Component, Default)]
pub struct MainCamera {
    offset: Vec3,
    angle: f32,
    easing: f32,
    camera_mode: CameraMode,
    desired_position: Vec3,
}

#[derive(Default)]
pub enum CameraMode {
    #[default]
    Fixed,
    Follow,
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle::default())
        .insert(MainCamera {
            offset: Vec3::new(0.0, 7.0, 10.0),
            angle: 0.0,
            easing: 4.0,
            camera_mode: CameraMode::Fixed,
            desired_position: Vec3::ZERO,
        });
}

fn update_camera_desired_position(
    mut camera_query: Query<&mut MainCamera>,
    player_data: Res<PlayerData>,
) {
    for mut camera in &mut camera_query {
        let mut starting_transform = Transform::from_translation(player_data.player_position);

        starting_transform.rotation = Quat::default();
        starting_transform.rotate_y(camera.angle.to_radians());
        let dir = starting_transform.forward().normalize();
        camera.desired_position =
            starting_transform.translation + (dir * camera.offset.z) + (Vec3::Y * camera.offset.y);
    }
}

fn position_camera(
    time: Res<Time>,
    player_data: Res<PlayerData>,
    mut camera_query: Query<(&mut Transform, &MainCamera)>,
) {
    for (mut transform, camera) in &mut camera_query {
        match camera.camera_mode {
            CameraMode::Fixed => {
                let lerped_position = transform.translation.lerp(
                    camera.desired_position,
                    time.delta_seconds() * camera.easing,
                );
                transform.translation = lerped_position;
                transform.look_at(player_data.player_position, Vec3::Y);
            }
            _ => (),
        }
    }
}

fn rotate_camera(
    mut camera_query: Query<&mut MainCamera>,
    actions_query: Query<&ActionState<PlayerAction>>,
) {
    for mut camera in &mut camera_query {
        for action in &actions_query {
            if action.just_pressed(PlayerAction::CamRotateLeft) {
                camera.angle -= 45.0;
            }
            if action.just_pressed(PlayerAction::CamRotateRight) {
                camera.angle += 45.0;
            }

            if camera.angle > 360.0 {
                camera.angle -= 360.0;
            }

            if camera.angle < -360.0 {
                camera.angle += 360.0;
            }
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gameplay), spawn_camera)
            .add_systems(
                Update,
                (
                    update_camera_desired_position,
                    position_camera,
                    rotate_camera,
                )
                    .run_if(in_state(GameState::Gameplay)),
            );
    }
}

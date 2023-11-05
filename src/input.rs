use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, *};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Default, Reflect)]
pub enum PlayerAction {
    #[default]
    Jump,
    Move,
    Interact,
    CamRotateRight,
    CamRotateLeft,
    CamModeChangePositive,
    CamModeChangeNegative,
}

#[derive(Bundle)]
pub struct InputListenerBundle {
    input_manager: InputManagerBundle<PlayerAction>,
}

impl InputListenerBundle {
    pub fn input_map() -> InputListenerBundle {
        use PlayerAction::*;

        let input_map = input_map::InputMap::new([
            (KeyCode::Space, Jump),
            (KeyCode::L, Interact),
            (KeyCode::Left, CamRotateLeft),
            (KeyCode::Right, CamRotateRight),
            (KeyCode::Up, CamModeChangePositive),
            (KeyCode::Down, CamModeChangeNegative),
        ])
        .insert_multiple([
            (GamepadButtonType::South, Jump),
            (GamepadButtonType::West, Interact),
            (GamepadButtonType::LeftTrigger2, CamRotateLeft),
            (GamepadButtonType::RightTrigger2, CamRotateRight),
        ])
        .insert(DualAxis::left_stick(), Move)
        .insert(VirtualDPad::wasd(), Move)
        .set_gamepad(Gamepad { id: 0 })
        .build();

        InputListenerBundle {
            input_manager: InputManagerBundle {
                input_map,
                ..Default::default()
            },
        }
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
    }
}

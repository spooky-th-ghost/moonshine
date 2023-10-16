use bevy::prelude::*;

#[derive(Component)]
pub struct Character;

#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    #[default]
    Preload,
    Load,
    Gameplay,
    MainMenu,
}

#[derive(Default)]
pub enum IndexPointer {
    #[default]
    Empty,
    FindAt(usize),
    WaitFor(usize),
}

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>();
    }
}

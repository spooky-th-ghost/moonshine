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

#[derive(Default, Debug)]
pub struct Unit(i32);

impl std::ops::Add for Unit {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Unit(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for Unit {
   fn add_assign(&mut self, rhs: Self) {
       self.0 += rhs.0;
   } 
}

impl std::ops::Sub for Unit {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Unit(self.0 - rhs.0)
    }
}

impl std::ops::SubAssign for Unit {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl From<f32> for Unit {
    fn from(value: f32) -> Self {
        Unit((value * 80.0) as i32)
    }
}

impl From<i32> for Unit {
    fn from(value: i32) -> Self {
        Unit(value)
    }
}

impl From<Unit> for f32 {
    fn from(value: Unit) -> Self {
        value.0 as f32 / 80.0
    }
}

#[derive(Default)]
pub struct UVec {
    pub x: Unit,
    pub y: Unit,
    pub z: Unit
}

impl UVec {
    pub fn new(x: Unit, y: Unit, z: Unit) -> Self {
        UVec { x, y, z }
    }
    pub fn int(x: i32, y: i32, z: i32) -> Self {
        UVec { x: Unit(x), y: Unit(y), z: Unit(z) }
    }
}

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>();
    }
}

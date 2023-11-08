use crate::core::{Character, GameState};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Default, Component)]
pub struct Direction(pub Vec3);

impl Direction {
    pub fn get(&self) -> Vec3 {
        self.0
    }

    pub fn set(&mut self, value: Vec3) {
        self.0 = value;
    }

    pub fn is_any(&self) -> bool {
        self.0 != Vec3::ZERO
    }

    pub fn is_active(&self) -> bool {
        self.0.length() >= 0.3
    }
}

#[derive(Default, Component)]
pub struct Drift(pub Vec3);

#[derive(Default, Component)]
pub struct Speed {
    current: f32,
    accel: f32,
    base: f32,
    max: f32,
    base_max: f32,
    accel_timer: Timer,
}

impl Speed {
    pub fn reset(&mut self) {
        self.current = self.base;
        self.max = self.base_max;
        self.accel_timer.reset();
    }

    pub fn accelerate(&mut self, delta: std::time::Duration, seconds: f32) {
        self.accel_timer.tick(delta);
        if self.accel_timer.finished() {
            if self.current < self.max {
                self.current = self.current + (self.max - self.current) * (seconds * self.accel);
            } else {
                self.current = self.max;
            }
        }
    }
}

#[derive(Default, Component)]
pub struct Momentum(f32);

impl Momentum {
    pub fn get(&self) -> f32 {
        self.0
    }

    pub fn set(&mut self, value: f32) {
        self.0 = value;
    }

    pub fn is_any(&self) -> bool {
        self.0 != 0.0
    }

    pub fn reset(&mut self) {
        self.0 = 0.0;
    }
}
#[derive(Component)]
pub struct Grounded;

#[derive(Bundle)]
pub struct MovementBundle {
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub external_impulse: ExternalImpulse,
    pub velocity: Velocity,
    pub friction: Friction,
    pub damping: Damping,
    pub gravity_scale: GravityScale,
    pub direction: Direction,
    pub character: Character,
    pub momentum: Momentum,
    pub locked_axes: LockedAxes,
    pub speed: Speed,
}

impl Default for MovementBundle {
    fn default() -> Self {
        MovementBundle {
            rigidbody: RigidBody::Dynamic,
            collider: Collider::default(),
            external_impulse: ExternalImpulse::default(),
            velocity: Velocity::default(),
            friction: Friction::default(),
            damping: Damping {
                linear_damping: 6.0,
                ..default()
            },
            gravity_scale: GravityScale::default(),
            direction: Direction::default(),
            character: Character,
            momentum: Momentum::default(),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            speed: Speed {
                base: 3.0,
                current: 3.0,
                accel: 2.5,
                max: 7.5,
                base_max: 7.5,
                accel_timer: Timer::from_seconds(0.6, TimerMode::Once),
                ..default()
            },
        }
    }
}

impl MovementBundle {
    pub fn with_rigidbody(mut self, rigidbody: RigidBody) -> Self {
        self.rigidbody = rigidbody;
        self
    }

    pub fn with_collider(mut self, collider: Collider) -> Self {
        self.collider = collider;
        self
    }

    pub fn with_impulse(mut self, external_impulse: ExternalImpulse) -> Self {
        self.external_impulse = external_impulse;
        self
    }

    pub fn with_velocity(mut self, velocity: Velocity) -> Self {
        self.velocity = velocity;
        self
    }

    pub fn with_damping(mut self, damping: Damping) -> Self {
        self.damping = damping;
        self
    }

    pub fn with_friction(mut self, friction: Friction) -> Self {
        self.friction = friction;
        self
    }

    pub fn with_gravity_scale(mut self, gravity_scale: f32) -> Self {
        self.gravity_scale = GravityScale(gravity_scale);
        self
    }
}

fn rotate_to_direction(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Direction, &Speed), (With<Character>, With<Grounded>)>,
    mut rotation_target: Local<Transform>,
) {
    for (mut transform, direction, speed) in &mut query {
        rotation_target.translation = transform.translation;
        let flat_velo_direction = Vec3::new(direction.0.x, 0.0, direction.0.z).normalize_or_zero();
        if flat_velo_direction != Vec3::ZERO {
            let target_position = rotation_target.translation + flat_velo_direction;

            rotation_target.look_at(target_position, Vec3::Y);
            let turn_speed = speed.current * 0.85;

            transform.rotation = transform
                .rotation
                .slerp(rotation_target.rotation, time.delta_seconds() * turn_speed);
        }
    }
}

fn handle_speed(
    time: Res<Time>,
    mut query: Query<(&mut Momentum, &mut Speed, &Direction), With<Grounded>>,
) {
    for (mut momentum, mut speed, direction) in &mut query {
        if direction.is_active() {
            speed.accelerate(time.delta(), time.delta_seconds());
            momentum.set(speed.current);
        } else {
            momentum.reset();
            speed.reset();
        }
    }
}

pub fn apply_momentum(mut query: Query<(&mut Velocity, &Transform, &Momentum)>) {
    for (mut velocity, transform, momentum) in &mut query {
        let mut speed_to_apply = Vec3::ZERO;
        let mut should_change_velocity: bool = false;

        if momentum.is_any() {
            should_change_velocity = true;
            let forward = transform.forward();
            speed_to_apply += forward * momentum.get();
        }

        if should_change_velocity {
            velocity.linvel.x = speed_to_apply.x;
            velocity.linvel.z = speed_to_apply.z;
        }
    }
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (rotate_to_direction, apply_momentum, handle_speed)
                .run_if(in_state(GameState::Gameplay)),
        );
    }
}

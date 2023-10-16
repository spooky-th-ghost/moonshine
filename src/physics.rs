use crate::core::Character;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Default, Component)]
pub struct Direction(pub Vec3);
#[derive(Default, Component)]
pub struct Drift(pub Vec3);
#[derive(Default, Component)]
pub struct Momentum(pub Vec3);
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

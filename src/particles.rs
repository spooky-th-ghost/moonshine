use bevy::prelude::*;
use bevy_hanabi::prelude::*;

pub struct ParticlePlugin;

#[derive(Resource)]
pub struct ParticleCache {
    pub dust: Handle<EffectAsset>,
}

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HanabiPlugin)
            .add_systems(Startup, initialize_particle_cache);
    }
}

fn initialize_particle_cache(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    assets: Res<AssetServer>,
) {
    // Define a color gradient from red to transparent black
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::splat(1.));
    gradient.add_key(1.0, Vec4::splat(0.));

    // Create a new expression module
    let mut module = Module::default();

    // On spawn, randomly initialize the position of the particle
    // to be over the surface of a sphere of radius 2 units.
    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(0.05),
        dimension: ShapeDimension::Surface,
    };

    // Also initialize a radial initial velocity to 6 units/sec
    // away from the (same) sphere center.
    let init_vel = SetVelocitySphereModifier {
        center: module.lit(Vec3::ZERO),
        speed: module.lit(0.5),
    };

    // Initialize the total lifetime of the particle, that is
    // the time for which it's simulated and rendered. This modifier
    // is almost always required, otherwise the particles won't show.
    let lifetime = module.lit(1.); // literal value "10.0"
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_size = SetAttributeModifier::new(Attribute::SIZE, module.lit(0.15));

    // Every frame, add a gravity-like acceleration downward
    let accel = module.lit(Vec3::new(0., -3., 0.));
    let update_accel = AccelModifier::new(accel);

    let texture_handle = assets.load("textures/dust.png");

    // Create the effect asset
    let effect = EffectAsset::new(
        // Maximum number of particles alive at a time
        10,
        // Spawn at a rate of 5 particles per second
        Spawner::rate(5.0.into()),
        // Move the expression module into the asset
        module,
    )
    .with_name("MyEffect")
    .init(init_pos)
    .init(init_vel)
    .init(init_size)
    .init(init_lifetime)
    .update(update_accel)
    .render(ParticleTextureModifier {
        texture: texture_handle,
    })
    .render(BillboardModifier)
    // Render the particles with a color gradient over their
    // lifetime. This maps the gradient key 0 to the particle spawn
    // time, and the gradient key 1 to the particle death (10s).
    .render(ColorOverLifetimeModifier { gradient });

    // Insert into the asset system
    let effect_handle = effects.add(effect);

    commands.insert_resource(ParticleCache {
        dust: effect_handle,
    });
}

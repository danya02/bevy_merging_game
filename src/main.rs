pub mod game_box;
pub mod plugins;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use plugins::MainPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(MainPlugin)
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2d::default());
}

fn setup_physics(mut commands: Commands) {
    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert(Transform::from_xyz(0.0, 400.0, 0.0))
        .insert(Velocity::linear(Vec2::new(100.0, -100.0)));
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert(Transform::from_xyz(100.0, 400.0, 0.0))
        .insert(Velocity::linear(Vec2::new(100.0, 100.0)));
}

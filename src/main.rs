pub mod ball;
pub mod collisions;
pub mod cursor;
pub mod game_box;
pub mod plugins;

use ball::Ball;
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

#[derive(Component)]
pub struct MainCamera;

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn((Camera2d::default(), MainCamera));
}

fn setup_physics(mut commands: Commands) {}

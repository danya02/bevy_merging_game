pub mod ball;
pub mod collisions;
pub mod cursor;
pub mod game_box;
pub mod plugins;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use plugins::MainPlugin;
use uom::si::{
    self,
    f32::ReciprocalLength,
    reciprocal_length::{reciprocal_centimeter, reciprocal_meter, reciprocal_millimeter},
};

lazy_static::lazy_static! {
    pub static ref PIXELS_PER_LENGTH: ReciprocalLength = ReciprocalLength::new::<reciprocal_millimeter>(2.5);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_LENGTH.get::<si::reciprocal_length::reciprocal_meter>(),
        ))
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

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use uom::si::{
    self,
    f32::{Length, Mass},
    length::millimeter,
};

use crate::PIXELS_PER_LENGTH;

/// A component for one of the balls in the game.
#[derive(Component)]
pub struct Ball {
    pub phase: usize,
}

pub struct BallPhase {
    pub diameter: Length,
    pub mass: Mass,
    pub texture: &'static str,
}

lazy_static::lazy_static! {
    /// information about euro coins taken from https://www.ecb.europa.eu/euro/coins/common/html/index.en.html
    pub static ref BALL_PHASES: Vec<BallPhase> = vec![
        BallPhase {
            diameter: Length::new::<millimeter>(16.25),
            mass: Mass::new::<si::mass::gram>(2.3),
            texture: "euro/png/common_1cent_800.png",
        },
        BallPhase {
            diameter: Length::new::<millimeter>(18.75),
            mass: Mass::new::<si::mass::gram>(3.06),
            texture: "euro/png/common_2cent_800.png",
        },
        BallPhase {
            diameter: Length::new::<millimeter>(21.25),
            mass: Mass::new::<si::mass::gram>(3.92),
            texture: "euro/png/common_5cent_800.png",
        },
        BallPhase {
            diameter: Length::new::<millimeter>(19.75),
            mass: Mass::new::<si::mass::gram>(4.1),
            texture: "euro/png/10common.png",
        },
        BallPhase {
            diameter: Length::new::<millimeter>(22.25),
            mass: Mass::new::<si::mass::gram>(5.74),
            texture: "euro/png/common_20cent_800.png",
        },
        BallPhase {
            diameter: Length::new::<millimeter>(24.75),
            mass: Mass::new::<si::mass::gram>(7.8),
            texture: "euro/png/common_50cent_800.png",
        },
        BallPhase {
            diameter: Length::new::<millimeter>(23.25),
            mass: Mass::new::<si::mass::gram>(7.5),
            texture: "euro/png/common_1euro_800.png",
        },
        BallPhase {
            diameter: Length::new::<millimeter>(25.75),
            mass: Mass::new::<si::mass::gram>(8.5),
            texture: "euro/png/common_2euro_800.png",
        },
    ];
}

#[derive(Bundle)]
pub struct BallBundle {
    ball: Ball,
    rigid_body: RigidBody,
    collider: Collider,
    restitution: Restitution,
    transform: Transform,
    active_events: ActiveEvents,
    mass_props: ColliderMassProperties,
    sprite: Sprite,
}

impl BallBundle {
    pub fn new(pos: Vec2, phase: usize, asset_server: &Res<AssetServer>) -> Self {
        let radius_in_pixels = BALL_PHASES[phase].diameter * *PIXELS_PER_LENGTH;
        let radius_in_pixels = radius_in_pixels / 2.0;
        let radius_in_pixels = radius_in_pixels.get::<si::ratio::ratio>();
        let mut sprite = Sprite::from_image(asset_server.load(BALL_PHASES[phase].texture));
        sprite.custom_size = Some(Vec2::new(radius_in_pixels * 2.0, radius_in_pixels * 2.0));

        BallBundle {
            ball: Ball { phase },
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(radius_in_pixels),
            restitution: Restitution::coefficient(0.2),
            transform: Transform::from_translation(pos.extend(0.0)),
            active_events: ActiveEvents::COLLISION_EVENTS,
            mass_props: ColliderMassProperties::Mass(
                BALL_PHASES[phase].mass.get::<si::mass::kilogram>(),
            ),
            sprite,
        }
    }
}

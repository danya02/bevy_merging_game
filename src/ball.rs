use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// A component for one of the balls in the game.
#[derive(Component)]
pub struct Ball {
    pub phase: usize,
}

pub struct BallPhase {
    pub color: Color,
    pub radius: f32,
}

pub const BALL_PHASES: &[BallPhase] = &[
    BallPhase {
        color: Color::hsl(0.0, 1.0, 1.0),
        radius: 10.0,
    },
    BallPhase {
        color: Color::hsl(120.0, 1.0, 1.0),
        radius: 15.0,
    },
    BallPhase {
        color: Color::hsl(240.0, 1.0, 1.0),
        radius: 20.0,
    },
];

#[derive(Bundle)]
pub struct BallBundle {
    ball: Ball,
    rigid_body: RigidBody,
    collider: Collider,
    restitution: Restitution,
    transform: Transform,
    active_events: ActiveEvents,
}

impl BallBundle {
    pub fn new(pos: Vec2, phase: usize) -> Self {
        BallBundle {
            ball: Ball { phase },
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(BALL_PHASES[phase].radius),
            restitution: Restitution::coefficient(0.7),
            transform: Transform::from_translation(pos.extend(0.0)),
            active_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}

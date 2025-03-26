use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::ball::{BALL_PHASES, Ball, BallBundle};

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_collide);
    }
}

fn on_collide(
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    q_ball: Query<(&Transform, &Ball)>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(a, b, flags) = collision_event {
            if let Some((first_transform, first_ball)) = q_ball.get(*a).ok() {
                if let Some((second_transform, second_ball)) = q_ball.get(*b).ok() {
                    if first_ball.phase != second_ball.phase {
                        continue;
                    }

                    commands.entity(*a).despawn();
                    commands.entity(*b).despawn();

                    let avg_pos = (first_transform.translation.truncate()
                        + second_transform.translation.truncate())
                        / 2.0;

                    if first_ball.phase + 1 >= BALL_PHASES.len() {
                        continue;
                    }

                    commands.spawn(BallBundle::new(avg_pos, first_ball.phase + 1));
                }
            }
        }
    }
}

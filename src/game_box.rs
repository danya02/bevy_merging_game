//! Represents the box that the game objects are going to live in.

use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, Sensor};

#[derive(Component)]
pub struct GameBox {
    pub width: f32,
    pub height: f32,
}

const WALL_THICKNESS: f32 = 5.0;

impl GameBox {
    /// Get the X coordinate of the inner edge of the left wall.
    /// Assuming that the box is centered at the origin.
    pub fn left_wall_inner_x(&self) -> f32 {
        -self.width / 2.0 + WALL_THICKNESS
    }

    /// Get the X coordinate of the inner edge of the right wall.
    /// Assuming that the box is centered at the origin.
    pub fn right_wall_inner_x(&self) -> f32 {
        self.width / 2.0 - WALL_THICKNESS
    }

    /// Get the Y coordinate of the outer edge of the ceiling.
    /// Assuming that the box is centered at the origin.
    pub fn ceiling_outer_y(&self) -> f32 {
        self.height / 2.0 + WALL_THICKNESS
    }
}

pub struct GameBoxPlugin;

impl Plugin for GameBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_box);
        app.add_systems(Update, update_game_box);
    }
}

#[derive(Component)]
struct LeftWall;

#[derive(Component)]
struct RightWall;

#[derive(Component)]
struct Floor;

#[derive(Component)]
struct Ceiling;

fn spawn_game_box(mut commands: Commands) {
    let width = 200.0;
    let height = 300.0;
    let root = commands
        .spawn((Transform::from_xyz(0.0, 0.0, 0.0), GameBox {
            width,
            height,
        }))
        .id();

    commands
        .spawn((
            Transform::from_xyz(-width / 2.0, 0.0, 0.0),
            LeftWall,
            Collider::cuboid(WALL_THICKNESS * 2.0, height / 2.0),
        ))
        .set_parent(root);

    commands
        .spawn((
            Transform::from_xyz(width / 2.0, 0.0, 0.0),
            RightWall,
            Collider::cuboid(WALL_THICKNESS * 2.0, height / 2.0),
        ))
        .set_parent(root);

    commands
        .spawn((
            Transform::from_xyz(0.0, -height / 2.0, 0.0),
            Floor,
            Collider::cuboid(width / 2.0, WALL_THICKNESS * 2.0),
        ))
        .set_parent(root);

    commands
        .spawn((
            Transform::from_xyz(0.0, height / 2.0, 0.0),
            Ceiling,
            Collider::cuboid(width / 2.0, WALL_THICKNESS * 2.0),
            Sensor,
        ))
        .set_parent(root);
}

fn update_game_box(
    mut game_box: Query<
        (&mut Transform, &GameBox),
        (
            Without<Ceiling>,
            Without<LeftWall>,
            Without<RightWall>,
            Without<Floor>,
        ),
    >,
    mut left_wall: Query<
        (&mut Transform, &mut Collider),
        (
            Without<Ceiling>,
            With<LeftWall>,
            Without<RightWall>,
            Without<Floor>,
            Without<GameBox>,
        ),
    >,
    mut right_wall: Query<
        (&mut Transform, &mut Collider),
        (
            Without<Ceiling>,
            Without<LeftWall>,
            With<RightWall>,
            Without<Floor>,
            Without<GameBox>,
        ),
    >,
    mut floor: Query<
        (&mut Transform, &mut Collider),
        (
            Without<Ceiling>,
            Without<LeftWall>,
            Without<RightWall>,
            With<Floor>,
            Without<GameBox>,
        ),
    >,
    mut ceiling: Query<
        (&mut Transform, &mut Collider),
        (
            With<Ceiling>,
            Without<LeftWall>,
            Without<RightWall>,
            Without<Floor>,
            Without<GameBox>,
        ),
    >,
) {
    let (mut transform, game_box) = game_box.single_mut();
    let (mut left_wall_transform, mut left_wall_collider) = left_wall.single_mut();
    let (mut right_wall_transform, mut right_wall_collider) = right_wall.single_mut();
    let (mut floor_transform, mut floor_collider) = floor.single_mut();
    let (mut ceiling_transform, mut ceiling_collider) = ceiling.single_mut();

    transform.translation.x = 0.0;
    transform.translation.y = 0.0;
    left_wall_transform.translation.x = -game_box.width / 2.0;
    left_wall_transform.translation.y = 0.0;
    right_wall_transform.translation.x = game_box.width / 2.0;
    right_wall_transform.translation.y = 0.0;
    floor_transform.translation.x = 0.0;
    floor_transform.translation.y = -game_box.height / 2.0;
    ceiling_transform.translation.x = 0.0;
    ceiling_transform.translation.y = game_box.height / 2.0;

    left_wall_collider
        .as_cuboid_mut()
        .unwrap()
        .set_half_extents(Vec2::new(10.0, game_box.height / 2.0));
    right_wall_collider
        .as_cuboid_mut()
        .unwrap()
        .set_half_extents(Vec2::new(10.0, game_box.height / 2.0));
    floor_collider
        .as_cuboid_mut()
        .unwrap()
        .set_half_extents(Vec2::new(game_box.width / 2.0, 10.0));
    ceiling_collider
        .as_cuboid_mut()
        .unwrap()
        .set_half_extents(Vec2::new(game_box.width / 2.0, 10.0));
}

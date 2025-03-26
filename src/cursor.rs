use bevy::{input::mouse::MouseButtonInput, prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

use crate::{
    MainCamera,
    ball::{Ball, BallBundle},
    game_box::GameBox,
};
pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cursor);
        app.add_systems(Update, update_cursor);
        app.add_systems(Update, onclick_cursor);
    }
}

#[derive(Component)]
struct CursorMarker;

fn spawn_cursor(mut commands: Commands, asset_server: Res<AssetServer>) {
    // The cursor is a sprite that is hidden by default.
    commands
        .spawn(Sprite::from_image(asset_server.load("cursor.png")))
        .insert(Transform::from_scale(Vec3::splat(0.1)))
        .insert(CursorMarker)
        .insert(Visibility::Hidden);
}

fn update_cursor(
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_box: Query<(&Transform, &GameBox), Without<CursorMarker>>,
    mut cursor: Query<(&mut Transform, &mut Visibility), (With<CursorMarker>, Without<GameBox>)>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    let (box_position, box_state) = q_box.single();

    let (mut cursor_transform, mut cursor_visibility) = cursor.single_mut();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        *cursor_visibility = Visibility::Visible;
        cursor_transform.translation.x = world_position.x.clamp(
            box_state.left_wall_inner_x(),
            box_state.right_wall_inner_x(),
        );
        cursor_transform.translation.y = box_state.ceiling_outer_y() + 50.0;
    } else {
        *cursor_visibility = Visibility::Hidden;
    }
}

fn onclick_cursor(
    mut commands: Commands,
    q_cursor: Query<&Transform, With<CursorMarker>>,
    mut ev_read: EventReader<MouseButtonInput>,
    asset_server: Res<AssetServer>,
) {
    for ev in ev_read.read() {
        if ev.state.is_pressed() {
            if ev.button == MouseButton::Left {
                let cursor_transform = q_cursor.single();
                commands.spawn(BallBundle::new(
                    cursor_transform.translation.truncate(),
                    0,
                    &asset_server,
                ));
            }
        }
    }
}

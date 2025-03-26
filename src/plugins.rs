use crate::{collisions::CollisionsPlugin, cursor::CursorPlugin, game_box::GameBoxPlugin};

pub struct MainPlugin;

impl bevy::prelude::Plugin for MainPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(GameBoxPlugin);
        app.add_plugins(CursorPlugin);
        app.add_plugins(CollisionsPlugin);
    }
}

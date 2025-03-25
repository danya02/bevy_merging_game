use crate::game_box::GameBoxPlugin;

pub struct MainPlugin;

impl bevy::prelude::Plugin for MainPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(GameBoxPlugin);
    }
}

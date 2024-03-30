use bevy::prelude::*;

mod light;
mod map;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((light::LightPlugin, map::MapPlugin, player::CameraPlugin))
        .run();
}

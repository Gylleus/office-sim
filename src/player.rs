use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_camera);
    }
}

fn add_camera(mut commands: Commands) {
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(0., 2.0, 0.).looking_to(Vec3::X, Vec3::Y),
        ..default()
    };

    commands.spawn(camera);
}

use crate::map::generate::{map_pixel_dimensions, setup_generator};
use bevy::{
    prelude::*,
    window::{Window, WindowPlugin, WindowResolution},
};
use bevy_procedural_tilemaps::prelude::*;
use characters::CharactersPlugin;
use state::StatePlugin;

mod characters;
mod collision;
mod config;
mod map;
mod state;

fn main() {
    let map_size = map_pixel_dimensions();

    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "src/assets".into(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(map_size.x as u32, map_size.y as u32),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            ProcGenSimplePlugin::<Cartesian3D, Sprite>::default(),
            StatePlugin,
            collision::CollisionPlugin,
            CharactersPlugin,
        ))
        .add_systems(Startup, (setup_camera, setup_generator))
        // .add_systems(Update, close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn close_on_esc(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: MessageWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.write(AppExit::Success);
    }
}

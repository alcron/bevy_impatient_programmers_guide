use crate::camera::CameraPlugin;
use crate::map::generate::setup_generator;
use bevy::{
    prelude::*,
    window::{MonitorSelection, Window, WindowMode, WindowPlugin},
};
use bevy_procedural_tilemaps::prelude::*;
use characters::CharactersPlugin;
use state::StatePlugin;

mod camera;
mod characters;
mod collision;
mod combat;
mod config;
mod inventory;
mod map;
mod particles;
mod state;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "src/assets".into(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            ProcGenSimplePlugin::<Cartesian3D, Sprite>::default(),
            StatePlugin,
            CameraPlugin,
            CharactersPlugin,
            inventory::InventoryPlugin,
            collision::CollisionPlugin,
            combat::CombatPlugin,
            particles::ParticlesPlugin,
        ))
        .add_systems(Startup, setup_generator)
        .add_systems(Update, close_on_f1)
        .run();
}

fn close_on_f1(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: MessageWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        app_exit_events.write(AppExit::Success);
    }
}

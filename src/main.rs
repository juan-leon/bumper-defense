use bevy::app::App;
use bevy::ecs::schedule::SystemSet;
use bevy::render::view::Msaa;
use bevy::DefaultPlugins;
use heron::PhysicsPlugin;

mod cli;
mod enemies;
mod game;
mod menu;
mod player;
mod util;
mod world;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

fn main() {
    let matches = cli::get_app().get_matches();
    let mut app = App::new();

    app.insert_resource(world::create_window(matches.value_of_t_or_exit("width")))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins);
    if matches.is_present("verbose") {
        app.add_plugin(LogDiagnosticsPlugin::default())
            .add_plugin(FrameTimeDiagnosticsPlugin::default());
    }
    app.add_plugin(PhysicsPlugin::default())
        .add_state(game::AppState::Menu)
        .add_plugin(world::WorldPlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(enemies::EnemyPlugin)
        .add_system_set(
            SystemSet::on_enter(game::AppState::InGame).with_system(world::create_camera),
        )
        .add_system_set(
            SystemSet::on_update(game::AppState::InGame)
                .with_system(bevy::input::system::exit_on_esc_system),
        )
        .run();
}

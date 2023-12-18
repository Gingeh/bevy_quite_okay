use bevy::prelude::*;
use bevy_quite_okay::{qoa, qoi};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(qoi::QoiPlugin)
        .add_plugins(qoa::QoaPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        texture: asset_server.load("bevy.qoi"),
        ..Default::default()
    });

    commands.spawn(AudioSourceBundle {
        source: asset_server.load::<qoa::QoaSource>("Windless Slopes.qoa"),
        ..Default::default()
    });
}

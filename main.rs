use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::app::AppExit;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_system(start_game.system())
        .run();
}

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Load the main menu UI
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..Default::default()
        },
        material: materials.add(Color::NONE.into()),
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(50.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.3, 0.3, 0.9).into()),
            ..Default::default()
        }).with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    value: "Start Game".to_string(),
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    style: TextStyle {
                        font_size: 30.0,
                        color: Color::WHITE,
                        ..Default::default()
                    },
                },
                ..Default::default()
            });
        });
    });
}

fn start_game(mut app_exit_events: EventWriter<AppExit>) {
    // Handle button click to start the game
    // Add your game start logic here
    app_exit_events.send(AppExit);
}

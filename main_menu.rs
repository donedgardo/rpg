use bevy::prelude::*;
use bevy::ui::style::{Style, Size, Val};
use bevy::ui::align::{AlignItems, JustifyContent};
use bevy::ui::node::NodeBundle;
use bevy::ui::button::ButtonBundle;
use bevy::ui::text::TextBundle;
use bevy::render::color::Color;
use bevy::asset::AssetServer;
use bevy::ecs::system::ResMut;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
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

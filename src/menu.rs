use bevy::prelude::*;
use crate::app_state::AppState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_menu)
            .add_system(button_system);
    }
}

fn create_button(materials: Handle<Image>) -> ButtonBundle {
    ButtonBundle {
        image: UiImage{
            texture: materials,
            flip_x: false,
            flip_y: false,
        },
        style: Style {
            size: Size::new(Val::Px(279.), Val::Px(76.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        ..Default::default()
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load the button materials
    let button_materials = asset_server.load("button_materials.png");
    let font = asset_server.load("Roboto/Roboto-Regular.ttf");

    // Create a node to contain the buttons
    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            gap: Size::new(Val::Px(32.), Val::Px(32.)),
            ..Default::default()
        },
        background_color: BackgroundColor::from(Color::rgb(0.26, 0.26, 0.32)),
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn((create_button(button_materials.clone()), LocalPlayButton))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section("New Game", TextStyle {
                    font: font.clone(),
                    font_size: 32.,
                    color: Default::default(),
                }));
        });
        parent.spawn((create_button(button_materials), OnlinePlayButton))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section("Online Game", TextStyle {
                    font,
                    font_size: 32.,
                    color: Default::default(),
                }));
            });
    });
}
fn button_system(
    mut commands: Commands,
    button_materials: Res<Assets<ColorMaterial>>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, Entity),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<State<AppState>>,
    world: Res<World>,
) {
    // Handle button interactions
    for (interaction, mut material, entity) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                if world.get::<OnlinePlayButton>(entity).is_some() {
                    app_state.set(AppState::Online).unwrap();
                }
            }
            _ => {}
        }
    }
}

#[derive(Component)]
struct LocalPlayButton;

#[derive(Component)]
struct OnlinePlayButton;


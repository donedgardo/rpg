use bevy::prelude::*;
use crate::app_state::AppState;
use crate::cleanup_ui::cleanup_system;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup_menu.in_schedule(OnEnter(AppState::Menu)))
            .add_system(button_system.in_set(OnUpdate(AppState::Menu)))
            .add_system(cleanup_system::<MenuButtons>.in_schedule(OnExit(AppState::Menu)));
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
        parent.spawn((create_button(button_materials.clone()), MenuButtons::LocalPlay))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section("New Game", TextStyle {
                    font: font.clone(),
                    font_size: 32.,
                    color: Default::default(),
                }));
        });
        parent.spawn((create_button(button_materials), MenuButtons::OnlinePlay))
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
    mut interaction_query: Query<
        (&Interaction, &MenuButtons),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<NextState<AppState>>,
) {
    // Handle button interactions
    for (interaction, button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                match button {
                    MenuButtons::OnlinePlay => {
                        app_state.set(AppState::WaitingForPlayers);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

#[derive(Component)]
enum MenuButtons {
    LocalPlay,
    OnlinePlay
}

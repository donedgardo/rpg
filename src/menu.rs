use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_menu)
            .add_system(button_system);
    }
}

fn create_button(materials: Handle<ColorMaterial>) -> ButtonBundle {
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

    // Create a node to contain the buttons
    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        background_color: BackgroundColor::from(Color::rgb(0.26, 0.26, 0.32)),
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn((create_button(button_materials.clone()), LocalGameButton));
        parent.spawn((create_button(button_materials.clone()), OneVOneGameButton));
        parent.spawn((create_button(button_materials), TwoVTwoGameButton));
    });
}
fn button_system(
    mut commands: Commands,
    button_materials: Res<Assets<ColorMaterial>>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Changed<Interaction>, With<Button>),
    >,
) {
    // Handle button interactions
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                // Handle button click
            }
            _ => {}
        }
    }
}

#[derive(Component)]
struct LocalGameButton;

#[derive(Component)]
struct OneVOneGameButton;

#[derive(Component)]
struct TwoVTwoGameButton;

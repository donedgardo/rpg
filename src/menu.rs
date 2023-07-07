use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_menu)
            .add_system(button_system);
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load the button materials
    let button_materials = asset_server.load("assets/button_materials.png");

    // Create a node to contain the buttons
    let node = commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        ..Default::default()
    }).current_entity().unwrap();

    // Create the buttons
    commands.spawn((ButtonBundle {
        image: UiImage{
            texture: button_materials.clone(),
            flip_x: false,
            flip_y: false,
        },
        ..Default::default()
    }, LocalGameButton)).push_children(&[node]);
    commands.spawn((ButtonBundle {
        image: UiImage{
            texture: button_materials.clone(),
            flip_x: false,
            flip_y: false,
        },
        ..Default::default()
    }, OneVOneGameButton)).push_children(&[node]);
    commands.spawn((ButtonBundle {
        image: UiImage{
            texture: button_materials,
            flip_x: false,
            flip_y: false,
        },
        ..Default::default()
    }, TwoVTwoGameButton)).push_children(&[node]);
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

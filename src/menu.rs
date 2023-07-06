use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(setup_menu.system())
            .add_system(button_system.system());
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load the button materials
    let button_materials = asset_server.load("assets/button_materials.png");
    // Create the buttons
    commands.spawn_bundle(ButtonBundle {
        background_color: BackgroundColor::from(Color::AZURE),
        ..Default::default()
    }).with(LocalGameButton);
    commands.spawn_bundle(ButtonBundle {
        background_color: BackgroundColor::from(Color::AZURE),
        ..Default::default()
    }).with(OneVOneGameButton);
    commands.spawn_bundle(ButtonBundle {
        background_color: BackgroundColor::from(Color::AZURE),
        ..Default::default()
    }).with(TwoVTwoGameButton);
}

fn button_system(
    mut commands: Commands,
    button_materials: Res<Assets<ColorMaterial>>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Mutated<Interaction>, With<Button>),
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

struct LocalGameButton;
struct OneVOneGameButton;
struct TwoVTwoGameButton;
